<script lang="ts">
	export let account: import('$server/defs/api/accounts/[account]/GET/Output').Output['account'];
	export let picture_id: string | null;

	import no_img from '$share/img/no-img-square.jpg';

	import { page } from '$app/stores';
	import CircularProgress from '$share/CircularProgress.svelte';
	import { ripple } from '$share/ripple';
	import { _post } from '$share/net.client';

	let error_message: string | null;
	let loading: boolean = false;

	$: bg_url = picture_id
		? `${$page.data.config.storagePublicURL}/station-pictures/webp/256/${picture_id}.webp`
		: no_img;

  let files: FileList | undefined;

  let _token = 0; 

  $: files?.[0] && on_file(files[0])
  const on_file = async (file: File) => {
    const token = ++_token;
    error_message = null;
    loading = true;

    try {
      const query: import("$server/defs/api/station-pictures/POST/Query").Query = {
        account_id: account._id,
        content_type: file.type,
        filename: file.name,
      };

      let qs = new URLSearchParams();
      qs.append("account_id", query.account_id);
      qs.append("content_type", query.content_type);
      qs.append("filename", query.filename);

      const data = await file.arrayBuffer();

      if(token !== _token) return;

      const res = await fetch(`/api/station-pictures?${qs}`, {
        method: "POST",
        body: data
      }).catch(e => {
        throw new Error("Could not connect with server");
      })

      if(token !== _token) return;

      const json: any = await res.json().catch(e => {
        throw new Error("Invalid JSON response from server");
      })

      if(token !== _token) return;

      if(json?.error?.message){
        throw new Error(String(json.error.message))
      }

      let picture = json as import("$server/defs/api/station-pictures/POST/Output").Output;
      picture_id = picture._id;

    } catch(e: any) {
      if(token === _token) error_message = String(e?.message);
    } finally {
      if(token === _token) loading = false;
    }
  }


</script>


<style>
	.station-picture-field {
		display: flex;
		flex-direction: row;
    gap: 2rem;
    padding: 2rem 2rem 0 2rem;
	}

  .start {
    flex: none;
  }

  .end {
    flex: 1;
  }

  .info-line + .info-line {
    margin-top: 0.25rem;
  }

  .btn-out {
    margin-bottom: 1rem;
  }

	.picture-out {
		width: 13rem;
		height: 13rem;
		display: flex;
		align-items: stretch;
		justify-content: stretch;
		overflow: hidden;
		border-radius: 0.5rem;
	}

	.picture-bg {
		flex: 1;
		background-size: contain;
		background-position: center;
		background-repeat: no-repeat;
	}

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
  }

	.error-message {
		color: var(--red);
	}
  
  input[type="file"] {
    display: none;
  }

  .btn-out {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  label {
    padding: 0.75rem 1rem;
    cursor: pointer;
    color: var(--blue);
    border-radius: 0.25rem;
    border: 1px var(--blue) solid;
    user-select: none;
    display: flex;
  }
  
  .error-message {
    margin-top: 1rem;
  }

  @media screen and (max-width: 600px) {
    .station-picture-field {
      flex-direction: column;
      gap: 1rem;
    }
  } 
</style>


<div class="station-picture-field">
	<div class="start">
		<div class="picture-out">
			<div class="picture-bg" style="background-image: url({bg_url})">
        {#if loading}
          <div class="loading">
            <CircularProgress />
          </div>
        {/if}
      </div>
		</div>
	</div>
	<div class="end">
    <div class="btn-out">
      <label class="ripple-container" use:ripple>
        Upload Image
        <input type="file" accept="image/*" bind:files />
      </label>
    </div>

    <div class="info-line">Image formats accepted: .jpg .jpeg .png .webp .gif</div>
		<div class="info-line">Minimum image size: 512x512px</div>
		<div class="info-line">Maximum file size: 2MB</div>
		<div class="info-line">Image must be square</div>
		{#if error_message != null}
			<div class="error-message">{error_message}</div>
		{/if}
	</div>
</div>