import type { PageServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ parent, request, getClientAddress }) => {
  const { accounts } = await parent();
  if(accounts.total === 1) {
    const _id = accounts.items[0]?._id;
    if(_id) {
      throw redirect(302, `/accounts/${_id}`);
    }
  }

  throw redirect(302, `/accounts`);
  
}) satisfies PageServerLoad;