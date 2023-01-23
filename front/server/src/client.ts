import StatusCode from "http-status-codes";
import qs from "qs";
import type { ErrorCode } from "./types";
import { Readable } from "stream";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "./contants";
import { ClientError } from "./client-error";
import { Logger } from "./logger";
import fetch, { Response, RequestInit, Headers } from "node-fetch";
import crypto from "crypto";

import http from "http";
import https from "https";

const qss = (v: any) => {
  return qs.stringify(v, { addQueryPrefix: true, skipNulls: true })
}

export class Client {

  private base_url: string;
  logger: Logger;

  stations: Stations;
  users: Users;
  auth: Auth;

  constructor(base_url: string, { logger }: { logger: Logger }) {
    this.base_url = base_url.trim().replace(/\/+$/g, "")
    this.logger = logger.scoped("client");
    
    this.stations = new Stations(this);
    this.users = new Users(this);
    this.auth = new Auth(this);
  }

  async fetch(_url: string, init: RequestInit = {}): Promise<Response> {
    const url = `${this.base_url}${_url}`;
    const method = init.method ?? "GET";
    this.logger.info(`fetch: ${method} ${url}`);
    return await fetch(url, { 
      agent: (url) => url.protocol === "http:" ? http.globalAgent : https.globalAgent,
      ...init
    }).catch(e => {
      this.logger.warn(`fetch error: ${e} | cause=${e.cause}`)
      throw new ClientError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_FETCH", "Gateway unavailable")
    })
  }

  async get_json_body<T>(res: Response): Promise<T> {

    const body: any = await res.json().catch(e => {
      this.logger.warn(`json error: ${e} cause=${e.cause}`)
      throw new ClientError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_JSON", "Gateway error");
    })

    if(body?.error) {
      let message = typeof body.error.message ? body.error.message : "Internal server error";
      let code = typeof body.error?.code === "string" ? (body.error.code as ErrorCode) : "FRONT_GATEWAY_MISSING_CODE";
      throw new ClientError(res.status, code, message);
    }

    return body as T;
  }

  json_headers({
    ip,
    token,
    wpayload
  }: {
    ip: string | null,
    token: string | null
    wpayload: boolean,
  }): Headers {
    
    const headers = new Headers();
    
    if(ip) headers.append(FORWARD_IP_HEADER, ip);
    if(token) headers.append(ACCESS_TOKEN_HEADER, token);
    if(wpayload) headers.append("content-type", "application/json");
    
    return headers
  }

  async json_request<T>(url: string, init: RequestInit): Promise<T> {
    const res = await this.fetch(url, init);
    const body = await this.get_json_body<T>(res);
    return body;
  }

  async get<T>(ip: string | null, token: string | null, url: string,): Promise<T> {
    return await this.json_request<T>(url, {
      headers: this.json_headers({ ip, token, wpayload: false }),
    });
  }

  async delete<T>(ip: string | null, token: string | null, url: string): Promise<T> {
    return await this.json_request<T>(url, {
      method: "DELETE",
      headers: this.json_headers({ ip, token, wpayload: false }),
    });
  }

  async post<T>(ip: string | null, token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "POST",
      headers: this.json_headers({ ip, token, wpayload: true }),
      body: JSON.stringify(payload)
    })
  }

  async put<T>(ip: string | null, token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "PUT",
      headers: this.json_headers({ ip, token, wpayload: true }),
      body: JSON.stringify(payload)
    })
  }

  async patch<T>(ip: string | null, token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "PATCH",
      headers: this.json_headers({ ip, token, wpayload: true }),
      body: JSON.stringify(payload)
    })
  }

  async me(ip: string | null, token: string): Promise<import("./defs/api/me/Output").Output> {
    return await this.get(ip, token, "/me");
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
}

export class AuthAdmin {
  client: Client;
  constructor(client: Client) {
    this.client = client
  }

  async login(ip: string | null, payload: import("./defs/api/auth/admin/login/POST/Payload").Payload): Promise<import("./defs/api/auth/admin/login/POST/Output").Output> {
    return await this.client.post(ip, null, "/auth/admin/login", payload)
  }

  async logout(ip: string | null, token: string): Promise<import("./defs/api/auth/admin/logout/POST/Output").Output> {
    return await this.client.post(ip, token, "/auth/admin/logout", void 0);
  }
}

export class AuthUser {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }

  async login(ip: string | null, payload: import("./defs/api/auth/user/login/POST/Payload").Payload): Promise<import("./defs/api/auth/user/login/POST/Output").Output> {
    return await this.client.post(ip, null, "/auth/user/login", payload)
  }

  async logout(ip: string | null, token: string): Promise<import("./defs/api/auth/user/logout/POST/Output").Output> {
    return await this.client.post(ip, token, "/auth/user/logout", void 0);
  }

  async register(ip: string | null, token: string, payload: import("./defs/api/auth/user/register/POST/Payload").Payload): Promise<import("./defs/api/auth/user/register/POST/Output").Output> {
    return await this.client.post(ip, token, "/auth/user/register", payload)
  }
}

export class Stations {

