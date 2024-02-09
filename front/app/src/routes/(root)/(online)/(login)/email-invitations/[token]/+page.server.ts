import { load_get } from '$lib/load.js';
import { redirect } from '@sveltejs/kit';

export const load = (async ({ depends, url, fetch, params, parent }) => {
  
  depends("resource:invitations");
  depends("api:invitations");

  const { token } = params;
  const result = await load_get<import("$api/invitations/get-by-token/[token]/GET/Output").Output>(`/api/invitations/get-by-token/${token}`, { fetch, url }, { redirectToLoginOnAuthErrors: false });  
  const { maybe_user } = await parent();

  // if the user receiver of the invitation is already logged in
  // we send him directly to the logged-in version of this page
  if(result.kind === "ok") {
    if(result.invitation.receiver && maybe_user) {
      if(result.invitation.receiver._id === maybe_user._id) {
        redirect(302, "/me/invitations");
      }
    }
  }

  return { result, token }

}) satisfies import("./$types").PageServerLoad;