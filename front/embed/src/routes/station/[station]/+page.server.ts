import { error } from "@sveltejs/kit";

export const load = (async ({ params, fetch }) => {
  const res = await fetch(`/station/${params.station}/data.json`)
    .catch(e => {
      error(500, "Internal error")
    })

  if(!res.ok) {
    const body = await res.json().catch(e => {
      error(500, "Internal error")
    })

    if(body.error == null) {
      error(500, "Internal error")
    }
    
    error(Number(body.error?.status ?? 500) || 500, String(body.error?.message ?? "Error"))
  }

  const station: import("../../../../../../defs/api/embed/station/[station]/GET/Output.js").Output = await res.json()
    .catch(e => {
      error(500, "Internal error")
    })

  return {
    station
  }
}) satisfies import("./$types").PageServerLoad;