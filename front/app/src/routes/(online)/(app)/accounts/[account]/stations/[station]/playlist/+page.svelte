<script lang="ts">
  export let data: import("./$types").PageData;

  import { beforeNavigate, invalidate } from "$app/navigation";
	import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
	import { action, ClientError, _delete, _get, _post, _put, _request } from "$share/net.client";
  import { mdiPlay, mdiPause, mdiAlertDecagram, mdiCheck, mdiTimerPauseOutline, mdiCircleEditOutline, mdiTrashCanOutline, mdiAutorenew, mdiCheckboxIntermediate, mdiCheckboxBlankOutline, mdiContentSaveOutline, mdiDrag, mdiMusic, mdiShuffleVariant, mdiShuffleDisabled, mdiRestart } from "@mdi/js";
	import Icon from "$share/Icon.svelte";
	import { onMount } from "svelte";
  import CircularProgress from "$share/CircularProgress.svelte";
	import { add } from "$share/actions";
	import { tooltip } from "$share/tooltip";
  import { fade, fly, scale, slide } from "svelte/transition";
	import { _message, _progress } from "$share/notify";
  import Dialog from "$share/Dialog.svelte";
  import { close, player_playing_audio_file_id, player_audio_state, resume, pause, play_track, player_state, play_station } from "$lib/components/Player/player";
  import { prevent_unload } from "$share/prevent-unload";

  let dragging_i: number | null = null;
  let drag_target_i: number | null = null;
  let dragging_tag_x = 0;
  let dragging_tag_y = 0;
  
  let dropping = false;

  let pointer_x = 0;
  let pointer_y = 0;

  let dragging_height = "100%";

  let drag_tag_target_element: HTMLElement | null;
  let drag_tag_source_element: HTMLElement | null;

  $: dragging_tag_x = pointer_x; 
  $: dragging_tag_y = pointer_y - (3 * 16 / 2) // 3rem / 2;

  let drag_autoscroll_timer: any;

  $: dragging_item = dragging_i == null ? null : data.files.items[dragging_i] ?? null;
  
  const drag_autoscroll = () => {
    if(pointer_y < 120) {
      document.scrollingElement!.scrollTop -= 3;
    } else if(pointer_y > window.innerHeight - 70) {
      document.scrollingElement!.scrollTop += 3;
    }
  }
  
  const nth_even = (dragging_i: number | null, drag_target_i: number | null, i: number): boolean => {
    if(dragging_i == null || drag_target_i == null || dragging_i === drag_target_i) return i % 2 !== 0;
    if(i === dragging_i) return drag_target_i % 2 !== 0;
    if(dragging_i < drag_target_i) {
      if(i > dragging_i && i <= drag_target_i) return i % 2 === 0;
    } else {
      if(i < dragging_i && i >= drag_target_i) return i % 2 === 0;
    }

    return i % 2 !== 0;
  }

  const is_drag_moved_up = (dragging_i: number | null, drag_target_i: number | null, i: number) => {
    if(dragging_i == null || drag_target_i == null) return false;
    return dragging_i < i && i <= drag_target_i;
  }

  const is_drag_moved_down = (dragging_i: number | null, drag_target_i: number | null, i: number) => {
    if(dragging_i == null || drag_target_i == null) return false;
    return dragging_i > i && i >= drag_target_i 
  }

  const on_pointer_move = (event: PointerEvent) => {
    pointer_x = event.x;
    pointer_y = event.y;
  }

  const on_drag_start = (i: number) => {
    const file = data.files.items[i];
    if(file != null) {
      const element = document.querySelector(`.file-item[data-file-id="${file._id}"]`) as HTMLElement;
      if(element != null) {
        drag_tag_source_element = element;
        dragging_height = `${element.clientHeight}px`;
      } else {
        drag_tag_source_element = element;
        dragging_height = "100%"
      }
    } else {
      drag_tag_source_element = null;
      dragging_height = "100%";
    }

    document.documentElement.classList.add("dragging");
    dragging_i = i;
    drag_autoscroll_timer = setInterval(drag_autoscroll, 2);

    add(window, "pointerup", on_drag_end, { capture: true, once: true })
  }

  const on_drag_end = async () => {
    document.documentElement.classList.remove("dragging");
    clearInterval(drag_autoscroll_timer);
    
    const from_i = dragging_i;
    const to_i = drag_target_i;

    if(from_i == null || to_i == null) {
      dragging_i = null;
      drag_target_i = null;
      return;
    }

    const from = data.files.items[from_i];
    const element = from && document.querySelector(`.file-item[data-file-id='${from._id}']`) as HTMLElement;
    
    drag_tag_target_element = element;

    dragging_i = null;
    drag_target_i = null;
    dropping = true;
    drag_reorder(from_i, to_i);
    element?.animate?.({
      opacity: [0, 1],
    }, {
      duration: 300,
      easing: "ease",
    });
    await sleep(300);
    dropping = false;
  }

  const on_drag_enter = async (i: number) => {
    if(dragging_i == null) return;
    if(drag_target_i === i) {
      if(dragging_i < i) drag_target_i = i - 1;
      else drag_target_i = i + 1;
    } else {
      drag_target_i = i;
    }
  }

  let restart_open = false;
  $: show_restart = get_show_restart($now_playing);
  const get_show_restart = (...args: any[]) => {
    if($now_playing == null) return false;
    if($now_playing.info.kind === "live") return false;
    if($now_playing.info.kind === "playlist") return true;
    if($now_playing.info.kind === "none") return $now_playing.info.start_on_connect;
  }

  const restart = action(async () => {
    restart_open = false;
    await _post(`/api/stations/${station_id}/restart-playlist`, undefined);
    _message("Palylist restarted");
    if($player_state.type === "station") {
      const station = $player_state.station;
      if(station._id === station_id && $player_state.audio_state !== "paused") {
        pause();
        play_station(station);
      }
    }
  })


  $: now_playing_file_id = $now_playing?.info.kind === "playlist" ? $now_playing.info.file._id : null; 

  $: station_id = data.station._id;
  const now_playing = get_now_playing_store(data.station._id, data.now_playing);
  $: if ($now_playing) data.now_playing = $now_playing.info;

  $: playlist_duration = getPlaylistDuration(data.files.items);

  const getPlaylistDuration = (files: AudioFile[]): number => {
    let d = 0;
    for(const item of files) {
      d += item.duration_ms;
    }
    return d;
  }

  $: files_set = new Set(uploading.map(item => item.file));
  
  const clear = async () => {
    
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

  type AudioFile = typeof data.files.items[number];

  const toggle_play = (file: AudioFile) => {
    if($player_playing_audio_file_id === file._id) {
      if($player_audio_state === "paused") resume();
      else pause();
    } else {
      play_track(file, data.station.picture_id);
    }
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

  let uploading: Item[] = [];
  let files: FileList | undefined;

  prevent_unload(() => {
    if(uploading.some(item => item.state === "waiting" || item.state === "uploading")) {
      return "Leaving this page will cancel pending uploads. Do you want to leave anyway?";
    } else {
      return null;
    }
  })

  let controller: AbortController | null = null;
  let unmounted = false;

  onMount(() => { 
    if(window.AbortController) controller = new AbortController();
     
    return () => {
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
      const _new_file: import("$server/defs/api/stations/[station]/files/POST/Output").Output = await _request(`/api/stations/${station_id}/files?filename=${encodeURIComponent(item.file.name)}`, {
        method: "POST",
        headers: {
          "content-length": String(item.file.size),
          "content-type": item.file.type,
        },
        signal: controller?.signal,
        body: item.file,
      })

      controller = null;
      item.state = "done";
      uploading = uploading;

      invalidate("api:stations/:id/limits");
      invalidate("api:stations/:id/files");

    } catch(e: any) {
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

  let _item_uid = 0;
  const upload_item_uid = () => {
    return ++_item_uid;
  }

  $: on_files(files);
  const on_files = (...args: any[]) => {
    if(!files) return;
    const _files = files;
    files = undefined;
    for(let i = 0; i < _files.length; i++) {
      const file = _files.item(i)!;
      if(files_set.has(file)) continue;
      const item: Item = { id: upload_item_uid(), file, state: "waiting" };
      uploading = [ ...uploading, item ];
      next();
    }
  }

  let deleting = false;

  const delete_file = action(async (file_id: string) => {
    deleting = true;

    try {
      await _delete(`/api/stations/${station_id}/files/${file_id}`);
      unselect(file_id);
      invalidate("api:stations/:id/limits");
      invalidate("api:stations/:id/files");
      _message("Track deleted");
      if($player_playing_audio_file_id === file_id) close();
    } catch(e) {
      sleep(300).then(() => deleting = false);
      throw e;
    }
    
    await sleep(300).then(() => deleting = false);
  })

  const del_selected = async () => {
    if(audio_item_to_delete == null) return;
    if((await delete_file(audio_item_to_delete._id)).ok) {
      audio_item_to_delete = null;
    } 
  }

  // TODO: fix open selection when only one item
  const set_del_selection_open = () => {
    const ids = $selected_ids;
    if(ids.length === 0) return;
    // if(ids.length === 1) {
    //   const item = data.files.items.find(item => item._id = ids[0]);
    //   if(item != null) audio_item_to_delete = item;
    //   else delete_selection_open = true;
    // } else {
      delete_selection_open = true;
    // }
  }

  const del_selection_all = action(async () => {
    
    deleting = true;

    try {

      delete_selection_open = false;
      const ids = $selected_ids;
      if(ids.length === 0) return;
      if(ids.length === 1) return delete_file(ids[0]);
      //const text = (i: number) => `Deleting ${ids.length} tracks... ${i} tracks deleted`;
      //const message = writable(text(0));
      const { resolve, reject } = _progress(`Deleting ${ids.length} tracks...`);
      try {
        let i = 0;
        for(const id of ids) {
          //await sleep(100);
          //message.set(text(i));
          await _delete(`/api/stations/${station_id}/files/${id}`);
          if($player_playing_audio_file_id && ids.includes($player_playing_audio_file_id)) close();
          data.files.items = data.files.items.filter(item => item._id !== id);
          data.files.total = data.files.items.length;
          i++;
        }
      } catch(e: any) {
        invalidate("api:stations/:id/limits");
        invalidate("api:stations/:id/files");
        $selected_ids = $selected_ids.filter(id => data.files.items.some(item => item._id === id));
        reject(String(e?.message));
        throw e;
      }

      invalidate("api:stations/:id/limits");
      invalidate("api:stations/:id/files");
      resolve(`${ids.length} tracks deleted`);
      $selected_ids = [];
  
    } catch(e) {
      sleep(300).then(() => deleting = false)
      throw e;
    }

    sleep(300).then(() => deleting = false)
  })

  import { expoOut } from "svelte/easing";
	import { writable } from "svelte/store";
	import TextField from "$lib/components/Form/TextField.svelte";
	import { get_now_playing_store } from "$lib/now-playing";
	import { sleep } from "$share/util";

  const file_item_out = (node: HTMLElement, { duration = 250 } = {}) => {
    return {
      css: (t: number, u: number) => {
        return `opacity: ${t}; transform: translateY(-${20 * u}px)`;
      },
      easing: expoOut,
      duration,
    }
  }

  let audio_item_to_delete: AudioFile | null = null;
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

  let audio_item_to_edit: AudioFile | null = null;
  let edit_current_title: string = "";
  let edit_current_artist: string = "";
  let edit_current_album: string = "";

  const open_edit_item = (item: AudioFile) => {
    edit_current_title = (item.metadata.title || "").trim();
    edit_current_artist = (item.metadata.artist || "").trim();
    edit_current_album = (item.metadata.album || "").trim();
    audio_item_to_edit = item;
  }

  const edit_save = action(async () => {
    if(audio_item_to_edit == null) return;
    const payload: import("$server/defs/api/stations/[station]/files/[file]/metadata/PATCH/Payload").Payload = {
      title: edit_current_title.trim() || null,
      artist: edit_current_artist.trim() || null,
      album: edit_current_album.trim() || null,
    }
    await _put(`/api/stations/${station_id}/files/${audio_item_to_edit._id}/metadata`, payload);
    invalidate("api:stations/:id/files");
    audio_item_to_edit = null;
  })

  // const move_up = async (index: number) => {
  //   await swap(index, index - 1);
  // }

  // const move_down = async (index: number) => {
  //   await swap(index, index + 1);
  // };

  // const move_to_first = action(async (index: number) => {
  //   const item = data.files.items[index];
  //   if(item == null) return;
  //   data.files.items.splice(index, 1);
  //   data.files.items = [item, ...data.files.items];
  //   try {
  //     await _post(`/api/stations/${station_id}/files/${item._id}/order/move-to-first`, undefined)
  //   } catch(e) {
  //     invalidate("station:files")
  //     throw e;
  //   }

  //   invalidate("station:files");
  // })

  // const move_to_last = action(async (index: number) => {
  //   const item = data.files.items[index];
  //   if(item == null) return;
  //   data.files.items.splice(index, 1);
  //   data.files.items = [...data.files.items, item];
  //   try {
  //     await _post(`/api/stations/${station_id}/files/${item._id}/order/move-to-last`, undefined)
  //   } catch(e) {
  //     invalidate("station:files")
  //     throw e;
  //   }

  //   invalidate("station:files");
  // })

  const drag_reorder = action(async (from_i: number, to_i: number) => {
    if(from_i === to_i) return;
    const file = data.files.items[from_i];
    const anchor = data.files.items[to_i];
    if(file == null || anchor == null || file._id === anchor._id) return;
   
    let sorted: AudioFile[] = [];
    for(const item of data.files.items) {
      if(item === file) continue;
      if(item === anchor) {
        if(from_i < to_i) {
          sorted.push(anchor, file);
        } else {
          sorted.push(file, anchor);
        }
      } else {
        sorted.push(item);
      }
    }

    data.files.items = sorted;

    try {
      if(from_i < to_i) {
        const payload: import("$server/defs/api/stations/[station]/files/[file]/order/move-after/POST/Payload").Payload = {
         anchor_file_id: anchor._id
        };
        await _post(`/api/stations/${station_id}/files/${file._id}/order/move-after`, payload);
      } else {
        const payload: import("$server/defs/api/stations/[station]/files/[file]/order/move-before/POST/Payload").Payload = {
           anchor_file_id: anchor._id
        }
        await _post(`/api/stations/${station_id}/files/${file._id}/order/move-before`, payload);
      }
    } catch(e) {
      invalidate("api:stations/:id/files");
      throw e;
    }

    invalidate("api:stations/:id/files");
  })

  // const swap = action(async (from_i: number, to_i: number) => {
  //   const from = data.files.items[from_i];
  //   const to = data.files.items[to_i];
  //   if(from == null || to == null) return;
  //   if(from._id === to._id) return;

  //   const payload: import("$server/defs/api/stations/[station]/files/[file]/order/swap/POST/Payload").Payload = {
  //     other_file_id: to._id
  //   };

  //   [data.files.items[from_i], data.files.items[to_i]] = [ to, from ];

  //   try {
  //     await _post(`/api/stations/${station_id}/files/${from._id}/order/swap`, payload);
  //   } catch(e) {
  //     invalidate("station:files");
  //     throw e;
  //   }

  //   invalidate("station:files");
  // })

  let file_input: HTMLInputElement;
  const open_file_input = (_event: MouseEvent) => {
    file_input?.click();
  }

  $: hide_tooltips = (
    dragging_item != null ||
    audio_item_to_delete != null ||
    audio_item_to_edit != null ||
    shuffle_open ||
    unshuffle_open ||
    ($selected_ids.length !== 0 && delete_selection_open)
  );


  let unshuffle_open = false;
  const unshuffle = action(async () => {
    unshuffle_open = false;
    try {
      await _post(`/api/stations/${station_id}/files/unshuffle`, undefined);
      _message("Playlist unshuffled");
    } catch(e) {
      invalidate("api:stations/:id/files").then(() => {
        play_shuffle_animation();
      });;
      throw e;
    }


    invalidate("api:stations/:id/files").then(() => {
      play_shuffle_animation();
    });
  })

  let shuffle_open = false;
  const shuffle = action(async () => {
    shuffle_open = false;
    try {
      await _post(`/api/stations/${station_id}/files/shuffle`, undefined);
      _message("Playlist shuffled");
    } catch(e) {
      invalidate("api:stations/:id/files").then(() => {
        play_shuffle_animation();
      });;
      throw e;
    }

    invalidate("api:stations/:id/files").then(() => {
      play_shuffle_animation();
    });;
  })

  const play_shuffle_animation = () => {
    const items = document.querySelectorAll(".file-item");
    
    // @ts-ignore
    if(items[0]?.animate) {
      for(let i = 0; i < items.length; i++) {
        const element = items[i] as HTMLElement;

        element.animate({
          opacity: [0, 0],
          transform: ["translateX(-60px)", "translateX(-60px)"],
        }, {
          duration: i * 50,
        }).onfinish = () => {
          element.animate({
            opacity: [0, 1],
            transform: ["translateX(-60px)", "none"],
          }, {
            duration: 300,
            easing: "ease",
          })
        };
      }
    }
  }

  const DRAG_TAG_IN_DURATION = 300;
  const DRAG_TAG_OUT_DURATION = 300;

  let dragging_animation: Animation[] | null = null;

  const cancel_dragging_animations = () => {
    if(dragging_animation) {
      for(const a of dragging_animation) {
        try {
          a.cancel();
        } finally {

        }

        dragging_animation = []
      }
    }
  }

  const dragging_tag_in = (element: HTMLElement, options = {}) => {
    return () => {
      if(!drag_tag_source_element?.animate) return scale(element, { duration: DRAG_TAG_IN_DURATION });

      cancel_dragging_animations();

      const style = getComputedStyle(element);
      const target_style = getComputedStyle(drag_tag_source_element);

      const tag_rect = element.getBoundingClientRect();
      const target_rect = drag_tag_source_element.getBoundingClientRect();
      
      const tag_text_element = element.querySelector(".dragging-tag-text") as HTMLElement; 
      const tag_text_rect = tag_text_element.getBoundingClientRect();
      const target_text_rect = (drag_tag_source_element.querySelector(".file-data-title") as HTMLElement).getBoundingClientRect();

      const x = target_rect.left - tag_rect.left;
      const y = target_rect.top - tag_rect.top;
      const target_width = target_text_rect.right - target_rect.left; 

      const text_x = (target_text_rect.left - target_rect.left) - (tag_text_rect.left - tag_rect.left);

      const a1 = tag_text_element.animate({
        "transform": [`translateX(${text_x}px)`, "none"],
      }, {
        duration: DRAG_TAG_IN_DURATION,
        easing: "ease",
        fill: "forwards",
      })      

      const a2 = element.animate({
        "opacity": [0, 1],
        "boxShadow": ["none", style.boxShadow],
        "borderRadius": [target_style.borderRadius, style.borderRadius],
        "backgroundColor": [target_style.backgroundColor, style.backgroundColor],
        "color": [ target_style.color, style.color ],
        "transform": [`translateX(${x}px) translateY(${y}px)`, "none"],
        "width": [`${target_width}px`, `${tag_rect.width}px`],
        "height": [`${target_rect.height}px`, `${tag_rect.height}px`],
      }, {
        duration: DRAG_TAG_IN_DURATION,
        easing: "ease",
        fill: "forwards",
      })

      dragging_animation = [a1, a2];

      return {
        duration: DRAG_TAG_IN_DURATION,
      }
    }
  }

  const dragging_tag_out = (element: HTMLElement, options = {}) => {
    return () => {
      if(!drag_tag_target_element?.animate) return scale(element, { duration: DRAG_TAG_OUT_DURATION });
     
      cancel_dragging_animations();

      const style = getComputedStyle(element);
      const target_style = getComputedStyle(drag_tag_target_element);

      const tag_rect = element.getBoundingClientRect();
      const target_rect = drag_tag_target_element.getBoundingClientRect();
      
      const tag_text_element = element.querySelector(".dragging-tag-text") as HTMLElement; 
      const tag_text_rect = tag_text_element.getBoundingClientRect();
      const target_text_rect = (drag_tag_target_element.querySelector(".file-data-title") as HTMLElement).getBoundingClientRect();

      const x = target_rect.left - tag_rect.left;
      const y = target_rect.top - tag_rect.top;
      const target_width = target_text_rect.right - target_rect.left; 
      // const w = target_rect.width / tag_rect.width;
      // const h = target_rect.height / tag_rect.height;

      const text_x = (target_text_rect.left - target_rect.left) - (tag_text_rect.left - tag_rect.left);

      const a1 = tag_text_element.animate({
        "transform": ["none", `translateX(${text_x}px)`],
      }, {
        duration: DRAG_TAG_OUT_DURATION,
        easing: "ease",
        fill: "forwards",
      })      

      const a2 = element.animate({
        "opacity": [1, 0],
        "boxShadow": [style.boxShadow, "none"],
        "borderRadius": [style.borderRadius, target_style.borderRadius],
        "color": [style.color, target_style.color],
        "backgroundColor": [style.backgroundColor, target_style.backgroundColor],
        "transform": ["none", `translateX(${x}px) translateY(${y}px)`],
        "width": [`${tag_rect.width}px`, `${target_width}px`],
        "height": [`${tag_rect.height}px`, `${target_rect.height}px`],
      }, {
        duration: DRAG_TAG_OUT_DURATION,
        easing: "ease",
        fill: "forwards",
      })

      dragging_animation = [a1, a2];

      return {
        duration: DRAG_TAG_OUT_DURATION,
      }
    }
  }

  import { cubicOut } from "svelte/easing";
  const shuffle_btn_transition = (element: HTMLElement, options = {}) => {
    const easing = cubicOut;
    const duration = 200;
    return () => {
      const parent = element.parentElement;
      if(parent == null) return scale(element, { duration, easing });
      const i = Array.prototype.indexOf.call(parent.children, element);
      if(i === -1 || i === 0) return scale(element, { duration, easing });
      const gap = parseFloat(getComputedStyle(parent).rowGap);
      const width = element.clientWidth;
      return {
        css: (t: number, u: number) => {
          return `margin-inline-start: ${(gap + width) * u * -1}px; transform: scale(${t});`
        },
        duration,
        easing,
      }
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

  .file-input {
    position: absolute;
    width: 1px;
    height: 1px;
    opacity: 0;
    user-select: none;
  }

  .upload-empty-out {
    margin-top: 1.5rem;
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

  .upload-message-box {
    margin-top: 1.5rem;
  }

  .upload-message > b {
    font-weight: 900;
  }

  .playlist-top {
    display: flex;
    flex-direction: row;
    align-items: center;
    column-gap: 1.5rem; 
    flex-wrap: wrap;
    padding: 1rem;
  }

  .playlist-top-title, .upload-box-title {
    font-weight: 600;
    font-size: 1.5rem;
  }
  
  .playlist-actions {
    display: flex;
    flex: none;
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
    margin-inline-start: auto;
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
  
  .file-data-item {
    padding: 1rem 1rem;
    font-weight: 400;
    font-size: 1rem;
    color: #555;
  }

  .file-data-text {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
    min-width: 8rem;
  }

  .file-preview-cell {
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
    border-radius: 0.5rem;
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
    position: relative;
    border-top: transparent 1px solid;
    transition: background-color 200ms ease, opacity 300ms ease, transform 200ms ease;
  }

  .file-item.even {
    background: #fff;
  }
  
  .file-item.odd {
    background: #f6f6f6;
  }

  .dropping .file-item {
    transition: none;
  }

  .file-item.dragging {
    z-index: 1;
    opacity: 0;
    pointer-events: none;
    /* transform: translateY(var(--dragging-offset-y)); */
    /*transition: background-color 150ms ease, opacity 200ms ease; */
  }

  .file-item.drag-moved-up {
    transform: translateY(calc(var(--dragging-height) * -1));
  }

  .file-item.drag-moved-down {
    transform: translateY(var(--dragging-height));
  }

  :global(html.dragging *) {
    user-select: none !important;
    cursor: move !important;
    cursor: -moz-grab !important;
    cursor: -webkit-grab !important;
    cursor: grab !important;
    cursor: -moz-grabbing !important;
    cursor: -webkit-grabbing !important;
    cursor: grabbing !important;
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
  
  .drag-cell {
    margin: 0;
    width: 2.5rem;
    font-size: 1.5rem;
    cursor: move; /* fallback if grab cursor is unsupported */
    cursor: -moz-grab;
    cursor: -webkit-grab;
    cursor: grab;
    transition: background-color 200ms ease;
    touch-action: none;
  }

  .not-dragging .drag-cell:hover {
    background-color: rgba(0,0,0,0.05);
  }

 /* (Optional) Apply a "closed-hand" cursor during drag operation. */
  .drag-cell:active {
    cursor: -moz-grabbing;
    cursor: -webkit-grabbing;
    cursor: grabbing;
  }

  .drag-handle {
    padding: 0 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
  }

  .file-btn-del {
    color: var(--red);
    border: var(--red) 2px solid;
    background: #fff;
    margin-inline-end: 1.5rem;
  }

  .file-btn-edit/*, .file-btn-move*/ {
    transition: background-color 150ms ease;
  }

  .not-dragging .file-btn-edit:hover/*, .file-btn-move:hover*/ {
    background-color: rgba(0,0,0,0.05);
  }

  /*
  .file-btn-hidden {
    visibility: hidden;
  }
  */

  .edit-dialog-btns, .delete-dialog-btns {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-end;
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .delete-btn,
  .edit-dialog-btn-cancel,
  .edit-dialog-btn-save,
  .delete-dialog-btn-delete,
  .delete-dialog-btn-cancel {
    padding: 0.5rem 0.75rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 0.25rem;
    transition: background-color 150ms ease;
  }

  .delete-btn:hover,
  .edit-dialog-btn-save:hover,
  .edit-dialog-btn-cancel:hover,
  .delete-dialog-btn-delete:hover,
  .delete-dialog-btn-cancel:hover {
    background: rgba(0,0,0,0.05);
  }

  .delete-btn, .delete-dialog-btn-delete {
    font-weight: 500;
    color: var(--red);
    border: 2px solid var(--red);
    box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
  }

  .edit-dialog-btn-save {
    font-weight: 500;
    color: var(--green);
    border: 2px solid var(--green);
    box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
  }

  .shuffle-dialog-btn {
    font-weight: 500;
    color: var(--blue);
    border: 2px solid var(--blue);
    box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
  }

  .delete-btn {
    border-radius: 100px;
  }

  .delete-dialog-btn-cancel {
    color: #555;
  }

  .edit-dialog-btn-icon, .delete-btn-icon, .delete-dialog-btn-icon {
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
    transition: background-color 150ms ease;
    border-radius: 50%;
    margin-inline: 0.25rem;
  }

  .not-dragging .select-all-btn:hover, .not-dragging .select-btn:hover {
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


  /*
  .cell-space-start {
    width: 1rem;
  }
  */

  .now-playing-circle {
    width: 0.65rem;
    height: 0.65rem;
    margin-inline: 0.5rem;
    border-radius: 50%;
    background-color: var(--green);
    opacity: 0;
    transform: scale(0);
    transition: transform 200ms ease, opacity 200ms ease;
  }

  .now-playing-circle.active {
    opacity: 1;
    transform: scale(1);
  }

  .edit-dialog-field:not(:first-child) {
    margin-top: 2rem;
  }

  .file-data-duration, .title-cell-duration {
    text-align: center;
  }

  .cell-title {
    min-width: 15rem;
  }

  .dragging-tag {
    position: fixed;
    z-index: 1000000;
    border-radius: 0.5rem;
    height: 3rem;
    padding: 1rem;
    line-height: 1rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    background: var(--blue);
    color: #fff;
    box-shadow: 0 4px 8px 0 rgba(0,0,0,.12),0 2px 4px 0 rgba(0,0,0,.08);
    pointer-events: none;
    transform-origin: top left;
  }

  .dragging-tag-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-inline-end: 0.5rem;
    font-size: 1rem;
  }

  .shuffle-btn {
    display: flex;
    flex: none;
    flex-direction: row;
    align-items: center;
    padding: 0.6rem;
    border-radius: 100px;
    color: var(--blue);
    transition: background-color 150ms ease;
  }

  .shuffle-btn:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .shuffle-btn-icon {
    display: flex;
    flex: none;
    align-items: cneter;
    justify-content: center;
    font-size: 1.3rem;
  }
</style>

<svelte:head>
  <title>Playlist</title>
</svelte:head>

<svelte:window on:pointermove={on_pointer_move} />

<Page>
  {@const all_selected = $selected_ids.length !== 0 && $selected_ids.length !== data.files.items.length}
  
  <h1>Playlist</h1>

  <div class="upload-message-box">
    <p class="upload-message">
      Create a playlist of music or old episodes to keep your station up 24x7.<br />
      When connection is lost or you are not broadcasting, <b>Playlist</b> will automatically take over.
    <p>
  </div>

  {#if data.files.total === 0 && uploading.length === 0}
   
    <div class="upload-empty-out">
      <button class="browse-btn ripple-container" use:ripple on:click={open_file_input}>
        Upload files
      </button>
    </div>
  
  {:else}

    <div class="upload-box">
      <div class="upload-top">
        <div class="upload-box-title">
          Upload
        </div>

        <button class="browse-btn ripple-container" use:ripple on:click={open_file_input}>
          Browse
        </button>
      </div>

      {#if uploading.length}
        <div class="uploading">
          {#each uploading as item (item.id)}
            <div class="upload-item" data-upload-state={item.state}>
              <div class="upload-item-name">{item.file.name}</div>
              <div class="upload-item-size">{upload_size(item.file.size)}</div>
              <div class="upload-icon">
                {#if item.state === "done"}
                  <div class="upload-icon-done" aria-label="Uploaded successfully" use:tooltip={"Uploaded successfully"} in:fade|local={{ duration: 200 }}>
                    <Icon d={mdiCheck} />
                  </div>
                {:else if item.state === "waiting"}
                  <div class="upload-icon-waiting" aria-label="Waiting" use:tooltip={"Waiting..."}  in:fade|local={{ duration: 200 }}>
                    <Icon d={mdiTimerPauseOutline} />
                  </div>
                {:else if item.state === "uploading"}
                  <div class="upload-icon-uploading" aria-current={true} aria-label="In progress" use:tooltip={"In progress..."}  in:fade|local={{ duration: 200 }}>
                    <CircularProgress />
                  </div>
                {:else if item.state === "error"}
                  {@const error_message = item.error.code ? `${item.error.message ?? "error"} (${item.error.code})` : item.error.message ?? "error"}
                  <div
                    class="upload-icon-error"
                    aria-errormessage={error_message}
                    use:tooltip={error_message}
                    in:fade|local={{ duration: 200 }}
                  >
                    <Icon d={mdiAlertDecagram} />
                  </div>
                {/if}
              </div>
              {#if item.state === "error"}
                <button class="upload-error-retry ripple-container" aria-label="Retry" use:tooltip={"Retry"} use:ripple on:click={() => retry(item)} >
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
        <div class="playlist-top-title">
          Tracks
          <span class="count">{data.files.total} {data.files.total === 1 ? "track" : "tracks"} - {total_duration(playlist_duration)}</span>
        </div>
        <!-- no shuffle for less than 2 files -->
        {#if show_restart || data.files.total >= 2}
          <div class="playlist-actions">
            {#if show_restart}
              <button
                class="shuffle-btn ripple-container"
                aria-label="Restart playlist"
                use:tooltip={"Restart playlist"}
                use:ripple
                on:click={() => restart_open = true}
                transition:shuffle_btn_transition|local
              >
                <div class="shuffle-btn-icon">
                  <Icon d={mdiRestart} />
                </div>
              </button>
            {/if}
            
            {#if data.files.total >= 2}
              {#if data.playlist_is_randomly_shuffled}
                <button
                  class="shuffle-btn ripple-container"
                  aria-label="Unshuffle playlist"
                  use:tooltip={"Unshuffle playlist"}
                  use:ripple
                  on:click={() => unshuffle_open = true}
                  transition:shuffle_btn_transition|local
                >
                  <div class="shuffle-btn-icon">
                    <Icon d={mdiShuffleDisabled} />
                  </div>
                </button>
              {/if}

              <button
                class="shuffle-btn ripple-container"
                aria-label="Shuffle playlist"
                use:tooltip={"Shuffle playlist"}
                use:ripple
                on:click={() => shuffle_open = true}
                transition:shuffle_btn_transition|local
              >
                <div class="shuffle-btn-icon">
                  <Icon d={mdiShuffleVariant} />
                </div>
              </button>
            {/if}
          </div>
        {/if}
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
        <div class="playlist-scroll-inner"
          on:pointerleave={event => {
            if(event.target === event.currentTarget) {
              drag_target_i = null;
            }
          }}
         >
         <table
          class="playlist-table"
          class:dropping
          class:not-dragging={dragging_item == null}
          style:--dragging-height={dragging_height}
        >
            <thead>
              <tr>
                <th class="grab-head-cell"></th>
                <th class="btn-cell">
                  <div class="select-all-cell">
                    <button
                        class="select-all-btn ripple-container"
                        class:check={all_selected}
                        aria-label={all_selected ? "Unselect all" : "Select all"}
                        use:ripple
                        on:click={toggle_selection_all}
                      >
                        {#if $selected_ids.length === 0}
                          <Icon d={mdiCheckboxBlankOutline} />
                        {:else if $selected_ids.length === data.files.items.length}
                          <Icon d={mdiCheck} />
                        {:else}
                          <Icon d={mdiCheckboxIntermediate} />
                        {/if}
                    </button>
                  </div>
                </th>
                <th class="btn-cell"></th>
                <th class="btn-cell"></th>
                <th class="cell-title">
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
                  <div class="title-cell-duration">
                    Duration
                  </div>
                </th>
                <!--
                  <th class="btn-cell"></th>
                  <th class="btn-cell"></th>
                  <th class="btn-cell"></th>
                  <th class="btn-cell"></th>
                -->
                <th class="btn-cell"></th>
                <th class="btn-cell"></th>
              </tr>
            </thead>
            <tbody in:fly|local={{ duration: 300, x: -50 }}>  
              {#each data.files.items as file, i (file._id)}
                {@const selected = $selected_ids.includes(file._id)}
                {@const player_playing = $player_playing_audio_file_id === file._id && $player_audio_state !== "paused"}
                {@const playlist_current = now_playing_file_id === file._id}
                {@const drag_target = drag_target_i === i}
                {@const dragging = dragging_i === i}
                {@const drag_moved_up = is_drag_moved_up(dragging_i, drag_target_i, i)}
                {@const drag_moved_down = is_drag_moved_down(dragging_i, drag_target_i, i)}
                {@const even = nth_even(dragging_i, drag_target_i, i)}
                <!--
                {@const can_move_up = i !== 0}
                {@const can_move_down = (i + 1) < data.files.items.length}  
                -->

                <tr class="file-item"
                  data-file-id={file._id}  
                  class:dragging
                  class:drag-target={drag_target}
                  class:drag-moved-up={drag_moved_up}
                  class:drag-moved-down={drag_moved_down}
                  class:even
                  class:odd={!even}
                  class:selected
                  aria-selected={selected}
                  in:fade|local={{ duration: 250 }}
                  out:file_item_out|local
                  on:pointerenter={() => on_drag_enter(i)}
                >
                  <td
                    class="drag-cell"
                    aria-label="Drag to rearrange"
                    use:tooltip={hide_tooltips ? null : "Drag to rearrange"}
                    on:pointerdown={event => {
                      pointer_x = event.x;
                      pointer_y = event.y;
                      // @ts-ignore
                      event.target?.releasePointerCapture?.(event.pointerId);
                      on_drag_start(i)
                    }}
                  >
                    <div class="drag-handle">
                      <Icon d={mdiDrag} />
                    </div>
                  </td>
                  
                  <td class="btn-cell">
                    <div class="select-cell">
                      <button
                        class="select-btn ripple-container"
                        class:check={selected}
                        aria-label={selected ? "Unselect" : "Select"}
                        use:ripple
                        on:click={() => toggle_select(file._id)}
                      >
                        {#if selected}
                          <Icon d={mdiCheck} />
                        {:else}
                          <Icon d={mdiCheckboxBlankOutline} />
                        {/if}
                      </button>
                    </div>
                  </td>
                  <td class="btn-cell">
                    <div class="file-preview-cell">
                      <button
                        class="file-preview-btn ripple-container"
                        aria-label={player_playing ? "Pause" : "Preview"}
                        use:ripple
                        on:click={() => toggle_play(file)}
                      >
                        {#if player_playing}
                          <Icon d={mdiPause} />
                        {:else}
                          <Icon d={mdiPlay} />
                        {/if}
                      </button>
                    </div>
                  </td>

                  <td class="btn-cell">
                    <div 
                      class="now-playing-circle"
                      role="presentation"
                      aria-hidden={!playlist_current}
                      aria-current={playlist_current}
                      class:active={playlist_current}
                    />
                  </td>

                  <td>
                    <div class="file-data-item">
                      <div class="file-data-text file-data-title">{file.metadata.title || file.filename}</div>
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
                    <div class="file-data-item file-data-duration">
                      {track_duration(file.duration_ms)}
                    </div>
                  </td>
                  <!--
                  <td class="btn-cell">
                    <button
                      class="file-btn file-btn-move ripple-container"
                      class:file-btn-hidden={!can_move_up}
                      use:ripple
                      use:tooltip={"Move to first"}
                      aria-hidden={!can_move_up}
                      aria-label="Move to first"
                      on:click={() => move_to_first(i)}
                    >
                      <Icon d={mdiChevronDoubleUp} />
                    </button>
                  </td>
                  <td class="btn-cell">
                    <button
                      class="file-btn file-btn-move ripple-container"
                      class:file-btn-hidden={!can_move_up}
                      use:ripple
                      use:tooltip={"Move upwards"}
                      aria-hidden={!can_move_up}
                      aria-label="Move upwards"
                      on:click={() => move_up(i)}
                    >
                      <Icon d={mdiChevronUp} />
                    </button>
                  </td>
                  <td class="btn-cell">
                    <button
                      class="file-btn file-btn-move ripple-container"
                      class:file-btn-hidden={!can_move_down}
                      use:ripple
                      use:tooltip={"Move downwards"}
                      aria-hidden={!can_move_down}
                      aria-label="Move downwards"
                      on:click={() => move_down(i)}
                    >
                      <Icon d={mdiChevronDown} />
                    </button>
                  </td>
                  <td class="btn-cell">
                    <button
                      class="file-btn file-btn-move ripple-container"
                      class:file-btn-hidden={!can_move_down}
                      use:ripple
                      use:tooltip={"Move to last"}
                      aria-hidden={!can_move_down}
                      aria-label="Move to last"
                      on:click={() => move_to_last(i)}
                    >
                      <Icon d={mdiChevronDoubleDown} />
                    </button>
                  </td>
                  -->
                  <td class="btn-cell">
                    <button
                      class="file-btn file-btn-edit ripple-container"
                      use:ripple
                      use:tooltip={"Edit"}
                      on:click={() => open_edit_item(file)}
                    >
                      <Icon d={mdiCircleEditOutline} />
                    </button>
                  </td>
                  <td class="btn-cell">
                    <button
                      class="file-btn file-btn-del ripple-container"
                      use:ripple
                      use:tooltip={"Delete"}
                      on:click={() => audio_item_to_delete = file}
                    >
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

   <input bind:this={file_input} class="file-input" type="file" multiple accept="audio/*" bind:files={files} />
</Page>

{#if audio_item_to_delete != null}
  <Dialog
    title="Delete track {audio_item_to_delete.metadata.title || audio_item_to_delete.filename}"
    width="400px"
    on_close={() => audio_item_to_delete = null}
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
    title="Delete {$selected_ids.length} {$selected_ids.length === 1 ? "track" : "tracks"}"
    width="400px"
    on_close={() => delete_selection_open = false}
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
{:else if audio_item_to_edit}
  <Dialog
    title="Edit track {audio_item_to_edit.metadata.title || audio_item_to_edit.filename}"
    width="400px"
    on_close={() => audio_item_to_edit = null}  
  >
    <div class="edit-dialog">
      <div class="edit-dialog-fields">
        <div class="edit-dialog-field">
          <TextField label="Title" bind:value={edit_current_title} />
        </div>
        <div class="edit-dialog-field">
          <TextField label="Artist" bind:value={edit_current_artist} />
        </div>
        <div class="edit-dialog-field">
          <TextField label="Album" bind:value={edit_current_album} />
        </div>
      </div>
      <div class="edit-dialog-btns">
        <button class="edit-dialog-btn-cancel ripple-container" use:ripple on:click={() => audio_item_to_edit = null}>
          Cancel
        </button>

        <button class="edit-dialog-btn-save ripple-container" use:ripple on:click={edit_save}>
          <div class="edit-dialog-btn-icon">
            <Icon d={mdiContentSaveOutline} />
          </div>
          Save
        </button>
      </div>
    </div>
  </Dialog>
{:else if shuffle_open}
  <Dialog
    title="Shuffle playlist"
    width="400px"
    on_close={() => shuffle_open = false}  
    >
    <div class="edit-dialog">

      <div class="delete-dialog-text">
        Are you sure you want to randomly shuffle the entire playlist?
      </div>

      <div class="edit-dialog-btns">
        <button class="edit-dialog-btn-cancel ripple-container" use:ripple on:click={() => shuffle_open = false}>
          Cancel
        </button>

        <button class="shuffle-dialog-btn edit-dialog-btn-save ripple-container" use:ripple on:click={shuffle}>
          <div class="edit-dialog-btn-icon">
            <Icon d={mdiShuffleVariant} />
          </div>
          Shuffle
        </button>
      </div>
    </div>
  </Dialog>
{:else if unshuffle_open}
    <Dialog
      title="Unshuffle playlist"
      width="400px"
      on_close={() => shuffle_open = false}  
    >
    <div class="edit-dialog">

      <div class="delete-dialog-text">
        Are you sure you want to unshuffle the entire playlist?
      </div>

      <div class="edit-dialog-btns">
        <button class="edit-dialog-btn-cancel ripple-container" use:ripple on:click={() => unshuffle_open = false}>
          Cancel
        </button>

        <button class="shuffle-dialog-btn edit-dialog-btn-save ripple-container" use:ripple on:click={unshuffle}>
          <div class="edit-dialog-btn-icon">
            <Icon d={mdiShuffleDisabled} />
          </div>
          Unshuffle
        </button>
      </div>
    </div>
  </Dialog>
{:else if restart_open}
  <Dialog
    title="Restart playlist"
    width="400px"
    on_close={() => restart_open = false}  
  >
  <div class="edit-dialog">

    <div class="delete-dialog-text">
      Are you sure you want to restart the playlist?
    </div>

    <div class="edit-dialog-btns">
      <button class="edit-dialog-btn-cancel ripple-container" use:ripple on:click={() => restart_open = false}>
        Cancel
      </button>

      <button class="shuffle-dialog-btn edit-dialog-btn-save ripple-container" use:ripple on:click={restart}>
        <div class="edit-dialog-btn-icon">
          <Icon d={mdiRestart} />
        </div>
        Restart
      </button>
    </div>
  </div>
</Dialog>
{/if}

{#if dragging_item != null}
  <div class="dragging-tag" in:dragging_tag_in|local out:dragging_tag_out|local style="top: {dragging_tag_y}px; left: {dragging_tag_x}px">
    <div class="dragging-tag-icon">
      <Icon d={mdiMusic} />
    </div>
    <div class="dragging-tag-title">
      <div class="dragging-tag-text">
        {dragging_item.metadata.title || dragging_item.filename}
      </div>
    </div>
  </div>
{/if}