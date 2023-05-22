<script lang="ts">
	import { invalidate } from '$app/navigation';
	export let data: import('./$types').PageData;
	import TextField from '$lib/components/Form/TextField.svelte';
	import Page from '$lib/components/Page.svelte';
	import { locale } from '$lib/locale';
	import Dialog from '$share/Dialog.svelte';
	import Icon from '$share/Icon.svelte';
	import { _post, action } from '$share/net.client';
	import { _message } from '$share/notify';
	import { ripple } from '$share/ripple';
	import { mdiContentCopy, mdiLink, mdiLockReset } from '@mdi/js';

  import _copy from "copy-to-clipboard";

  const copy = (value: string) => {
    return {
      icon: mdiContentCopy,
      label: $locale.copy_to_clipboard,
      action: () => {
        _copy(value);
        _message($locale.pages["station/broadcast"].notifier.copied_to_clipboard);
      }
    }
  }

  let reset_password_open = false;

  const reset_password = action(async () => {
    const { new_password }: import("$api/stations/[station]/reset-source-password/POST/Output").Output =
      await _post(`/api/stations/${data.station._id}/reset-source-password`, undefined);
    
    data.station.source_password = new_password;
    reset_password_open = false;
    _message($locale.pages["station/broadcast"].notifier.mount_password_reset);
    invalidate("resource:stations");
  })

</script>

<style>
	.page {
		display: flex;
		flex-direction: column;
		align-items: center;
    --field-container-bg: #f3f3f3;
	}

	.page-title {
		margin-top: 3rem;
		margin-bottom: 4rem;
		font-size: 2rem;
		font-weight: 600;
	}

	.settings {
		background: #fff;
		box-shadow: var(--some-shadow);
		display: flex;
		flex-direction: row;
		width: min(95%, 1200px);
		border-radius: 0.5rem;
    padding: 2rem 3rem 3rem 3rem;
    gap: 5rem;
    margin-bottom: 5rem;
	}

	.side {
		flex: 1;
	}

	.side-title {
		font-size: 1.2rem;
		font-weight: 600;
	}

  .urls {
    display: flex;
    flex-direction: column;
    margin-top: 1.5rem;
    gap: 2rem;
  }

  .url-field {
    margin-top: 0.35rem;
  }

  .ice-settings {
    margin-top: 3rem;
  }

  .ice-field + .ice-field {
    margin-top: 2rem;
  }

  .ice-pass-out {
    display: flex;
    flex-direction: row;
    align-items: stretch;
  }

  .ice-pass {
    flex: 1;
  }

  .ice-pass-reset-btn {
    flex: none;
    color: var(--blue);
    border: var(--blue) 1px solid;
    background: #fff;
    padding: 0 1rem;
    border-radius: 0 0.5rem 0.5rem 0;
  }


  @media screen and (max-width: 700px) {
    .settings {
      flex-direction: column;
    }
  }

  .reset-dialog-btns {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-end;
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .reset-dialog-btn-cancel,
  .reset-dialog-btn {
    padding: 0.5rem 0.75rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 0.25rem;
    transition: background-color 150ms ease;
  }

  .reset-dialog-btn:hover,
  .reset-dialog-btn-cancel:hover {
    background: rgba(0,0,0,0.05);
  }

  .reset-dialog-btn {
    font-weight: 500;
    color: var(--blue);
    border: 2px solid var(--blue);
    box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
  }

  .reset-dialog-btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-inline: -0.25rem 0.5rem;
    font-size: 1.2rem;
  }
</style>

<svelte:head>
	<title>{$locale.pages["station/broadcast"].head.title}</title>
</svelte:head>

