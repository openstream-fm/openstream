import StatusCode from "http-status-codes";
import qs from "qs";
import type { Readable } from "stream";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "./defs/constants.js";
import { ClientError, type ClientErrorCode } from "./error.js";
// import type { Logger } from "./logger.js";
import node_fetch, { Headers } from "node-fetch";
import type { Response, RequestInit } from "node-fetch";

import http from "http";
import https from "https";

import type { PublicErrorCode } from "./defs/error/PublicErrorCode.js";

export { ClientError, type ClientErrorCode };

const qss = (v: any) => {
  return qs.stringify(v, { addQueryPrefix: true, skipNulls: true })
}

export class Client {

  private base_url: string;
  private node_fetch: typeof node_fetch;

  // logger: Logger;

  me: Me;
  auth: Auth;
  plans: Plans;
  admins: Admins;
  users: Users;
  accounts: Accounts;
  stations: Stations;
  analytics: Analytics;
  app_analytics: AppAnalytics;
  invitations: AccountInvitations;
  payment_methods: PaymentMethods;
  stream_connections: StreamConnections;
  stream_connections_lite: StreamConnectionsLite;

  constructor(base_url: string, { /*ogger,*/ fetch = node_fetch }: { /*logger: Logger,*/ fetch?: typeof node_fetch } = {}) {
    this.base_url = base_url.trim().replace(/\/+$/g, "")
    // this.logger = logger.scoped("client");

    this.node_fetch = fetch;

    this.me = new Me(this);
    this.auth = new Auth(this);
    this.plans = new Plans(this);
    this.admins = new Admins(this);
    this.users = new Users(this);
    this.accounts = new Accounts(this);
    this.stations = new Stations(this);
    this.analytics = new Analytics(this);
    this.app_analytics = new AppAnalytics(this);
    this.invitations = new AccountInvitations(this);
    this.payment_methods = new PaymentMethods(this);
    this.stream_connections = new StreamConnections(this);
    this.stream_connections_lite = new StreamConnectionsLite(this);
  }

  async fetch(_url: string, init: RequestInit = {}): Promise<Response> {
    const url = `${this.base_url}${_url}`;
    const method = init.method ?? "GET";
    // this.logger.debug(`fetch: ${method} ${url}`);
    return await this.node_fetch(url, {
      agent: url => url.protocol === "http:" ? http.globalAgent : https.globalAgent,
      ...init
    }).catch(e => {
      // this.logger.warn(`fetch error: ${e} | cause=${e.cause}`)
      console.warn("client fetch failed", e, e.cause ?? null);
      throw new ClientError(StatusCode.BAD_GATEWAY, "CLIENT_GATEWAY_FETCH", "Gateway unavailable")
    })
  }

  async get_json_body<T>(res: Response): Promise<T> {

    const body: any = await res.json().catch(e => {
      // this.logger.warn(`json error: ${e} cause=${e.cause}`)
      throw new ClientError(StatusCode.BAD_GATEWAY, "CLIENT_GATEWAY_JSON", "Gateway error");
    })

    if (body?.error) {
      let message = typeof body.error.message ? body.error.message : "Internal server error";
      let code: ClientErrorCode = 
        typeof body.error?.code === "string" ?
        (body.error.code as PublicErrorCode) :
        "CLIENT_GATEWAY_MISSING_CODE";
        
      throw new ClientError(res.status, code, message);
    }

    return body as T;
  }

  json_headers({
    ip,
    ua,
    token,
    wpayload
  }: {
    ip: string | null,
    ua: string | null,
    token: string | null
    wpayload: boolean,
  }): Headers {

    const headers = new Headers();

    if (ip) headers.append(FORWARD_IP_HEADER, ip);

    // remove default user agent
    headers.append("user-agent", ua || "openstream-unknown")

    if (token) headers.append(ACCESS_TOKEN_HEADER, token);

    if (wpayload) headers.append("content-type", "application/json");


    return headers
  }

