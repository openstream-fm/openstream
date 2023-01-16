import { browser } from "$app/environment";
import { _get } from "$share/net.client";
import { derived, get, writable } from "svelte/store";

export type PlayerState = PlayerState.Closed | PlayerState.Station | PlayerState.AudioFile;

export type AudioState = "playing" | "loading" | "paused";

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
      name: string
    }
    now_playing: import("$server/defs/api/accounts/[account]/now-playing/GET/Output").Output | null;
  }

  export interface AudioFile extends Base {
    type: "track"
    audio_state: AudioState,
    file: import("$server/defs/db/AudioFile").AudioFile
  }
}

let audio: HTMLAudioElement | null = null;
let now_playing_timer: any = null;

export const pause = () => {
  // TODO: why onpause not called with station audio type
  console.log("[player] pause()");
  audio?.pause();
  set_audio_state("paused");
  const $state = get(player_state);
  if($state.type === "closed") {}
  else if($state.type === "track") {}
  else if($state.type === "station") {
    console.log("[player] destroy tag");
    destroy_audio_tag(); 
  }
  else assert_never($state);
}

export const resume = () => {
  const $state = get(player_state);
  if($state.type === "closed") console.warn("[player] resume called with player_state.type === 'closed'");
  else if($state.type === "track") audio?.play();
  else if($state.type === "station") {
    if($state.audio_state === "paused") play_station($state.station);
  } else assert_never($state);
}

const player_state = writable<PlayerState>({ type: "closed" });
const readable_player_state = { subscribe: player_state.subscribe };
export { readable_player_state as player_state };

export const player_title = derived(player_state, (state): string => {
  if(state.type === "closed") return "";
  else if(state.type === "track") return state.file.metadata.title || state.file.filename;
  else if(state.type === "station") return state.station.name;
  else return assert_never(state);
})

export const player_subtitle = derived(player_state, (state): string | null => {
  if(state.type === "closed") return null;
  else if(state.type === "track") return state.file.metadata.artist;
  else if(state.type === "station") {
    if(state.now_playing == null) return null;
    else if(state.now_playing.kind === "none") return null;
    else if(state.now_playing.kind === "live") return "Live streaming";
    else if(state.now_playing.kind === "playlist") {
      const artist = state.now_playing.file.metadata.artist;
      const title = state.now_playing.file.metadata.title || state.now_playing.file.filename;
      if(artist) {
        return `${title} - ${artist}`
      } else {
        return title;
      }
    }
    else return assert_never(state.now_playing)
  }
  else return assert_never(state)
})

export const player_playing_audio_file_id = derived(player_state, (state): string | null => {
  const $state = get(player_state);
  if($state.type === "track") return $state.file._id;
  else return null;
})

export const player_playing_station_id = derived(player_state, (state): string | null => {
  const $state = get(player_state);
  if($state.type === "station") return $state.station._id;
  else return null;
})

export const player_audio_state = derived(player_state, (state): AudioState => {
  if(state.type === "closed") return "paused";
  else if(state.type === "station") return state.audio_state;
  else if(state.type === "track") return state.audio_state;
  else return assert_never(state);
})

export const play_station = (station: { _id: string, name: string }) => {
  if(!browser) throw new Error("player.play_station called in ssr context");
  destroy_audio_tag();
  player_state.set({
    type: "station",
    now_playing: null,
    audio_state: "loading",
    station,
  })
  // TODO: stream url
  const audio = get_audio_tag(`https://stream.local.openstream.fm/stream/${station._id}`)
  audio.play().catch(e => {
    console.warn(`[player] error playing station ${station._id}`, e)
  })
  start_now_playing_updater();
}

export const play_track = (file: import("$server/defs/db/AudioFile").AudioFile) => {
  if(!browser) throw new Error("player.play_track called in ssr context");
  destroy_audio_tag();
  stop_now_playing_updater();
  player_state.set({
    type: "track",
    file,
    audio_state: "loading",
  })
  const audio = get_audio_tag(`/api/accounts/${file.account_id}/files/${file._id}/stream`);
  audio.play().catch(e => {
    console.warn(`[player] error playing audio track ${file._id}`, e);
  })
}

export const close = () => {
  destroy_audio_tag()
  stop_now_playing_updater();
  player_state.set({ type: "closed" });
}

const destroy_audio_tag = () => {
  if(audio != null) {
    audio.pause();
    audio.src = "data:audio/wav;base64,UklGRjIAAABXQVZFZm10IBIAAAABAAEAQB8AAEAfAAABAAgAAABmYWN0BAAAAAAAAABkYXRhAAAAAA==";
  }
}

const stop_now_playing_updater = () => {
  if(now_playing_timer) clearTimeout(now_playing_timer);
  now_playing_timer = null;
}

const NOW_PLAYING_INTERVAL = 3_000;

const start_now_playing_updater = () => {
  const fn = async () => {
    try {
      const $station_id = get(player_playing_station_id);
      if($station_id == null) return;
      const now_playing: import("$server/defs/api/accounts/[account]/now-playing/GET/Output").Output = await _get(`/api/accounts/${$station_id}/now-playing`);
      const $state = get(player_state);
      if($state.type !== "station") return;
      if($state.station._id !== $station_id) return;
      player_state.set({
        ...$state,
        now_playing,
      })
    } catch(e) {
      console.warn("[player]: error getting now playing info", e)
    }

    setTimeout(fn, NOW_PLAYING_INTERVAL);
  }

  fn();
  now_playing_timer = setTimeout(fn, NOW_PLAYING_INTERVAL);
}

const set_audio_state = (audio_state: AudioState) => {
  console.log("[player] set audio state", audio_state);
  const $state = get(player_state);
  if($state.type === "closed") return;
  else if($state.type === "station") player_state.set({ ...$state, audio_state });
  else if($state.type === "track") player_state.set({ ...$state, audio_state });
  else assert_never($state);
}

const get_audio_tag = (src: string): HTMLAudioElement => {
  if(audio == null) {
    audio = new Audio(src);

    set_audio_state("loading");

    audio.onpause = () => {
      console.log("[player] onpause");
      set_audio_state("paused");
    }

    audio.onerror = () => {
      console.log("[player] onerror")
      set_audio_state("paused");
    }

    audio.onseeking = () => {
      console.log("[player] onsseking")
      set_audio_state("loading");
    }

    audio.onplay = () => {
      console.log("[player] onplay")
      set_audio_state("loading");
    }

    audio.onplaying = () => {
      console.log("[player] onplaying")
      set_audio_state("playing");
    }
    
    return audio
  
  } else {

    audio.src = src;
    
    return audio;
  
  }
}

const assert_never = (v: never): never => { throw new Error("assert never called with value:", v) }
