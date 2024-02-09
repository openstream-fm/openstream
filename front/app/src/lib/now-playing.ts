import { browser } from "$app/environment";
import { type Readable, type Writable, readable, writable } from "svelte/store";
import { default_logger } from "$share/logger";
import { sleep } from "$share/util";
import { GET, unwrap } from "./client";
import type { Unwrap } from "./client";

// export type NowPlaying = import("$api/stations/[station]/now-playing/GET/Output").Output;
export type NowPlaying = Unwrap<Awaited<ReturnType<typeof GET<"/stations/{station}/now-playing">>>>;
export type StoreValue = { station_id: string, info: NowPlaying };

export const NOW_PLAYING_INTERNVAL = 3000;

const map = new Map<string, Writable<StoreValue | null>>();

const logger = default_logger.scoped("now-playing");

export const get_now_playing_store = (station_id: string, default_info: NowPlaying | null = null): Readable<StoreValue | null> => {
  
  const start_value = default_info ? { station_id, info: default_info } : null;

  if(!browser) return readable(start_value);
  const existent = map.get(station_id);
  if(existent != null) return { subscribe: existent.subscribe };
  
  const store = writable<StoreValue | null>(start_value);
  
  let count = 0;
  let stopped = false;
  
  const start = async () => {
    
    logger.info(`start ${station_id} => ${map.size} (${[...map.keys()].join(",")}) current subs`)
    let _prev_skip = false;
    let last = 0;

    while(true) {
      await sleep(100);
      if(stopped) break;
      const skip = document.visibilityState === "hidden";
      const prev_skip = _prev_skip;
      _prev_skip = skip;
     
      if(skip) {
        if(prev_skip !== skip) {
          logger.info(`pausing update for station ${station_id}, (document: ${document.visibilityState})`);
        }
      } else {
        if(prev_skip !== skip) {
          logger.info(`(re)starting update for station ${station_id}, (document: ${document.visibilityState})`);
        }

        if(Date.now() - last < NOW_PLAYING_INTERNVAL) continue;
        try {
          const info = unwrap(await GET("/stations/{station}/now-playing", {
            params: {
              path: { station: station_id }
            }
          }));
          logger.info(`info updated for ${station_id}`)
          store.set({ station_id, info });
        } catch(e) {
          logger.warn(`error obtaining now playing info: ${e}`);
        } finally {
          last = Date.now();
        }
      }
    }
  }

  const stop = () => {
    stopped = true;
    map.delete(station_id);
    logger.info(`stop ${station_id} => ${map.size} (${[...map.keys()].join(",")}) current subs`)
  }

  const subscribe = store.subscribe;
  store.subscribe = (run, invalidate) => {
    count += 1;
    if(count === 1) start();
    const unsub = subscribe(run, invalidate); 
    return () => {
      count -= 1;
      if(count <= 0) stop();
      unsub();
    }
  }

  map.set(station_id, store);

  return { subscribe: store.subscribe }
}