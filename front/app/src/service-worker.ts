/// <reference types="@sveltejs/kit" />
/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />

import "./service-worker-env";
import { registerRoute, setCatchHandler, setDefaultHandler } from "workbox-routing";
import { CacheFirst, NetworkOnly } from "workbox-strategies";
import { CacheableResponsePlugin } from 'workbox-cacheable-response';
import { matchPrecache, precacheAndRoute } from "workbox-precaching";
import { build, version } from "$service-worker";

precacheAndRoute([
  { url: "/offline", revision: version },
  ...build.map(url => {
    return { 
      url,
      revision: url.includes("/immutable/") ? "0" : version,
    };
  })
]);

setDefaultHandler(new NetworkOnly());

setCatchHandler(async ({request, url}) => {
  if(url.origin === self.origin && request.destination === "document") {
    const response = await matchPrecache("/offline");
    if(response) return response;
  }

  return Response.error();
})

// build that is not present in files list
registerRoute(
  ({ url }) => url.pathname.startsWith("/_app/immutable/"),
  new CacheFirst({
    cacheName: "build-imgs",
    plugins: [
      new CacheableResponsePlugin({
        statuses: [0, 200]
      })
    ]
  })
)

// station pictures are immutable
registerRoute(
  ({ request, url }) => request.destination === "image" && url.pathname.startsWith(`/station-pictures`),
  new CacheFirst({
    cacheName: `station-pictures`,
    plugins: [
      new CacheableResponsePlugin({
        statuses: [0, 200]
      })
    ]
  })
)

// audio files are immutable and the current media token is part of the url
registerRoute(
  ({ request, url }) => request.destination === "audio" && /^\/stations\/[^\/]+\/files\/[^\/]+\/stream/.test(url.pathname),
  new CacheFirst({
    cacheName: "station-audio-files",
    plugins: [
      new CacheableResponsePlugin({
        statuses: [0, 200],
      })
    ]
  })
)

// fonts are immutable, if we ever change them, we'll change the cacheName here  
registerRoute(
  ({ request, url }) => url.origin === self.origin && request.destination === "font",
  new CacheFirst({
    cacheName: "fonts",
    plugins: [
      new CacheableResponsePlugin({
        statuses: [0, 200]
      })
    ]
  })
)