  client: Client;
  
  files: StationFiles;

  constructor(client: Client) {
    this.client = client;
    this.files = new StationFiles(client);
  }

  async list(ip: string | null, token: string, query: import("./defs/api/stations/GET/Query").Query): Promise<import("./defs/api/stations/GET/Output").Output> {
    return await this.client.get(ip, token, `/stations${qss(query)}`);
  }

  async get(ip: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/GET/Output").Output> {
    return await this.client.get(ip, token, `/stations/${id}`);
  }

  async post(ip: string | null, token: string, payload: import("./defs/api/stations/POST/Payload").Payload): Promise<import("./defs/api/stations/POST/Output").Output> {
    return await this.client.post(ip, token, `/stations`, payload);
  }

  async get_now_playing(ip: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/now-playing/GET/Output").Output> {
    return await this.client.get(ip, token, `/stations/${id}/now-playing`);
  }

  async get_dashboard_stats(ip: string | null, token: string, id: string): Promise<import("./defs/api/stations/[station]/dashboard-stats/GET/Output").Output> {
    return await this.client.get(ip, token, `/stations/${id}/dashboard-stats`);
  }
}

export class Users {
  client: Client;
  stations: UserStations;

  constructor(client: Client) {
    this.client = client;
    this.stations = new UserStations(client);
  }

  async list(ip: string | null, token: string, query: import("./defs/api/users/GET/Query").Query): Promise<import("./defs/api/stations/GET/Output").Output> {
    return await this.client.get(ip, token, `/users${qss(query)}`);
  }

  async get(ip: string | null, token: string, user_id: string): Promise<import("./defs/api/users/[user]/GET/Output").Output> {
    return await this.client.get(ip, token, `/users/${user_id}`);
  }

  async post(ip: string | null, token: string, payload: import("./defs/api/users/POST/Payload").Payload): Promise<import("./defs/api/users/POST/Output").Output> {
    return await this.client.post(ip, token, `/users`, payload);
  }
}

export class UserStations {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  /*
  async list(token: string, user_id: string, query: import("./defs/api/users/[user]GET/Query").Query): Promise<import("./defs/api/stations/GET/Output").Output> {
    return await this.client.get(`/users/${user_id}${qss(query)}`, token);
  }
  */
}


export class StationFiles {
  
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }


  async list(ip: string | null, token: string, station_id: string, query: import("./defs/api/stations/[station]/files/GET/Query").Query): Promise<import("./defs/api/stations/[station]/files/GET/Output").Output> {
    return await this.client.get(ip, token, `/stations/${station_id}/files${qss(query)}`);
  }

  async get(ip: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/GET/Output").Output> {
    return await this.client.get(ip, token, `/stations/${station_id}/files/${file_id}`);
  }

  async delete(ip: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/DELETE/Output").Output> {
    return await this.client.delete(ip, token, `/stations/${station_id}/files/${file_id}`);
  }

  async post(ip: string | null, token: string, station_id: string, content_type: string, content_length: number, query: import("./defs/api/stations/[station]/files/POST/Query").Query, data: Readable): Promise<import("./defs/api/stations/[station]/files/POST/Output").Output> {
    
    const headers = new Headers();

    if(ip) headers.append(FORWARD_IP_HEADER, ip);
    headers.append(ACCESS_TOKEN_HEADER, token);
    headers.append("content-type", content_type);
    headers.append("content-length", String(content_length));

    let res = await this.client.fetch(`/stations/${station_id}/files${qss(query)}`, {
      method: "POST",
      headers,
      body: data 
    })

    return await this.client.get_json_body(res)
  }

  async put_metadata(ip: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/metadata/PUT/Payload").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/metadata/PUT/Output").Output> {
    return await this.client.put(ip, token, `/stations/${station_id}/files/${file_id}/metadata`, payload);
  }

  async swap_order(ip: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/order/swap/POST/Payload").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/order/swap/POST/Output").Output> {
    return await this.client.post(ip, token, `/stations/${station_id}/files/${file_id}/order/swap`, payload);
  }

  async move_to_first(ip: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-to-first/POST/Output").Output> {
    return await this.client.post(ip, token, `/stations/${station_id}/files/${file_id}/order/move-to-first`, undefined);
  }

  async move_to_last(ip: string | null, token: string, station_id: string, file_id: string): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-to-last/POST/Output").Output> {
    return await this.client.post(ip, token, `/stations/${station_id}/files/${file_id}/order/move-to-last`, undefined);
  }

  async move_before(ip: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/order/move-before/POST/Payload").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-before/POST/Output").Output> {
    return await this.client.post(ip, token, `/stations/${station_id}/files/${file_id}/order/move-before`, payload)
  }

  async move_after(ip: string | null, token: string, station_id: string, file_id: string, payload: import("./defs/api/stations/[station]/files/[file]/order/move-after/POST/Payload").Payload): Promise<import("./defs/api/stations/[station]/files/[file]/order/move-after/POST/Output").Output> {
    return await this.client.post(ip, token, `/stations/${station_id}/files/${file_id}/order/move-after`, payload)
  }
}