  async json_request<T>(url: string, init: RequestInit): Promise<T> {
    const res = await this.fetch(url, init);
    const body = await this.get_json_body<T>(res);
    return body;
  }

  async get<T>(ip: string | null, ua: string | null, token: string | null, url: string,): Promise<T> {
    return await this.json_request<T>(url, {
      headers: this.json_headers({ ip, ua, token, wpayload: false }),
    });
  }

  async delete<T>(ip: string | null, ua: string | null, token: string | null, url: string): Promise<T> {
    return await this.json_request<T>(url, {
      method: "DELETE",
      headers: this.json_headers({ ip, ua, token, wpayload: false }),
    });
  }

  async post<T>(ip: string | null, ua: string | null, token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "POST",
      headers: this.json_headers({ ip, ua, token, wpayload: true }),
      body: JSON.stringify(payload)
    })
  }

  async put<T>(ip: string | null, ua: string | null, token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "PUT",
      headers: this.json_headers({ ip, ua, token, wpayload: true }),
      body: JSON.stringify(payload)
    })
  }

  async patch<T>(ip: string | null, ua: string | null, token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "PATCH",
      headers: this.json_headers({ ip, ua, token, wpayload: true }),
      body: JSON.stringify(payload)
    })
  }

  async get_stream_stats(ip: string | null, ua: string | null, token: string): Promise<import("./defs/api/stream-stats/GET/Output.js").Output> {
    return await this.get(ip, ua, token, `/stream-stats`);
  }

  async get_stream_stats_item_now(ip: string | null, ua: string | null, token: string): Promise<import("./defs/api/stream-stats/now/GET/Output.js").Output> {
    return await this.get(ip, ua, token, `/stream-stats/now`);
  }

  async get_stream_stats_item_now_count(ip: string | null, ua: string | null, token: string): Promise<import("./defs/api/stream-stats/now/count/GET/Output.js").Output> {
    return await this.get(ip, ua, token, `/stream-stats/now/count`);
  }

  async get_stream_stats_now_count_by_station(ip: string | null, ua: string | null, token: string): Promise<import("./defs/api/stream-stats/now/count-by-station/GET/Output.js").Output> {
    return await this.get(ip, ua, token, `/stream-stats/now/count-by-station`);
  }

  async get_stream_stats_item_since(ip: string | null, ua: string | null, token: string, num: number | string, unit: string): Promise<import("./defs/api/stream-stats/[last-unitvalue]/GET/Output.js").Output> {
    return await this.get(ip, ua, token, `/stream-stats/last-${num}${unit}`);
  }

  async get_stream_stats_item_since_count(ip: string | null, ua: string | null, token: string, num: number | string, unit: string): Promise<import("./defs/api/stream-stats/[last-unitvalue]/count/GET/Output.js").Output> {
    return await this.get(ip, ua, token, `/stream-stats/last-${num}${unit}/count`);
  }
}

export class Me {
  client: Client;
  devices: MeDevices;
  api_keys: MeApiKeys;

  constructor(client: Client) {
    this.client = client;
    this.devices = new MeDevices(client);
    this.api_keys = new MeApiKeys(client);
  }

  async me(ip: string | null, ua: string | null, token: string): Promise<import("./defs/api/me/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/me`);
  }
}

export class MeDevices {
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }

  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/me/devices/GET/Query.js").Query): Promise<import("./defs/api/me/devices/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/me/devices${qss(query)}`);
  }

  async delete(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/me/devices/[device]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/me/devices/${id}`);
  }
}

