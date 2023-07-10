import { load_get } from "$lib/load";

export const load = (async ({ fetch, url }) => {
  const station_id = url.searchParams.get("station") || null;
  const deployment_id = url.searchParams.get("deployment") || null;
  const referer = url.searchParams.get("referer") || null;

  const params = new URLSearchParams();
  params.set("show", "open");
  params.set("limit", String(100_000));
  params.set("sort", "creation-desc");
  if(station_id) params.set("stations[]", station_id);

  let stream_connections = await load_get<import("$api/stream-connections/GET/Output").Output>(
    `/api/stream-connections?${params}`,
    // `/api/stream-connections?show=${"all"}&limit=${60}&sort=${"creation-desc"}${station_id ? `&stations[]=${station_id}` : ""}`,
    { fetch, url}
  );

  if(deployment_id != null) {
    const items = stream_connections.items.filter(item => item.deployment_id === deployment_id);
    stream_connections = {
      skip: stream_connections.skip,
      limit: stream_connections.limit,
      total: items.length,
      items,
    }
  }

  if(referer != null) {
    let items = stream_connections.items;
    if(referer === "null") {
      items = items.filter(item => item.request.headers.referer == null && item.request.headers.origin == null);
    } else {
      const r = `//${referer}`;
      items = items.filter(item => (item.request.headers.referer || item.request.headers.origin || "").includes(r))
    }
  
    stream_connections = {
      skip: stream_connections.skip,
      limit: stream_connections.limit,
      total: items.length,
      items,
    }
  }

  return {
    stream_connections,
    stream_connections_query: {
      station_id,
      deployment_id,
      referer
    }
  }

}) satisfies import("./$types").PageLoad;