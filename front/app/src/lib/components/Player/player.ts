import { browser } from "$app/environment";
import { default_logger } from "$share/logger";
import { get_now_playing_store, type NowPlaying } from "$lib/now-playing";
import { derived, get, writable } from "svelte/store";
import { page } from "$app/stores";
import { equals } from "$server/util/collections";
import { locale } from "$share/locale";
import { STATION_PICTURES_VERSION } from "$server/defs/constants";
import type { Unwrap, GET } from "$lib/client";

export type PlayerState = PlayerState.Closed | PlayerState.Station | PlayerState.AudioFile;

export type AudioState = "playing" | "loading" | "paused";

const logger = default_logger.scoped("player");

const hasMediaSession = browser && ("mediaSession" in navigator) && typeof MediaMetadata !== "undefined";

export namespace PlayerState {
  export interface Base {
    type: string
  }

  export interface Closed extends Base {
    type: "closed"
  }

  export interface Station extends Base {
    type: "station"
    audio_state: AudioState,
    station: {
      _id: string,
      picture_id: string,
      name: string
    }
  }

  export type AudioFileItem = Unwrap<Awaited<ReturnType<typeof GET<"/stations/{station}/files/{file}">>>>["item"];

  export interface AudioFile extends Base {
    type: "track"
    audio_state: AudioState,
    file: AudioFileItem,
    picture_id: string,
  }
}

let audio: HTMLAudioElement | null = null;

const now_playing = writable<NowPlaying | null>(null);
const readonly = { subscribe: now_playing.subscribe };
export { readonly as player_now_playing }

export const storage_audio_url = (station_id: string, file_id: string) => {
  const base: string = get(page).data.config.storage_public_url;
  return `${base}/stations/${station_id}/files/${file_id}/stream?token=${media_token()}`
}

export const station_stream_url = (station_id: string) => {
  const base = get(page).data.config.stream_public_url;
  return `${base}/stream/${station_id}`
}


export const media_token = () => {
  return get(page).data.user?.media_key ?? "";
}

let current_now_playing_unsub: (() => void) | null = null;
const now_playing_start = (station_id: string) => {
  now_playing_stop();
  logger.info("now playing subscriber start");
  const store = get_now_playing_store(station_id);
  current_now_playing_unsub = store.subscribe(v => now_playing.set(v?.info ?? null));
}

const now_playing_stop = () => {
  now_playing.set(null);
  if (current_now_playing_unsub) {
    logger.info("now playing subscriber stop")
    current_now_playing_unsub();
    current_now_playing_unsub = null;
  }
}


export const pause = () => {
  logger.info("pause()");
  audio?.pause();
  set_audio_state("paused");
  const $state = get(player_state);
  if ($state.type === "closed") { }
  else if ($state.type === "track") { }
  else if ($state.type === "station") {
    logger.info("destroy tag");
    destroy_audio_tag();
  }
  else assert_never($state);
}

export const resume = () => {
  const $state = get(player_state);
  if ($state.type === "closed") logger.warn("resume called with player_state.type === 'closed'");
  else if ($state.type === "track") audio?.play();
  else if ($state.type === "station") {
    if ($state.audio_state === "paused") {
      const audio = get_audio_tag(station_stream_url($state.station._id));
      audio.play();
    }
  } else assert_never($state);
}

const player_state = writable<PlayerState>({ type: "closed" });
const readable_player_state = { subscribe: player_state.subscribe };
export { readable_player_state as player_state };

export const player_title = derived(player_state, (state): string => {
  if (state.type === "closed") return "";
  else if (state.type === "track") return state.file.metadata.title || state.file.filename;
  else if (state.type === "station") return state.station.name;
  else return assert_never(state);
})