export class MeApiKeys {
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }

  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/me/api-keys/GET/Query.js").Query): Promise<import("./defs/api/me/devices/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/me/api-keys${qss(query)}`);
  }

  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/me/api-keys/POST/Payload.js").Payload): Promise<import("./defs/api/me/api-keys/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/me/api-keys`, payload);
  }
  
  async patch(ip: string | null, ua: string | null, token: string, id: string, payload: import("./defs/api/me/api-keys/[id]/PATCH/Payload.js").Payload): Promise<import("./defs/api/me/api-keys/[id]/PATCH/Output.js").Output> {
    return await this.client.patch(ip, ua, token, `/me/api-keys/${id}`, payload);
  }
    
  async delete(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/me/api-keys/[id]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/me/api-keys/${id}`);
  }
}


export class Auth {
  client: Client;
  user: AuthUser;
  admin: AuthAdmin;
  constructor(client: Client) {
    this.client = client;
    this.user = new AuthUser(client);
    this.admin = new AuthAdmin(client);
  }

  async send_email_verification_code(ip: string | null, ua: string | null, payload: import("./defs/api/auth/email-verification/send-code/POST/Payload.js").Payload): Promise<import("./defs/api/auth/email-verification/send-code/POST/Output.js").Output> {
    return await this.client.post(ip, ua, null, `/auth/email-verification/send-code`, payload)
  }
}

export class AuthAdmin {
  client: Client;
  constructor(client: Client) {
    this.client = client
  }

  async login(ip: string | null, ua: string | null, token: string | null, payload: import("./defs/api/auth/admin/login/POST/Payload.js").Payload): Promise<import("./defs/api/auth/admin/login/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, "/auth/admin/login", payload)
  }

  async logout(ip: string | null, ua: string | null, token: string): Promise<import("./defs/api/auth/admin/logout/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, "/auth/admin/logout", void 0);
  }

  async delegate(ip: string | null, ua: string | null, token: string, user_id: string, payload: import("./defs/api/auth/admin/delegate/[user]/POST/Payload.js").Payload): Promise<import("./defs/api/auth/admin/delegate/[user]/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/auth/admin/delegate/${user_id}`, payload);
  }
}

export class AuthUser {
  client: Client;
  recovery_token: AuthUserRecoveryToken;

  constructor(client: Client) {
    this.client = client;
    this.recovery_token = new AuthUserRecoveryToken(this.client);
  }

  async email_exists(ip: string | null, ua: string | null, token: string | null, email: string): Promise<import("./defs/api/auth/user/email-exists/[email]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/auth/user/email-exists/${email}`);
  }

  async login(ip: string | null, ua: string | null, token: string | null, payload: import("./defs/api/auth/user/login/POST/Payload.js").Payload): Promise<import("./defs/api/auth/user/login/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, "/auth/user/login", payload)
  }

  async logout(ip: string | null, ua: string | null, token: string): Promise<import("./defs/api/auth/user/logout/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, "/auth/user/logout", void 0);
  }

  async register(ip: string | null, ua: string | null, token: string | null, payload: import("./defs/api/auth/user/register/POST/Payload.js").Payload): Promise<import("./defs/api/auth/user/register/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, "/auth/user/register", payload)
  }

  async recover(ip: string | null, ua: string | null, token: string | null, payload: import("./defs/api/auth/user/recover/POST/Payload.js").Payload): Promise<import("./defs/api/auth/user/recover/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, "/auth/user/recover", payload)
  }
}

export class AuthUserRecoveryToken {
  
  client: Client;
  
  constructor(client: Client) {
    this.client = client;
  }

  async get(ip: string | null, ua: string | null, token: string | null, key: string): Promise<import("./defs/api/auth/user/recovery-token/[token]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/auth/user/recovery-token/${key}`)
  }

  async set_password(ip: string | null, ua: string | null, token: string | null, key: string, payload: import("./defs/api/auth/user/recovery-token/[token]/set-password/POST/Payload.js").Payload): Promise<import("./defs/api/auth/user/recovery-token/[token]/set-password/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/auth/user/recovery-token/${key}/set-password`, payload)
  }
}

