<script lang="ts">
	export let data: import('./$types').PageData;
	import TextField from '$lib/components/Form/TextField.svelte';
	import Page from '$lib/components/Page.svelte';
	import { _message } from '$share/notify';
	import { ripple } from '$share/ripple';
	import { mdiContentCopy, mdiLink } from '@mdi/js';

  import _copy from "copy-to-clipboard";

  const copy = (value: string) => {
    return {
      icon: mdiContentCopy,
      action: () => {
        _copy(value);
        _message("Copied to clipboard");
      }
    }
  }

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
</style>

<svelte:head>
	<title>Broadcast</title>
</svelte:head>

<Page>
	<div class="page">
		<div class="page-title">Broadcast</div>

		<div class="settings">
			<div class="side start">
				<div class="side-title">Icecast Settings</div>
				<div class="ice-settings">
          <div class="ice-field">
            <TextField
              label="Server address"
              value={data.config.source_public_url}
              btn={copy(data.config.source_public_url)}
              readonly
            />
          </div>
          <div class="ice-field">
            <TextField
              label="Port"
              value="80"
              btn={copy("80")}
              readonly
            />
          </div>
          <div class="ice-field">
            <TextField
              label="Mountpoint"
              value="{data.station._id}/source"
              btn={copy(`${data.station._id}/source`)}
              readonly
            />
          </div>
          <div class="ice-field">
            <TextField
              label="Username"
              value="source"
              btn={copy("source")}
              readonly
            />
          </div>
          <div class="ice-field">
            <div class="ice-pass-out">
              <div class="ice-pass">
                <TextField 
                  label="Password"
                  value={data.station.source_password}
                  btn={copy(data.station.source_password)}
                  readonly
                />
              </div>
              <button class="ice-pass-reset-btn ripple-container" use:ripple>
                Reset
              </button>
            </div>
          </div>
          <div class="ice-field">
            <div class="ice-pass-wrap">
              <TextField label="Encoding" value="MP3 or AAC" readonly />
            </div>
          </div>
        </div>
			</div>
			<div class="side end">
				<div class="side-title">Stream URLs</div>
				<div class="side-content">
					<div class="urls">
						<div class="url">
							<div class="url-title">MAIN</div>
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
							<div class="url-title">M3U</div>
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