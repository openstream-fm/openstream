import { load_get } from "$lib/load";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params }) => {
  
  const { stations, accounts } = await parent();

  const station = stations.items.find(item => item._id === params.station);
  
  if(station == null) throw error(404, { status: 404, message: "Station not found", code: "FRONT_RESOURCE_NOT_FOUND" })

  const account = accounts.items.find(account => station.account_id === account._id);

  return { account, station }

}) satisfies import("./$types").PageLoad;
