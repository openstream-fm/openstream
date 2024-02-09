import { load_call, client } from '$lib/load.js';

export const load = (async ({ depends, fetch, params }) => {
  depends("api:auth/user/recovery-token/:token");
  depends("resource:user-recovery-tokens");
  const result = await load_call(() => client.GET("/auth/user/recovery-token/{token}", { params: { path: { token: params.token } }, fetch }));
  return { 
    result,
    token: params.token,
  };
}) satisfies import("./$types").PageServerLoad;