export class Plans {
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }

  async list(ip: string | null, ua: string | null, token: string | null, query: import("./defs/api/plans/GET/Query.js").Query): Promise<import("./defs/api/plans/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/plans${qss(query)}`);
  }

  async get(ip: string | null, ua: string | null, token: string | null, plan_id: string): Promise<import("./defs/api/plans/[plan]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/plans/${plan_id}`);
  }

  async get_by_slug(ip: string | null, ua: string | null, token: string | null, plan_slug: string): Promise<import("./defs/api/plans/by-slug/[slug]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/plans/by-slug/${plan_slug}`);
  }

  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/plans/POST/Payload.js").Payload): Promise<import("./defs/api/plans/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/plans`, payload);
  }

  async patch(ip: string | null, ua: string | null, token: string, plan_id: string, payload: import("./defs/api/plans/[plan]/PATCH/Payload.js").Payload): Promise<import("./defs/api/plans/[plan]/PATCH/Output.js").Output> {
    return await this.client.patch(ip, ua, token, `/plans/${plan_id}`, payload);
  }

  async delete(ip: string | null, ua: string | null, token: string, plan_id: string): Promise<import("./defs/api/plans/[plan]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/plans/${plan_id}`);
  }
}

export class Accounts {
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }

  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/accounts/GET/Query.js").Query): Promise<import("./defs/api/accounts/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts${qss(query)}`);
  }

  async get(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/accounts/[account]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${id}`);
  }

  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/accounts/POST/Payload.js").Payload): Promise<import("./defs/api/accounts/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/accounts`, payload);
  }

  async patch(ip: string | null, ua: string | null, token: string, account_id: string, payload: import("./defs/api/accounts/[account]/PATCH/Payload.js").Payload): Promise<import("./defs/api/accounts/[account]/PATCH/Output.js").Output> {
    return await this.client.patch(ip, ua, token, `/accounts/${account_id}`, payload);
  }

  async delete(ip: string | null, ua: string | null, token: string, account_id: string): Promise<import("./defs/api/accounts/[account]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/accounts/${account_id}`);
  }

  async list_members(ip: string | null, ua: string | null, token: string, account_id: string): Promise<import("./defs/api/accounts/[account]/members/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${account_id}/members`)
  }

  async delete_member(ip: string | null, ua: string | null, token: string, account_id: string, member_id: string): Promise<import("./defs/api/accounts/[account]/members/[member]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/accounts/${account_id}/members/${member_id}`)
  }

  async set_member_role(ip: string | null, ua: string | null, token: string, account_id: string, member_id: string, payload: import("./defs/api/accounts/[account]/members/[member]/set-role/POST/Payload.js").Payload): Promise<import("./defs/api/accounts/[account]/members/[member]/set-role/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/accounts/${account_id}/members/${member_id}/set-role`, payload);
  }

  async get_stream_stats(ip: string | null, ua: string | null, token: string, account_id: string): Promise<import("./defs/api/accounts/[account]/stream-stats/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${account_id}/stream-stats`);
  }

  async get_stream_stats_item_now(ip: string | null, ua: string | null, token: string, account_id: string): Promise<import("./defs/api/accounts/[account]/stream-stats/now/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${account_id}/stream-stats/now`);
  }

  async get_stream_stats_item_now_count(ip: string | null, ua: string | null, token: string, account_id: string): Promise<import("./defs/api/accounts/[account]/stream-stats/now/count/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${account_id}/stream-stats/now/count`);
  }

  async get_stream_stats_now_count_by_station(ip: string | null, ua: string | null, token: string, account_id: string): Promise<import("./defs/api/accounts/[account]/stream-stats/now/count-by-station/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${account_id}/stream-stats/now/count-by-station`);
  }

  async get_stream_stats_item_since(ip: string | null, ua: string | null, token: string, account_id: string, num: number | string, unit: string): Promise<import("./defs/api/accounts/[account]/stream-stats/[last-unitvalue]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${account_id}/stream-stats/last-${num}${unit}`);
  }

  async get_stream_stats_item_since_count(ip: string | null, ua: string | null, token: string, account_id: string, num: number | string, unit: string): Promise<import("./defs/api/accounts/[account]/stream-stats/[last-unitvalue]/count/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/accounts/${account_id}/stream-stats/last-${num}${unit}/count`);
  }
}

export class Stations {

  client: Client;

  files: StationFiles;

  pictures: StationPictures;

