import { load_get } from "$lib/load";
import { error, redirect } from "@sveltejs/kit";

export const load = (async ({ fetch, url, parent, depends, params }) => {

  depends("api:stations/:id");

  let current_page: null | "dashboard" | "profile" | "playlist" | "broadcast" | "relay" | "settings" = null;
  
  const { stations, account } = await parent();
  const station = stations.items.find(item => item._id === params.station);
  
  if(station == null) {
    throw error(404, {
      status: 404,
      code: "CLIENT_STATION_NOT_FOUND",
      message: `Station with id ${params.station} does not exists or has been deleted`,
    })
  }

  if(station.account_id !== account._id) {
    throw error(404, {
      status: 404,
      code: "CLIENT_STATION_ACCOUNT_MISMATCH",
      message: `Station with id ${station._id} doesn't belong to this account`,
    })
  }

  return { station, current_page }

}) satisfies import("./$types").LayoutServerLoad;