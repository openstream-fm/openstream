import { load_get } from "$lib/load";
import { qss } from "$share/qs";

export const load = (async ({ fetch, url }) => {
  const params: import("$api/stream-connections/GET/Query").Query = { 
    show: "open",
    limit: 100_000,
    sort: "creation-desc",
  }
  
  let stream_connections = await load_get<import("$api/stream-connections/GET/Output").Output>(
    `/api/stream-connections${qss(params)}`,
    { fetch, url }
  );
  
  return {
    stream_connections
  }

}) satisfies import("./$types").PageLoad;