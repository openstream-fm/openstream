import "./service-worker-env";
import { registerRoute, setCatchHandler, setDefaultHandler } from "workbox-routing";
import { CacheFirst, NetworkOnly } from "workbox-strategies";
import { CacheableResponsePlugin } from 'workbox-cacheable-response';
import { matchPrecache, precacheAndRoute } from "workbox-precaching";
import { build, version } from "$service-worker";

// there's a quirk in sveltekit not working when serving /offline from other url
const offline_urls: string[] = [];
for(let i = 1; i < 8; i++) {
  const url = "/" + Array(i).fill("offline").join("/")
  offline_urls.push(url);
}

addEventListener("install", event => {
  // @ts-ignore
  self.skipWaiting();
})

precacheAndRoute([
  ...offline_urls.map(url => {
    return { url, revision: version }
  }),
  ...build.map(url => {
    return { url, revision: version };
  })
]);

setDefaultHandler(new NetworkOnly());

setCatchHandler(async ({request, url}) => {
  if(url.origin === self.origin && request.destination === "document") {
    const target = "/" + url.pathname.slice(1).split("/").fill("offline").join("/");
    const response = await matchPrecache(target);
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
    cacheName: "station-pictures",
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
        statuses: [0, 200]
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

registerRoute(
  ({ request }) => request.destination === "document",
  new NetworkOnly(),  
)

registerRoute(
  ({ url }) => url.origin === self.origin && (url.pathname === "/api" || url.pathname.startsWith("/api/")),
  new NetworkOnly(),  
)

