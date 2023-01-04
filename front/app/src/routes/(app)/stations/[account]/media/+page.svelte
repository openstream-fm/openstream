<script lang="ts">
  export let data: import("./$types").PageData;

  import { beforeNavigate, invalidate } from "$app/navigation";
	import Page from "$lib/components/Page.svelte";
	import { ripple } from "$lib/ripple";
	import { action, ClientError, _delete, _post, _request } from "$share/net.client";
  import { mdiPlay, mdiPause, mdiAlertDecagram, mdiCheck, mdiTimerPauseOutline, mdiCircleEditOutline, mdiTrashCanOutline, mdiAutorenew, mdiCheckboxIntermediate, mdiCheckboxMarked, mdiCheckboxBlankOutline, mdiCheckboxMarkedOutline } from "@mdi/js";
	import Icon from "$share/Icon.svelte";
	import { onMount } from "svelte";
  import CircularProgress from "$share/CircularProgress.svelte";
	import { tooltip } from "$lib/actions";
	import { fade, slide } from "svelte/transition";
	import { _message, _progress } from "$share/notify";
  import Dialog from "$share/Dialog.svelte";
  import { sleep } from "$share/util";

  $: account_id = data.account._id;

  $: playlist_duration = getPlaylistDuration(data.files.items);
  const getPlaylistDuration = (files: FileDocument[]): number => {
    let d = 0;
    for(const item of files) {
      d += item.duration_ms;
    }
    return d;
  }

  const clear = async () => {
    
    for(const item of uploading) {
      if(item.state !== "uploading" && item.state !== "waiting") {
        set.delete(item.file);
      }
    }

    set = set;

    uploading = uploading.filter(item => {
      return item.state === "uploading" || item.state === "waiting";
    })

    document.scrollingElement?.scrollTo({ top: 0, behavior: "smooth" });
  }
  
  const upload_size = (size: number) => {

    let acc = size;
    for(const unit of ["bytes", "KB", "MB"]) {
      if(acc < 1000) {
        return `${Math.round(acc)} ${unit}`;
      }
      acc = acc / 1000;
    }

    return `${Math.round(acc)} GB`;
  }

  type FileDocument = typeof data.files.items[number];
  type PlayingItem = { file: FileDocument, audio: HTMLAudioElement };
  let playing_item: PlayingItem | null = null;

  const toggle_play = (file: FileDocument) => {
    if(playing_item == null) {
      const audio = new Audio(`/api/accounts/${account_id}/files/${file._id}/stream`);
      const onend = () => {
        if(playing_item?.file._id === file._id) {
          playing_item = null;
        }
      }
      audio.onabort = onend;
      audio.onerror = onend;
      audio.onpause = onend;
      audio.onended = onend;
      audio.play();
      playing_item = { file, audio };
    } else if (playing_item.file._id === file._id) {
      stop_playback();
    } else {
      stop_playback();
      toggle_play(file);
    }
  }

  const stop_playback = () => {
    if(playing_item == null) return;
    const { audio } = playing_item;
    playing_item = null;
    audio.pause();
  }

  const S = 1000;
  const M = S * 60;
  const H = M * 60;
  const track_duration = (ms: number) => {
    const h = Math.floor(ms / H);
    const m = Math.floor((ms % H) / M);
    const s = Math.round((ms % M) / S)
    if(h) {
      return `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`
    } else {
      return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`
    }
  }

  const total_duration = (ms: number) => {
    const h = Math.floor(ms / H);
    const m = Math.floor((ms % H) / M);
    const s = Math.round((ms % M) / S)
    
    if(h) {
      return `${h}h ${m}m ${s}s`;
    }

    if(m) {
      return `${m}m ${s}s`;
    }
    
    return `${s}s`;
  }

  type State = { state: "uploading", error?: undefined } | { state: "done", error?: undefined } | { state: "waiting", error?: undefined } | { state: "error", error: ClientError };
  type Item = { file: File, id: number } & State;

  let set = new Set<File>();
  let uploading: Item[] = [];
  let files: FileList | undefined;
 
  // uploading.push({
  //   id: Math.random(),
  //   state: "done",
  //   file: {
  //     name: "audio.mp3",
  //     size: Math.floor(Math.random() * 10000000),
  //   } as any
  // });

  // uploading.push({
  //   id: Math.random(),
  //   state: "error",
  //   file: {
  //     name: "as ndalksdalksa da sd.mp3",
  //     size: Math.floor(Math.random() * 10000000),
  //   } as any,
  //   error: { message: "error message", status: 500, code: "UPLOAD_INTENRAL_IO" } as any
  // });

  // uploading.push({
  //   id: Math.random(),
  //   state: "uploading",
  //   file: {
  //     name: "as ndalksdalksa da sd.mp3",
  //     size: Math.floor(Math.random() * 10000000),
  //   } as any,
  // });

  // uploading.push({
  //   id: Math.random(),
  //   state: "waiting",
  //   file: {
  //     name: "as ndalksdalksa da sd.mp3",
  //     size: Math.floor(Math.random() * 10000000),
  //   } as any,
  // });

  beforeNavigate(({ willUnload, from, to, cancel }) => {
    if(from && to && from.route === to.route) return;
    if(uploading.some(item => item.state === "waiting" || item.state === "uploading")) {
      if(willUnload){
        cancel();
      } else {
        const ok = confirm("Leaving this page will cancel pending uploads. Do you want to leave anyway?");
        if(!ok) cancel();
      }
    }
  })


  let controller: AbortController | null = null;
  let unmounted = false;

  onMount(() => { 
    if(window.AbortController) controller = new AbortController();
    return () => {
      if(playing_item != null) {
        const audio = playing_item.audio;
        playing_item = null;
        audio.pause();
      }

      unmounted = true;
      if(controller != null) controller.abort();
    }
  })

  const uploading_current_size = (): number => {
    let n = 0;
    for(const item of uploading) {
      if(item.state === "uploading") n += 1;
    }
    return n;
  }

  const MAX_CONCURRENT_UPLOADS = 1;

  const next = async () => {
    if(unmounted) return;
    if(uploading_current_size() >= MAX_CONCURRENT_UPLOADS) return;
    const item = uploading.find(item => item.state === "waiting");
    if(item == null) return;
    item.state = "uploading";
    uploading = uploading;
    
    next();

    try {
      const _new_file: import("$server/defs/api/accounts/[account]/files/POST/Output").Output = await _request(`/api/accounts/${account_id}/files?filename=${encodeURIComponent(item.file.name)}`, {
        method: "POST",
        headers: {
          "content-length": String(item.file.length),
          "content-type": item.file.type,
        },
        signal: controller?.signal,
        body: item.file,
      })

      controller = null;
      item.state = "done";
      uploading = uploading;

      invalidate("account:limits");
      invalidate("account:files");

    } catch(e: any) {
      set.delete(item.file);
      item.state = "error";
      item.error = e;
      uploading = uploading;
    }

    next();
  }

  const retry = (item: Item) => {
    if(item.state !== "error") return;
    // @ts-ignore
    delete item.error;
    // @ts-ignore
    item.state = "waiting";
    uploading = uploading;
    next();
  }

  $: on_files(files);
  const on_files = (...args: any[]) => {
    if(!files) return;
    const _files = files;
    files = undefined;
    for(let i = 0; i < _files.length; i++) {
      const file = _files.item(i)!;
      if(set.has(file)) continue;
      const item: Item = { id: Math.random(), file, state: "waiting" };
      uploading = [ ...uploading, item ];
      next();
    }
  }

  const del = action(async (file_id: string) => {
    await _delete(`/api/accounts/${account_id}/files/${file_id}`);
    unselect(file_id);
    invalidate("account:limits");
    invalidate("account:files");
    _message("Track deleted");
    if(playing_item?.file._id === file_id) stop_playback(); 
  })

  const del_selected = async () => {
    if(audio_item_to_delete == null) return;
    if(await del(audio_item_to_delete._id)) {
      audio_item_to_delete = null;
    } 
  }

  const set_del_selection_open = () => {
    const ids = $selected_ids;
    if(ids.length === 0) return;
    if(ids.length === 1) {
      const item = data.files.items.find(item => item._id = ids[0]);
      if(item != null) audio_item_to_delete = item;
      else delete_selection_open = true;
    } else {
      delete_selection_open = true;
    }
  }

  const del_selection_all = action(async () => {
    const ids = $selected_ids;
    if(ids.length === 0) return;
    delete_selection_open = false;
    //const text = (i: number) => `Deleting ${ids.length} tracks... ${i} tracks deleted`;
    //const message = writable(text(0));
    const { resolve, reject } = _progress(`Deleting ${ids.length} tracks...`);
    try {
      let i = 0;
      for(const id of ids) {
        //await sleep(100);
        //message.set(text(i));
        await _delete(`/api/accounts/${account_id}/files/${id}`);
        if(playing_item && ids.includes(playing_item.file._id)) stop_playback();
        data.files.items = data.files.items.filter(item => item._id !== id);
        data.files.total = data.files.items.length;
        i++;
      }
    } catch(e: any) {
      await invalidate("account:limits");
      await invalidate("account:files");
      $selected_ids = $selected_ids.filter(id => data.files.items.some(item => item._id === id));
      reject(String(e?.message));
      throw e;
    }

    invalidate("account:limits");
    invalidate("account:files");
    resolve(`${ids.length} tracks deleted`);
    $selected_ids = [];
  })

  import { expoOut } from "svelte/easing";
	import { writable } from "svelte/store";
  const file_item_out = (node: HTMLElement, { duration = 250 } = {}) => {
    return {
      css: (t: number, u: number) => {
        return `opacity: ${t}; transform: translateY(-${20 * u}px)`;
      },
      easing: expoOut,
      duration,
    }
  }

  let audio_item_to_delete: FileDocument | null = null;
  let delete_selection_open: boolean = false;

  const selected_ids = writable<string[]>([]);

  const select = (id: string) => {
    if($selected_ids.includes(id)) return;
    $selected_ids = [...$selected_ids, id];
  } 
  
  const unselect = (id: string) => {
    $selected_ids = $selected_ids.filter(item => item !== id);
  }

  const toggle_select = (id: string) => {
    if($selected_ids.includes(id)) unselect(id);
    else select(id);
  }

  const toggle_selection_all = () => {
    if($selected_ids.length === data.files.items.length) {
      $selected_ids = [];
    } else {
      $selected_ids = data.files.items.map(item => item._id);
    }
  }
</script>

<style>
  .browse-btn {
    padding: 0.75rem 1rem;
    color: var(--blue);
    background-color: transparent;
    border: 2px var(--blue) solid;
    border-radius: 100px;
    appearance: none;
    cursor: pointer;
    /* box-shadow: 0 4px 20px 0 rgb(0 0 0 / 16%); */
    font-weight: 500;
    background: #fff;
  }

  .empty-message {
    font-size: 1.25rem;
    margin: 2rem 0;
  }

  .file-input {
    display: none;
  }

  .upload-empty-out {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .playlist-box, .upload-box {
    background: #fff;
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 5%);
    border-radius: 0.5rem;
    margin-top: 1.5rem;
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .playlist-box-title, .upload-box-title {
    font-weight: 600;
    font-size: 1.5rem;
  }

  .playlist-box-title {
    padding: 1rem;
  }

  .count {
    color: #666;
    font-size: 0.6em;
    vertical-align: middle;
    margin-inline-start: 0.75rem;
  }

  .playlist-scroll {
    overflow-x: auto;
    width: 100%;
  }

  .playlist-scroll-inner {
    display: flex;
  }

  table {
    flex: 1;
    border: 1px solid #f3f3f3;
    border-collapse: collapse;
  }

  tbody > tr:nth-child(odd) {
    background: #f6f6f6;
  }

  .file-data-item {
    padding: 1rem 1rem;
    font-weight: 400;
    font-size: 1rem;
    color: #555;
  }

  .file-data-text {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-preview-cell {
    padding: 0 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .file-preview-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.75rem;
    color: #444;
    padding: 0.5rem;
    cursor: pointer;
    appearance: none;
    background: transparent;
    border: 0;
    margin: auto;
  }

  th {
    text-align: left;
    font-weight: 500;
    font-size: 1rem;
  }

  th:not(.btn-cell) > div {
    padding: 1rem;
  }

  .file-item {
    border-top: transparent 1px solid;
  }

  .file-item.selected {
    background: var(--selection-blue);
    border-top: rgba(0,0,0,0.1) 1px solid;
  } 

  .upload-top {
    padding: 1rem 1rem;
    flex: 1;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }

  .upload-item {
    display: flex;
    flex-direction: row;
    padding: 0.5rem 1.5rem;
    align-items: center;
  }

  .upload-item-size {
    background: #eee;
    border-radius: 50px;
    padding: 0.5rem;
    margin-inline-start: 2rem;
    white-space: nowrap;
    flex: none;
  }

  .upload-icon {
    margin-inline-start: 1rem;
    font-size: 1.25rem;
    flex: none;
  }

  .upload-icon > div {
    display: flex;
    padding: 0.75rem;
    border-radius: 100px;
    flex: none;
  }

  .upload-icon-done {
    color: var(--green);
  }

  .upload-icon-error {
    background: var(--red);
    color: #fff;
  }

  .upload-icon-uploading {
    color: var(--blue);
  }

  .upload-icon-waiting {
    color: var(--orange);
  }

  .upload-error-retry {
    display: flex;
    padding: 0.4rem;
    border-radius: 100px;
    flex: none;
    appearance: none;
    color: var(--blue);
    border: 2px var(--blue) solid;
    cursor: pointer;
    margin-inline-start: 1rem;
    font-size: 1.1rem;
    background: transparent;
  }

  .uploading-clear-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .uploading-clear {
    padding: 0.75rem;
    border: #eee 3px solid;
    border-radius: 50px;
    user-select: none;
    cursor: pointer;
    appearance: none;
    margin: 2rem auto;
    background: transparent;
  }
  
  .btn-cell {
    width: 0;
  }

  .file-btn {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    margin: 0 0.5rem;
    border-radius: 50%;
    font-size: 1.1rem;
    width: 2rem;
    height: 2rem;
    user-select: none;
    cursor: pointer;
    appearance: none;
    background: transparent;
    border: 0;
    padding: 0;
  }
  
  .file-btn-del {
    color: var(--red);
    border: var(--red) 2px solid;
    background: #fff;
    margin-inline-end: 1.5rem;
  }

  .delete-dialog-btns {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-end;
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .delete-btn, .delete-dialog-btn-delete, .delete-dialog-btn-cancel {
    padding: 0.5rem 0.75rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 0.25rem;
    transition: background-color 150ms ease;
  }

  .delete-btn:hover, .delete-dialog-btn-delete:hover, .delete-dialog-btn-cancel:hover {
    background: rgba(0,0,0,0.05);
  }

  .delete-btn, .delete-dialog-btn-delete {
    font-weight: 500;
    color: var(--red);
    border: 2px solid var(--red);
    box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
  }

  .delete-btn {
    border-radius: 100px;
  }

  .delete-dialog-btn-cancel {
    color: #555;
  }

  .delete-btn-icon, .delete-dialog-btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-inline: -0.25rem 0.5rem;
    font-size: 1.2rem;
  }

  .selection-actions {
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    flex-wrap: wrap-reverse;
    gap: 1rem;
  }

  .btn-cell {
    vertical-align: middle;
  }

  .select-all-btn, .select-btn {
    width: 2.5rem;
    height: 2.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #444;
    font-size: 1.5rem;
    transition: background-color 150ms ease, color 200ms ease;
    border-radius: 50%;
  }

  .select-all-btn:hover, .select-btn:hover {
    background: rgba(0,0,0,0.05);    
  }

  .selection-count {
    background: var(--selection-blue);
    color: rgba(0,0,0,0.5);
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 100px;
    padding: 0.6rem 1rem;
    font-size: 0.9rem;
  }

  .selection-count-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    margin-inline-start: 0.5rem;
  }

  .cell-space-start {
    width: 1rem;
  }
</style>

<svelte:head>
  <title>Media</title>
</svelte:head>

<Page>
  <h1>Media</h1>

  {#if data.files.total === 0 && uploading.length === 0}
    <div class="empty">
      <div class="empty-message">
        Your media collection is empty
      </div>
      
      <div class="upload-empty-out">
        <label for="media-upload-files" class="browse-btn ripple-container" use:ripple>
          Upload files
        </label>
      </div>
    </div>
  {:else}
  
    <div class="upload-box">
      <div class="upload-top">
        <div class="upload-box-title">
          Upload
        </div>

        <label for="media-upload-files" class="browse-btn ripple-container" use:ripple>
          Browse
        </label>
      </div>

      {#if uploading.length}
        <div class="uploading">
          {#each uploading as item (item.id)}
            <div class="upload-item" data-upload-state={item.state}>
              <div class="upload-item-name">{item.file.name}</div>
              <div class="upload-item-size">{upload_size(item.file.size)}</div>
              <div class="upload-icon">
                {#if item.state === "done"}
                  <div class="upload-icon-done" use:tooltip={"Uploaded successfully"} in:fade|local={{ duration: 200 }}>
                    <Icon d={mdiCheck} />
                  </div>
                {:else if item.state === "waiting"}
                  <div class="upload-icon-waiting" use:tooltip={"Waiting..."}  in:fade|local={{ duration: 200 }}>
                    <Icon d={mdiTimerPauseOutline} />
                  </div>
                {:else if item.state === "uploading"}
                  <div class="upload-icon-uploading" use:tooltip={"In progress..."}  in:fade|local={{ duration: 200 }}>
                    <CircularProgress />
                  </div>
                {:else if item.state === "error"}
                  <div
                    class="upload-icon-error"
                    use:tooltip={item.error.code ? `${item.error.message ?? "error"} (${item.error.code})` : item.error.message ?? "error"}
                    in:fade|local={{ duration: 200 }}
                  >
                    <Icon d={mdiAlertDecagram} />
                  </div>
                {/if}
              </div>
              {#if item.state === "error"}
                <button class="upload-error-retry ripple-container" use:tooltip={"Retry"} use:ripple on:click={() => retry(item)} >
                 <Icon d={mdiAutorenew} />
                 </button>
               {/if}
            </div>
          {/each}
        </div>
        <div class="uploading-clear-out">
          <button class="uploading-clear ripple-container" use:ripple on:click={clear}>
            Clear done items
          </button>
        </div>
      {/if}
    </div>

    <div class="playlist-box">
      <div class="playlist-top">
        <div class="playlist-box-title">
          Tracks
          <span class="count">{data.files.total} {data.files.total === 1 ? "track" : "tracks"} - {total_duration(playlist_duration)}</span>
        </div>
      </div>

      {#if $selected_ids.length}
        <div class="selection-actions" transition:slide|local={{ duration: 200 }}>
          <button class="delete-btn selected-action selected-action-remove" use:ripple on:click={set_del_selection_open}>
            <div class="delete-btn-icon selected-action-icon">
              <Icon d={mdiTrashCanOutline} />
            </div>
            Delete selected
          </button>

          <div class="selection-count">
            <div class="selection-count-text">
              {#if $selected_ids.length === 1}
                1 track selected
              {:else}
                {$selected_ids.length} tracks selected
              {/if}
            </div>
            <div class="selection-count-icon">
              <Icon d={mdiCheck} />
            </div>
          </div>
        </div>
      {/if}

      <div class="playlist-scroll">
        <div class="playlist-scroll-inner">
         <table class="playlist-table">
            <thead>
              <tr>
                <th class="cell-space-start"></th>
                <th class="btn-cell">
                  <div class="select-all-cell">
                    <button class="select-all-btn" class:check={$selected_ids.length !== 0 && $selected_ids.length === data.files.items.length} use:ripple on:click={toggle_selection_all}>
                      {#if $selected_ids.length === 0}
                        <Icon d={mdiCheckboxBlankOutline} />
                      {:else if $selected_ids.length === data.files.items.length}
                        <Icon d={mdiCheckboxMarkedOutline} />
                      {:else}
                        <Icon d={mdiCheckboxIntermediate} />
                      {/if}
                    </button>
                  </div>
                </th>
                <th class="btn-cell"></th>
                <th>
                  <div>
                    Title
                  </div>
                </th>
                <th>
                  <div>
                    Artist
                  </div>
                </th>
                <th>
                  <div>
                    Album
                  </div>
                </th>
                <th>
                  <div>
                    Duration
                  </div>
                </th>
                <th class="btn-cell"></th>
                <th class="btn-cell"></th>
              </tr>
            </thead>
            <tbody>  
              {#each data.files.items as file (file._id)}
                
                {@const selected = $selected_ids.includes(file._id)}

                <tr class="file-item" class:selected in:fade|local={{ duration: 250 }} out:file_item_out|local>
                  <td class="cell-space-start"></td>
                  <td class="btn-cell">
                    <div class="select-cell">
                      <button class="select-btn" class:check={selected} use:ripple on:click={() => toggle_select(file._id)}>
                        {#if $selected_ids.includes(file._id)}
                          <Icon d={mdiCheckboxMarkedOutline} />
                        {:else}
                          <Icon d={mdiCheckboxBlankOutline} />
                        {/if}
                      </button>
                    </div>
                  </td>
                  <td class="btn-cell">
                    <div class="file-preview-cell">
                      <button class="file-preview-btn" on:click={() => toggle_play(file)}>
                        <Icon d={playing_item?.file._id === file._id ? mdiPause : mdiPlay} />
                      </button>
                    </div>
                  </td>
                  <td>
                    <div class="file-data-item">
                      <div class="file-data-text">{file.metadata.title || file.filename}</div>
                    </div>
                  </td>
                  <td>
                    <div class="file-data-item">
                      <div class="file-data-text">{file.metadata.artist || "-"}</div>
                    </div>
                  </td>
                  <td>
                    <div class="file-data-item">
                      <div class="file-data-text">{file.metadata.album || "-"}</div>
                    </div>
                  </td>
                  <td>
                    <div class="file-data-item">
                      {track_duration(file.duration_ms)}
                    </div>
                  </td>
                  <td class="btn-cell">
                    <button class="file-btn file-btn-edit ripple-container" use:ripple use:tooltip={"Edit"}>
                      <Icon d={mdiCircleEditOutline} />
                    </button>
                  </td>
                  <td class="btn-cell">
                    <button class="file-btn file-btn-del ripple-container" use:ripple use:tooltip={"Delete"} on:click={() => audio_item_to_delete = file}>
                      <Icon d={mdiTrashCanOutline} />
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
   {/if}

   <input id="media-upload-files" class="file-input" type="file" multiple accept="audio/*" bind:files={files} />
</Page>

{#if audio_item_to_delete != null}
  <Dialog
    title="Delete track {audio_item_to_delete.metadata.title || audio_item_to_delete.filename}"
    width="400px"
    onClose={() => audio_item_to_delete = null}
  >
    <div class="delete-dialog">
      <div class="delete-dialog-text">
        This action is permanent.
      </div>
      <div class="delete-dialog-btns">

        <button class="delete-dialog-btn-cancel ripple-container" use:ripple on:click={() => audio_item_to_delete = null}>
          Cancel
        </button>
        
        <button class="delete-dialog-btn-delete ripple-container" use:ripple on:click={del_selected}>
          <div class="delete-dialog-btn-icon">
            <Icon d={mdiTrashCanOutline} />
          </div>
          Delete
        </button>
      </div>
    </div>
  </Dialog>
{:else if $selected_ids.length && delete_selection_open}
  <Dialog
    title="Delete {$selected_ids.length} tracks"
    width="400px"
    onClose={() => delete_selection_open = false}
  >
    <div class="delete-dialog">
      <div class="delete-dialog-text">
        This action is permanent.
      </div>
      <div class="delete-dialog-btns">

        <button class="delete-dialog-btn-cancel ripple-container" use:ripple on:click={() => delete_selection_open = false}>
          Cancel
        </button>
        
        <button class="delete-dialog-btn-delete ripple-container" use:ripple on:click={del_selection_all}>
          <div class="delete-dialog-btn-icon">
            <Icon d={mdiTrashCanOutline} />
          </div>
          Delete
        </button>
      </div>
    </div>
  </Dialog>
{/if}
