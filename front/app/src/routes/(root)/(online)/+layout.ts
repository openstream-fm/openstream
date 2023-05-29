import { load_get, load_get_me } from "$lib/load";

export const load = (async ({ depends, fetch, url }) => {
  depends("resource:users");
  depends("api:users/me");
  
  const maybe_user = load_get_me({ fetch, url });
  
  return { maybe_user };

}) satisfies import("./$types").LayoutLoad;