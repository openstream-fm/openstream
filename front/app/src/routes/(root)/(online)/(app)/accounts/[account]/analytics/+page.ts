// import { load_get } from "$lib/load";

// export const load = (async ({ fetch, url, depends, parent }) => {
  
//   depends("resource:analytics")
//   depends("api:analytics");

//   const { account, stations } = await parent();
//   const account_stations = stations.items.filter(item => item.account_id === account._id);

//   const since = new Date(account.created_at);
//   const until = new Date();

//   const query = new URLSearchParams();
//   query.append("since", since.toJSON());
//   query.append("until", until.toJSON());
  
//   for(const station of account_stations) {
//     query.append("stations[]", station._id);
//   }
  
//   //const { analytics } = await load_get<import("$api/analytics/GET/Output").Output>(`/api/analytics?${query}`, { fetch, url });
  
//   return { 
//     analytics
//   }

// }) satisfies import("./$types").PageLoad;