import type { Data } from "./types";

export const load = async () => {
  
  const storage_avail = 2_000_000_000;
  const storage_used = Math.round(Math.random() * storage_avail);

  const listeners_avail = 1_000;
  const listeners_used = Math.round(Math.random() * listeners_avail);

  const bandwidth_avail = 5_000_000_000_000;
  const bandwidth_used = Math.round(Math.random() * bandwidth_avail);

  const data: Data = {
    storage: { avail: storage_avail, used: storage_used },
    listeners: { avail: listeners_avail, used: listeners_used },
    bandwidth: { avail: bandwidth_avail, used: bandwidth_used },
    on_air: Math.random() >= 0.5,
    live_streaming: Math.random() >= 0.5,
    ice: {
      host: "lemos.openstream.fm",
      port: 80,
      mount: "aikasdn1/source",
      user: "source",
      password: "aklkasd012d091j",
      encoding: "MP3 / AAC",
    }
  }

  return data;
}