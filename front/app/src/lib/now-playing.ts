import { browser } from "$app/environment";
import { _get } from "$share/net.client";
import { type Readable, type Writable, readable, writable } from "svelte/store";
import { default_logger } from "$share/logger";

export type NowPlaying = import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output;
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
  
  let timer_id: any;

  const start = () => {
    
    logger.info(`start ${station_id} => ${map.size} (${[...map.keys()].join(",")}) current subs`)
    
    const fn = async () => {
      const token = timer_id;
      const skip = document.hidden === true;
      if(skip) {
        logger.info(`skipping tick for station ${station_id} because of document.hidden`);
      } else {
        try {
          const info = await _get<NowPlaying>(`/api/stations/${station_id}/now-playing`);
          logger.info(`info updated for ${station_id}`)
          store.set({ station_id, info });
        } catch(e) {
          logger.warn(`error obtaining now playing info: ${e}`);
        }
      }

      if(token === timer_id) timer_id = setTimeout(fn, skip ? 1000 : NOW_PLAYING_INTERNVAL);
    }

    fn();
  }

  const stop = () => {
    map.delete(station_id);
    clearTimeout(timer_id);
    timer_id = -1;
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