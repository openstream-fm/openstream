import { redirect } from "@sveltejs/kit";

export const load = (async ({ parent }) => {
  const { stations } = await parent();
  if(stations.total === 1) {
    const _id = stations.items[0]?._id;
    if(_id) {
      throw redirect(302, `/stations/${_id}`);
    }
  }

  throw redirect(302, `/stations`);
  
}) satisfies import("./$types").PageLoad;