export const player_subtitle = derived([player_state, now_playing], ([state, now_playing]): string | null | undefined => {
  if (state.type === "closed") return null;
  else if (state.type === "track") return state.file.metadata.artist;
  else if (state.type === "station") {
    if (now_playing == null) return null;
    else if (now_playing.kind === "none") return null;
    else if (now_playing.kind === "live") {
      const title = now_playing.title?.trim() || null;
      const artist = now_playing.artist?.trim() || null;
      if (title && artist) {
        return `${title} - ${artist}`
      } else if (title) {
        return title
      } else if (artist) {
        return artist
      } else {
        return get(locale).misc.player.Live_Streaming;
      }
    } else if (now_playing.kind === "playlist") {
      const title = now_playing.title?.trim() || now_playing.filename.trim() || null;
      const artist = now_playing.artist;
      if (title && artist) {
        return `${title} - ${artist}`
      } else if (title) {
        return title;
      } else if (artist) {
        return artist;
      } else {
        return get(locale).misc.player.Playlist;
      }
    } else if(now_playing.kind === "external-relay") {
      return get(locale).misc.player.Relay;
    } 
    
    else {
      return assert_never(now_playing)
    }
  } else {
    return assert_never(state)
  }
})

export const player_playing_audio_file_id = derived(player_state, (state): string | null => {
  if (state.type === "track") return state.file._id;
  else return null;
})

export const player_playing_station_id = derived(player_state, (state): string | null => {
  if (state.type === "station") return state.station._id;
  else return null;
})

export const player_audio_state = derived(player_state, (state): AudioState => {
  if (state.type === "closed") return "paused";
  else if (state.type === "station") return state.audio_state;
  else if (state.type === "track") return state.audio_state;
  else return assert_never(state);
})

export const play_station = (station: { _id: string, picture_id: string, name: string }) => {
  if (!browser) throw new Error("player.play_station called in ssr context");
  const $state = get(player_state);
  if ($state.type === "station" && $state.station._id === station._id) {
    resume();
  } else {
    destroy_audio_tag();
    player_state.set({
      type: "station",
      audio_state: "loading",
      station,
    })
    const audio = get_audio_tag(station_stream_url(station._id))
    audio.play().catch(e => {
      logger.warn(`error playing station ${station._id} => ${e}`)
    })

    now_playing_start(station._id);
  }
}


export const player_picture_id = derived(player_state, $player_state => {
  if ($player_state.type === "closed") return null;
  else if ($player_state.type === "station") return $player_state.station.picture_id;
  else if ($player_state.type === "track") return $player_state.picture_id;
  else assert_never($player_state);
})

// we use derived to subscribe to two store at once
// we need to subscribe to the store, derived only runs if it has subscribers
derived([player_state, now_playing], ([$player_state, $now_playing]) => {
  if (
    $player_state.type === "station" &&
    //$player_state.audio_state === "paused" &&
    $now_playing?.kind === "none" &&
    $now_playing.start_on_connect === false
  ) {
    close();
  }
}).subscribe(() => { })

// media session
if (hasMediaSession) {
  
  navigator.mediaSession.setActionHandler("pause", () => {
    pause()
  })

  navigator.mediaSession.setActionHandler("play", () => {
    const $player_state = get(player_state);
    if ($player_state.type === "closed") return;
    else if ($player_state.type === "station") {
      play_station($player_state.station);
    } else if ($player_state.type === "track") {
      resume();
    } else {
      assert_never($player_state);
    }
  })

  derived([player_state, now_playing], ([$player_state, $now_playing]) => {

    if ($player_state.type === "closed") {
      navigator.mediaSession.metadata = null;
      navigator.mediaSession.playbackState = "none";
      logger.info("set mediaSession metadata", null);
      logger.info("set mediaSession playbackState", "none");
    } else {
      let title: string | undefined;
      let artist: string | undefined;
      let picture_id: string;

      if ($player_state.type === "station") {
        picture_id = $player_state.station.picture_id;
        if ($now_playing) {
          if ($now_playing.kind === "live") {
            if($now_playing.title && $now_playing.artist) {
              title = `${$now_playing.title} - ${$now_playing.artist}`;
              artist = $player_state.station.name;
            } else if($now_playing.title) {
              title = $now_playing.title;
              artist = $player_state.station.name;
            } else {
              title = $player_state.station.name;
              artist = get(locale).misc.player.Live_Streaming
            }
          } else if ($now_playing.kind === "playlist") {
            if($now_playing.artist) {
              title = `${$now_playing.title || $now_playing.filename} - ${$now_playing.artist}`;
              artist = $player_state.station.name;
            } else {
              title = `${$now_playing.title || $now_playing.filename}`;
              artist = $player_state.station.name;
            }
          } else if ($now_playing.kind === "none") {
            title = $player_state.station.name;
            artist = undefined;
          } else if ($now_playing.kind === "external-relay") {
            title = $player_state.station.name;
            artist = undefined;
          }
        } else {
          title = $player_state.station.name;
          artist = undefined;
        }
      } else if ($player_state.type === "track") {
        picture_id = $player_state.picture_id;
        title = $player_state.file.metadata.title || $player_state.file.filename;
        artist = $player_state.file.metadata.artist || undefined;
      } else {
        return assert_never($player_state)
      }

      const storage_url = get(page).data.config.storage_public_url;

      const artwork = [32, 64, 128, 192, 256, 512].map(size => {
        return {
          src: `${storage_url}/station-pictures/png/${size}/${picture_id}.png?v=${STATION_PICTURES_VERSION}`,
          sizes: `${size}x${size}`,
          type: "image/png",
        }
      })

      const prev_metadata = {
        title: navigator.mediaSession.metadata?.artist || undefined,
        artist: navigator.mediaSession.metadata?.album || undefined,
        artwork: navigator.mediaSession.metadata?.artwork || undefined,
      }

      const new_metadata = {
        title,
        artist,
        artwork
      }
      
      if(!equals(prev_metadata, new_metadata)) {
        navigator.mediaSession.metadata = new MediaMetadata(new_metadata);
        logger.info("set mediaSession metadata", new_metadata);
      }
    }
  }).subscribe(() => { })

  player_audio_state.subscribe($player_audio_state => {
    if ($player_audio_state === "paused") {
      navigator.mediaSession.playbackState = "paused";
      logger.info("set mediaSession playbackState", "paused");
    } else if ($player_audio_state === "playing" || $player_audio_state === "loading") {
      logger.info("set mediaSession playbackState", "playing");
      navigator.mediaSession.playbackState = "playing"
    } else {
      return assert_never($player_audio_state);
    }
  })
}

