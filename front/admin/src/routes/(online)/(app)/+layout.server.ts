import { client, load_call } from "$lib/load";
import { redirect } from "@sveltejs/kit";

const unpage = <T extends { deleted_at?: string | null | undefined }>(page: { items: T[] }): [all: T[], current: T[], deleted: T[]] => {
  const all = page.items;
  const current: T[] = [];
  const deleted: T[] = [];
  for (const item of all) {
    if (item.deleted_at == null) current.push(item);
    else deleted.push(item);
  }

  return [all, current, deleted];
}

export const load = (async ({ fetch, url, parent, depends }) => {

  depends("resource:plans", "resource:admins", "resource:users", "resource:accounts", "resource:stations");
  depends("api:admins", "api:users", "api:accounts", "api:stations");

  const { maybe_admin } = await parent();
  if (maybe_admin == null) {
    const to = `${url.pathname}${url.search}`;
    const login_url = to === "/" ? "/login" : `/login#${to}`
    redirect(302, login_url);
  }

  const params = { query: { show: "all" as const, limit: 100_000 } };

  const [
    admins_page,
    users_page,
    accounts_page,
    stations_page,
    plans_page,
  ] = await Promise.all([
    load_call(() => client.GET("/admins", { params, fetch })),
    load_call(() => client.GET("/users", { params, fetch })),
    load_call(() => client.GET("/accounts", { params, fetch })),
    load_call(() => client.GET("/stations", { params, fetch })),
    load_call(() => client.GET("/plans", { params, fetch })),
  ]);


  const [all_admins, admins, deleted_admins] = unpage(admins_page);
  const [all_users, users, deleted_users] = unpage(users_page);
  const [all_accounts, accounts, deleted_accounts] = unpage(accounts_page);
  const [all_stations, stations, deleted_stations] = unpage(stations_page);
  const [all_plans, plans, deleted_plans] = unpage(plans_page);

  return {
    all_admins, admins, deleted_admins,
    all_users, users, deleted_users,
    all_accounts, accounts, deleted_accounts,
    all_stations, stations, deleted_stations,
    all_plans, plans, deleted_plans,
    admin: maybe_admin,
  }

}) satisfies import("./$types").LayoutServerLoad;