  constructor(client: Client) {
    this.client = client;
    this.files = new StationFiles(client);
    this.pictures = new StationPictures(client);
  }

  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/stations/GET/Query.js").Query): Promise<import("./defs/api/stations/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations${qss(query)}`);
  }

  async get(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${id}`);
  }

  async delete(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/stations/${id}`);
  }

  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/stations/POST/Payload.js").Payload): Promise<import("./defs/api/stations/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations`, payload);
  }

  async is_slug_available(ip: string | null, ua: string | null, token: string | null, query: import("./defs/api/stations/is-slug-available/GET/Query.js").Query): Promise<import("./defs/api/stations/is-slug-available/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/is-slug-available${qss(query)}`);
  }

  async transfer(ip: string | null, ua: string | null, token: string, id: string, payload: import("./defs/api/stations/[station]/transfer/POST/Payload.js").Payload): Promise<import("./defs/api/stations/[station]/transfer/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${id}/transfer`, payload);
  }

  async patch(ip: string | null, ua: string | null, token: string, id: string, payload: import("./defs/api/stations/[station]/PATCH/Payload.js").Payload): Promise<import("./defs/api/stations/[station]/PATCH/Output.js").Output> {
    return await this.client.patch(ip, ua, token, `/stations/${id}`, payload);
  }

  async get_stream_stats(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/stream-stats/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${id}/stream-stats`);
  }

  async get_stream_stats_item_now(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/stream-stats/now/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${id}/stream-stats/now`);
  }

  async get_stream_stats_item_now_count(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/stream-stats/now/count/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${id}/stream-stats/now/count`);
  }

  async get_stream_stats_item_since(ip: string | null, ua: string | null, token: string, id: string, num: number | string, unit: string): Promise<import("./defs/api/stations/[station]/stream-stats/[last-unitvalue]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${id}/stream-stats/last-${num}${unit}`);
  }

  async get_stream_stats_item_since_count(ip: string | null, ua: string | null, token: string, id: string, num: number | string, unit: string): Promise<import("./defs/api/stations/[station]/stream-stats/[last-unitvalue]/count/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${id}/stream-stats/last-${num}${unit}/count`);
  }

  async get_now_playing(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/now-playing/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${id}/now-playing`);
  }

  // async get_dashboard_stats(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/dashboard-stats/GET/Output.js").Output> {
  //   return await this.client.get(ip, ua, token, `/stations/${id}/dashboard-stats`);
  // }

  async restart_playlist(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/restart-playlist/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${id}/restart-playlist`, undefined);
  }

  async reset_source_password(ip: string | null, ua: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/reset-source-password/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${id}/reset-source-password`, undefined);
  }
}

export class StationPictures {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }

  async post(ip: string | null, ua: string | null, token: string, query: import("./defs/api/station-pictures/POST/Query.js").Query, data: Readable | Buffer): Promise<import("./defs/api/station-pictures/POST/Output.js").Output> {
    const headers = new Headers();

    if (ip) headers.append(FORWARD_IP_HEADER, ip)
    if (ua) headers.append("user-agent", ua)
    headers.append(ACCESS_TOKEN_HEADER, token)
    headers.append("content-type", "application/octet-stream")

    let res = await this.client.fetch(`/station-pictures${qss(query)}`, {
      method: "POST",
      headers,
      body: data
    })

    return await this.client.get_json_body(res)
  }
}

export class Admins {
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }

  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/admins/GET/Query.js").Query): Promise<import("./defs/api/admins/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/admins${qss(query)}`);
  }

  async get(ip: string | null, ua: string | null, token: string, admin_id: string): Promise<import("./defs/api/admins/[admin]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/admins/${admin_id}`);
  }

  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/admins/POST/Payload.js").Payload): Promise<import("./defs/api/admins/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/admins`, payload);
  }

  async patch(ip: string | null, ua: string | null, token: string, admin_id: string, payload: import("./defs/api/admins/[admin]/PATCH/Payload.js").Payload): Promise<import("./defs/api/admins/[admin]/PATCH/Output.js").Output> {
    return await this.client.patch(ip, ua, token, `/admins/${admin_id}`, payload);
  }

  async change_password(ip: string | null, ua: string | null, token: string, admin_id: string, payload: import("./defs/api/admins/[admin]/change-password/POST/Payload.js").Payload): Promise<import("./defs/api/admins/[admin]/change-password/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/admins/${admin_id}/change-password`, payload);
  }
}

