import { load_call, client } from "$lib/load";

export const load = (async ({ fetch }) => {
  const stream_connections = await load_call(
    () => client.GET("/stream-connections", { 
      params: {
        query: {
          show: "open",
          limit: 100_000,
          sort: "creation-desc"  
        }
      },
      fetch
    })
  )

  return {
    stream_connections
  }

}) satisfies import("./$types").PageServerLoad;