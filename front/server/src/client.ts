import StatusCode from "http-status-codes";
import qs from "qs";
import type { ErrorCode } from "./types";
import { Readable } from "stream";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "./contants";
import { ClientError } from "./client-error";
import { Logger } from "./logger";
import fetch, { Response, RequestInit } from "node-fetch";

import http from "http";
import https from "https";

const qss = (v: any) => {
  return qs.stringify(v, { addQueryPrefix: true, skipNulls: true })
}

export class Client {

  private base_url: string;
  logger: Logger;

  accounts: Accounts;
  users: Users;
  auth: Auth;

  constructor(base_url: string, { logger }: { logger: Logger }) {
    this.base_url = base_url.trim().replace(/\/+$/g, "")
    this.logger = logger.scoped("client");
    
    this.accounts = new Accounts(this);
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
  }): Record<string, string> {
    const _ip = ip ? { [FORWARD_IP_HEADER]: ip } : {} as Record<string, string>; 
    const _token = token ? { [ACCESS_TOKEN_HEADER]: token } : {} as Record<string, string>;
    const _content = wpayload ? { "content-type": "application/json" } : {} as Record<string, string>;
    return {
      ..._ip,
      ..._token,
      ..._content,
    }
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

export class Accounts {

  client: Client;
  
  files: AccountFiles;

  constructor(client: Client) {
    this.client = client;
    this.files = new AccountFiles(client);
  }

  async list(ip: string | null, token: string, query: import("./defs/api/accounts/GET/Query").Query): Promise<import("./defs/api/accounts/GET/Output").Output> {
    return await this.client.get(ip, token, `/accounts${qss(query)}`);
  }

  async get(ip: string | null, token: string, id: string): Promise<import("./defs/api/accounts/[account]/GET/Output").Output> {
    return await this.client.get(ip, token, `/accounts/${id}`);
  }

  async post(ip: string | null, token: string, payload: import("./defs/api/accounts/POST/Payload").Payload): Promise<import("./defs/api/accounts/POST/Output").Output> {
    return await this.client.post(ip, token, `/accounts`, payload);
  }
}

export class Users {
  client: Client;
  accounts: UserAccounts;

  constructor(client: Client) {
    this.client = client;
    this.accounts = new UserAccounts(client);
  }

  async list(ip: string | null, token: string, query: import("./defs/api/users/GET/Query").Query): Promise<import("./defs/api/accounts/GET/Output").Output> {
    return await this.client.get(ip, token, `/users${qss(query)}`);
  }

  async get(ip: string | null, token: string, user_id: string): Promise<import("./defs/api/users/[user]/GET/Output").Output> {
    return await this.client.get(ip, token, `/users/${user_id}`);
  }

  async post(ip: string | null, token: string, payload: import("./defs/api/users/POST/Payload").Payload): Promise<import("./defs/api/users/POST/Output").Output> {
    return await this.client.post(ip, token, `/users`, payload);
  }
}

export class UserAccounts {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  /*
  async list(token: string, user_id: string, query: import("./defs/api/users/[user]GET/Query").Query): Promise<import("./defs/api/accounts/GET/Output").Output> {
    return await this.client.get(`/users/${user_id}${qss(query)}`, token);
  }
  */
}


export class AccountFiles {
  
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }


  async list(ip: string | null, token: string, account_id: string, query: import("./defs/api/accounts/[account]/files/GET/Query").Query): Promise<import("./defs/api/accounts/[account]/files/GET/Output").Output> {
    return await this.client.get(ip, token, `/accounts/${account_id}/files${qss(query)}`);
  }

  async get(ip: string | null, token: string, account_id: string, file_id: string): Promise<import("./defs/api/accounts/[account]/files/[file]/GET/Output").Output> {
    return await this.client.get(ip, token, `/accounts/${account_id}/files/${file_id}`);
  }

  async delete(ip: string | null, token: string, account_id: string, file_id: string): Promise<import("./defs/api/accounts/[account]/files/[file]/DELETE/Output").Output> {
    return await this.client.delete(ip, token, `/accounts/${account_id}/files/${file_id}`);
  }

  async post(ip: string | null, token: string, account_id: string, content_type: string, content_length: number, query: import("./defs/api/accounts/[account]/files/POST/Query").Query, data: Readable): Promise<import("./defs/api/accounts/[account]/files/POST/Output").Output> {
    let res = await this.client.fetch(`/accounts/${account_id}/files${qss(query)}`, {
      method: "POST",
      headers: {
        ...(ip ? { [FORWARD_IP_HEADER]: ip } : {}),
        "content-type": content_type,
        "content-length": String(content_length),
        [ACCESS_TOKEN_HEADER]: token,
      },
      body: data 
    })

    return await this.client.get_json_body(res)
  }
}