export class Users {
  client: Client;
  stations: UserStations;

  constructor(client: Client) {
    this.client = client;
    this.stations = new UserStations(client);
  }

  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/users/GET/Query.js").Query): Promise<import("./defs/api/stations/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/users${qss(query)}`);
  }

  async get(ip: string | null, ua: string | null, token: string, user_id: string): Promise<import("./defs/api/users/[user]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/users/${user_id}`);
  }

  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/users/POST/Payload.js").Payload): Promise<import("./defs/api/users/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/users`, payload);
  }

  async patch(ip: string | null, ua: string | null, token: string, user_id: string, payload: import("./defs/api/users/[user]/PATCH/Payload.js").Payload): Promise<import("./defs/api/users/[user]/PATCH/Output.js").Output> {
    return await this.client.patch(ip, ua, token, `/users/${user_id}`, payload);
  }

  async delete(ip: string | null, ua: string | null, token: string, user_id: string): Promise<import("./defs/api/users/[user]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/users/${user_id}`);
  }

  async change_password(ip: string | null, ua: string | null, token: string, user_id: string, payload: import("./defs/api/users/[user]/change-password/POST/Payload.js").Payload): Promise<import("./defs/api/users/[user]/change-password/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/users/${user_id}/change-password`, payload);
  }
}

export class UserStations {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  /*
  async list(token: string, user_id: string, query: import("./defs/api/users/[user]GET/Query.js").Query): Promise<import("./defs/api/stations/GET/Output.js").Output> {
    return await this.client.get(`/users/${user_id}${qss(query)}`, token);
  }
  */
}


export class StationFiles {

  client: Client;

  constructor(client: Client) {
    this.client = client;
  }


  async list(ip: string | null, ua: string | null, token: string, station_id: string, query: import("./defs/api/stations/[station]/files/GET/Query.js").Query): Promise<import("./defs/api/stations/[station]/files/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${station_id}/files${qss(query)}`);
  }

  async get(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stations/${station_id}/files/${file_id}`);
  }

  async delete(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/stations/${station_id}/files/${file_id}`);
  }

  async post(ip: string | null, ua: string | null, token: string, station_id: string, content_type: string, content_length: number, query: import("./defs/api/stations/[station]/files/POST/Query.js").Query, data: Readable | Buffer): Promise<import("./defs/api/stations/[station]/files/POST/Output.js").Output> {

    const headers = new Headers();

    if (ip) headers.append(FORWARD_IP_HEADER, ip)
    if (ua) headers.append("user-agent", ua)
    headers.append(ACCESS_TOKEN_HEADER, token)
    headers.append("content-type", content_type)
    headers.append("content-length", String(content_length))

    let res = await this.client.fetch(`/stations/${station_id}/files${qss(query)}`, {
      method: "POST",
      headers,
      body: data
    })

    return await this.client.get_json_body(res)
  }

  async put_metadata(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/metadata/PUT/Payload.js").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/metadata/PUT/Output.js").Output> {
    return await this.client.put(ip, ua, token, `/stations/${station_id}/files/${file_id}/metadata`, payload);
  }

  async shuffle(ip: string | null, ua: string | null, token: string, station_id: string): Promise<import("./defs/api/stations/[station]/files/shuffle/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${station_id}/files/shuffle`, undefined);
  }

  async unshuffle(ip: string | null, ua: string | null, token: string, station_id: string): Promise<import("./defs/api/stations/[station]/files/unshuffle/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${station_id}/files/unshuffle`, undefined);
  }

  async swap_order(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/order/swap/POST/Payload.js").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/order/swap/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${station_id}/files/${file_id}/order/swap`, payload);
  }

  async move_to_first(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-to-first/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${station_id}/files/${file_id}/order/move-to-first`, undefined);
  }

  async move_to_last(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-to-last/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${station_id}/files/${file_id}/order/move-to-last`, undefined);
  }

  async move_before(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/order/move-before/POST/Payload.js").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-before/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${station_id}/files/${file_id}/order/move-before`, payload)
  }

  async move_after(ip: string | null, ua: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/order/move-after/POST/Payload.js").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-after/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/stations/${station_id}/files/${file_id}/order/move-after`, payload)
  }
}