export const play_track = (file: PlayerState.AudioFileItem, picture_id: string) => {
  if (!browser) throw new Error("player.play_track called in ssr context");
  destroy_audio_tag();
  now_playing_stop();
  player_state.set({
    type: "track",
    file,
    audio_state: "loading",
    picture_id,
  })

  const audio = get_audio_tag(storage_audio_url(file.station_id, file._id));

  audio.play().catch(e => {
    logger.warn(`error playing audio track ${file._id} => ${e}`);
  })
}

export const close = () => {
  destroy_audio_tag()
  now_playing_stop();
  player_state.set({ type: "closed" });
}

const destroy_audio_tag = () => {
  if (audio != null) {
    audio.pause();
    audio.src = "data:audio/wav;base64,UklGRjIAAABXQVZFZm10IBIAAAABAAEAQB8AAEAfAAABAAgAAABmYWN0BAAAAAAAAABkYXRhAAAAAA==";
  }
}

const set_audio_state = (audio_state: AudioState) => {
  logger.info("set audio state", audio_state);
  const $state = get(player_state);
  if ($state.type === "closed") return;
  else if ($state.type === "station") player_state.set({ ...$state, audio_state });
  else if ($state.type === "track") player_state.set({ ...$state, audio_state });
  else assert_never($state);
}

const get_audio_tag = (src: string): HTMLAudioElement => {
  if (audio == null) {

    audio = new Audio(src);

    let start = Date.now();

    const _play = audio.play;
    audio.play = () => {
      start = Date.now();
      return _play.call(audio)
    }

    set_audio_state("loading");

    audio.onpause = () => {
      logger.info("onpause");
      set_audio_state("paused");
    }

    audio.onerror = () => {
      logger.info("onerror")
      set_audio_state("paused");
    }

    audio.onseeking = () => {
      logger.info("onseeking")
      set_audio_state("loading");
    }

    audio.onplay = () => {
      logger.info("onplay")
      set_audio_state("loading");
    }

    audio.onplaying = () => {
      logger.info("onplaying")
      set_audio_state("playing");
    }

    audio.onended = () => {
      const $player_state = get(player_state);
      if ($player_state.type === "station") {
        const src = audio?.src;
        if (src != null) {
          if (Date.now() - start > 5000) {
            destroy_audio_tag();
            const audio = get_audio_tag(src);
            audio.play();
          }
        }
      }
    }

    return audio

  } else {

    audio.src = src;

    return audio;

  }
}

const assert_never = (v: never): never => { throw new Error("assert never called with value:", v) }
