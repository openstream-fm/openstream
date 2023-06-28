import { load_get } from '$lib/load.js';

export const load = (async ({ depends, url, fetch, params }) => {
  depends("api:auth/user/recovery-token/:token");
  depends("resource:user-recovery-tokens");
  const result = await load_get<import("$api/auth/user/recovery-token/[token]/GET/Output").Output>(`/api/auth/user/recovery-token/${params.token}`, { url, fetch });
  return { 
    result,
    token: params.token,
  };
}) satisfies import("./$types").PageServerLoad;