<script lang="ts">
	import { invalidate } from "$app/navigation";
	import Page from "$lib/components/Page.svelte";
	import { ripple } from "$lib/ripple";
	import { ClientError, _post, _request } from "$share/net.client";
	import type { PageData } from "./$types";
  export let data: PageData;
  import { page } from "$app/stores";

  $: accountId = $page.params.account;
  $: accountFiles = data.files as import("$server/defs/api/accounts/[account]/files/GET/Output").Output;

  type State = { state: "uploading", error?: undefined } | { state: "waiting", error?: undefined } | { state: "error", error: ClientError };

  type Item = { file: File, id: number } & State;

  const set = new Set<File>();
  let uploading: Item[] = [];  
  let current: Item | null = null;
  let files: FileList;

  const next = async () => {
    if(current != null) return;
    const item = uploading.find(item => item.state === "waiting");
    if(item == null) return;
    item.state = "uploading";
    uploading = uploading;
    current = item;
    const id = item.id;
    try {
      const newFile: import("$server/defs/api/accounts/[account]/files/POST/Output").Output = await _request(`/api/accounts/${accountId}/files?filename=${encodeURIComponent(item.file.name)}`, {
        method: "POST",
        headers: {
          "content-length": String(item.file.length),
          "content-type": item.file.type,
        },
        body: item.file,
      })

      uploading = uploading.filter(item => item.id !== id);

      invalidate("api:files");

    } catch(e: any) {
      set.delete(item.file);
      item.state = "error";
      item.error = e;
      uploading = uploading;
    }

    current = null;
    await next();
  }

  $: onFiles(files);
  const onFiles = (files?: FileList) => {
    if(!files) return;
    for(let i = 0; i < files.length; i++) {
      const file = files.item(i)!;
      if(set.has(file)) continue;
      const item: Item = { id: Math.random(), file, state: "waiting" };
      uploading = [ ...uploading, item ];
      next();
    }
  }
</script>

<style>
  .upload {
    padding: 0.75rem 1rem;
    color: #fff;
    background-color: var(--blue);
    border: 0;
    appearance: none;
    cursor: pointer;
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 16%);
  }

  .empty-message {
    font-size: 1.25rem;
    margin: 1.5rem 0 2.25rem 0;
  }

  .file-input {
    display: none;
  }

  .files {
    margin-bottom: 2rem;
  }

  .upload-wrap {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .file-item {
    display: flex;
    flex-direction: row;
    padding: 1rem;
  }
</style>

<Page>
  <h1>Media</h1>

  {#if accountFiles.total === 0}
    <div class="empty">
      <div class="empty-message">
        Your media collection is empty
      </div>
      
      <div class="upload-wrap">
        <label class="upload ripple-container" use:ripple>
          Upload files
          <input class="file-input" type="file" multiple accept="audio/*" bind:files={files} />
        </label>
      </div>
    </div>
  {:else}
    <div class="files">
      {#each accountFiles.items as file (file._id)}
        <div class="file-item">
          <div class="file-data">
            <div class="file-item-name">Name: {file.metadata.title || file.filename}</div>
            <div class="file-item-artist">Artist: {file.metadata.artist || "Unkown"}</div>
            <div class="file-item-album">Album: {file.metadata.album || "Unkown"}</div>
          </div>
          <div class="file-play">
            <audio controls preload="metadata" src="/api/accounts/{accountId}/files/{file._id}/stream" />
          </div>
        </div>
      {/each}
    </div>

    <div class="upload-wrap upload-wrap-2">
      <label class="upload ripple-container" use:ripple>
        Upload files
        <input class="file-input" type="file" multiple accept="audio/*" bind:files={files} />
      </label>
    </div>
  {/if}

  {#if uploading.length}
    <div class="uploading">
      {#each uploading as item (item.id)}
        {#if item.state === "waiting"}
          <div class="upload-item upload-item-waiting">
            File {item.file.name} waiting in queue ...
          </div>
        {:else if item.state === "uploading"}
          <div class="upload-item upload-item-uploading">
            Uploading file {item.file.name} ...
          </div>
        {:else if item.state === "error"}
          <div class="upload-item upload-item-error">
            Upload of file {item.file.name} failed: {item.error.message} ({item.error.code})
          </div>
        {/if}
      {/each}
    </div>
  {/if}
</Page>