import { redirect } from "@sveltejs/kit";

export const load = (async ({ parent }) => {

  const { accounts } = await parent();
  
  if(accounts.total === 1) {
    const _id = accounts.items?.[0]._id;
    if(_id) throw redirect(302, `/accounts/${_id}`)
  }

  return {};

}) satisfies import("./$types").LayoutLoad;