<Page>
	<div class="page">
		<div class="page-title">{$locale.pages["station/broadcast"].title}</div>

		<div class="settings">
			<div class="side start">
				<div class="side-title">{$locale.pages["station/broadcast"].icecast_settings}</div>
				<div class="ice-settings">
          <div class="ice-field">
            <TextField
              label={$locale.pages["station/broadcast"].fields.address}
              value={data.config.source_public_host}
              btn={copy(data.config.source_public_host)}
              readonly
            />
          </div>
          <div class="ice-field">
            <TextField
              label={$locale.pages["station/broadcast"].fields.port}
              value="{`${data.config.source_public_port}`}"
              btn={copy(`${data.config.source_public_port}`)}
              readonly
            />
          </div>
          <div class="ice-field">
            <TextField
              label={$locale.pages["station/broadcast"].fields.mountpoint}
              value="{data.station._id}/source"
              btn={copy(`${data.station._id}/source`)}
              readonly
            />
          </div>
          <div class="ice-field">
            <TextField
              label={$locale.pages["station/broadcast"].fields.username}
              value="source"
              btn={copy("source")}
              readonly
            />
          </div>
          <div class="ice-field">
            <div class="ice-pass-out">
              <div class="ice-pass">
                <TextField 
                  label={$locale.pages["station/broadcast"].fields.password}
                  value={data.station.source_password}
                  btn={copy(data.station.source_password)}
                  readonly
                />
              </div>
              <button class="ice-pass-reset-btn ripple-container" use:ripple on:click={() => reset_password_open = true}>
                {$locale.pages["station/broadcast"].password_reset}
              </button>
            </div>
          </div>
          <div class="ice-field">
            <div class="ice-pass-wrap">
              <TextField
                label={$locale.pages["station/broadcast"].fields.encoding} 
                value="MP3 {$locale.pages["station/broadcast"].encoding_or} AAC"
                readonly
              />
            </div>
          </div>
        </div>
			</div>
			<div class="side end">
				<div class="side-title">
          {$locale.pages["station/broadcast"].links.title}
        </div>
				<div class="side-content">
					<div class="urls">
						<div class="url">
							<div class="url-title">
                {$locale.pages["station/broadcast"].links.main}
              </div>
							<div class="url-field">
								<TextField
									label=""
									icon={mdiLink}
									readonly
									btn={copy(`${data.config.stream_public_url}/stream/${data.station._id}`)}
									value="{data.config.stream_public_url}/stream/{data.station._id}"
								/>
							</div>
						</div>
						<div class="url">
							<div class="url-title">M3U</div>
							<div class="url-field">
								<TextField
									label=""
									icon={mdiLink}
									readonly
                  btn={copy(`${data.config.stream_public_url}/stream/${data.station._id}.m3u`)}
									value="{data.config.stream_public_url}/stream/{data.station._id}.m3u"
								/>
							</div>
						</div>
						<div class="url">
							<div class="url-title">PLS</div>
							<div class="url-field">
								<TextField
									label=""
									icon={mdiLink}
									readonly
                  btn={copy(`${data.config.stream_public_url}/stream/${data.station._id}.pls`)}
									value="{data.config.stream_public_url}/stream/{data.station._id}.pls"
								/>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</Page>

{#if reset_password_open}
  <Dialog
    title={$locale.pages["station/broadcast"].dialogs.reset_password.title}
    width="400px"
    on_close={() => reset_password_open = false}  
    >
    <div class="reset-dialog">

    <div class="reset-dialog-text">
      {$locale.pages["station/broadcast"].dialogs.reset_password.message}
    </div>

    <div class="reset-dialog-btns">
      <button class="reset-dialog-btn-cancel ripple-container" use:ripple on:click={() => reset_password_open = false}>
        {$locale.pages["station/broadcast"].dialogs.reset_password.cancel}
      </button>

      <button class="reset-dialog-btn ripple-container" use:ripple on:click={reset_password}>
        <div class="reset-dialog-btn-icon">
          <Icon d={mdiLockReset} />
        </div>
        {$locale.pages["station/broadcast"].dialogs.reset_password.submit}
      </button>
    </div>
    </div>
  </Dialog>
  {/if}