export class Analytics {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }

  async get(ip: string | null, ua: string | null, token: string, query: import("./defs/api/analytics/GET/Query.js").Query): Promise<import("./defs/api/analytics/GET/Output.js").Output> {
    const url = `/analytics${qss(query)}`;
    return await this.client.get(ip, ua, token, url);
  }
}

export class AppAnalytics {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }

  async get(ip: string | null, ua: string | null, token: string, query: import("./defs/api/app-analytics/GET/Query.js").Query): Promise<import("./defs/api/app-analytics/GET/Output.js").Output> {
    const url = `/app-analytics${qss(query)}`;
    return await this.client.get(ip, ua, token, url);
  }
}

export class PaymentMethods {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  
  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/payment-methods/GET/Query.js").Query): Promise<import("./defs/api/payment-methods/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/payment-methods${qss(query)}`);
  }

  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/payment-methods/POST/Payload.js").Payload): Promise<import("./defs/api/payment-methods/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/payment-methods`, payload);
  }


  async get(ip: string | null, ua: string | null, token: string, payment_method_id: string): Promise<import("./defs/api/payment-methods/[payment-method]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/payment-methods/${payment_method_id}`);
  }
}

export class StreamConnections {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  
  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/stream-connections/GET/Query.js").Query): Promise<import("./defs/api/stream-connections/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stream-connections${qss(query)}`);
  }
}

export class StreamConnectionsLite {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  
  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/stream-connections-lite/GET/Query.js").Query): Promise<import("./defs/api/stream-connections-lite/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/stream-connections-lite${qss(query)}`);
  }
}


export class AccountInvitations {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  
  async list(ip: string | null, ua: string | null, token: string, query: import("./defs/api/invitations/GET/Query.js").Query): Promise<import("./defs/api/invitations/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/invitations${qss(query)}`);
  }
  
  async post(ip: string | null, ua: string | null, token: string, payload: import("./defs/api/invitations/POST/Payload.js").Payload): Promise<import("./defs/api/invitations/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/invitations`, payload);
  }

  async get(ip: string | null, ua: string | null, token: string, invitation_id: string): Promise<import("./defs/api/invitations/[invitation]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/invitations/${invitation_id}`);
  }

  async delete(ip: string | null, ua: string | null, token: string, invitation_id: string): Promise<import("./defs/api/invitations/[invitation]/DELETE/Output.js").Output> {
    return await this.client.delete(ip, ua, token, `/invitations/${invitation_id}`);
  }

  async get_by_token(ip: string | null, ua: string | null, token: string | null, invitation_token: string): Promise<import("./defs/api/invitations/get-by-token/[token]/GET/Output.js").Output> {
    return await this.client.get(ip, ua, token, `/invitations/get-by-token/${invitation_token}`); 
  }

  async accept(ip: string | null, ua: string | null, token: string | null, payload: import("./defs/api/invitations/accept/POST/Payload.js").Payload): Promise<import("./defs/api/invitations/accept/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/invitations/accept`, payload); 
  }

  async reject(ip: string | null, ua: string | null, token: string | null, payload: import("./defs/api/invitations/reject/POST/Payload.js").Payload): Promise<import("./defs/api/invitations/reject/POST/Output.js").Output> {
    return await this.client.post(ip, ua, token, `/invitations/reject`, payload); 
  }
}