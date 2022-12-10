import StatusCode from "http-status-codes";
import fetch, { Response, RequestInit } from "node-fetch";
import http from "http";
import https from "https";
import qs from "qs";
import type { ErrorCode } from "./types";


const qss = (v: any) => {
  qs.stringify(v, { addQueryPrefix: true, skipNulls: true })
}

export class ClientError extends Error {
  status: number
  code: ErrorCode

  constructor(status: number, code: ErrorCode, message: string) {
    super(message);
    this.status = status;
    this.code = code;
  }
}

export class Client {

  private baseURL: string;
  private agent: http.Agent | https.Agent;

  accounts: Accounts;
  users: Users;

  constructor(baseURL: string) {
    this.baseURL = baseURL.trim().replace(/\/+$/g, "")
    this.agent = this.baseURL.startsWith("https") ? new https.Agent() : new http.Agent();
    
    this.accounts = new Accounts(this);
    this.users = new Users(this);
  }

  async fetch(url: string, init: RequestInit = {}): Promise<Response> {
    return await fetch(`${this.baseURL}/${url}`, {
      ...init,
      agent: this.agent,
    }).catch(e => {
      throw new ClientError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_FETCH", "Gateway unavailable")
    })
  }

  async get_json_body<T>(res: Response): Promise<T> {

    const body: any = await res.json().catch(e => {
      throw new ClientError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_JSON", "Gateway error");
    })

    if(body?.error) {
      let message = typeof body.error.message ? body.error.message : "Internal server error";
      let code = typeof body.error?.code === "string" ? (body.error.code as ErrorCode) : "FRONT_GATEWAY_MISSING_CODE";
      throw new ClientError(res.status, code, message);
    }

    return body as T;
  }

  json_headers(token: string | null, with_payload: boolean): Record<string, string> {
    const _token = token ? { "x-access-token": token } : {} as Record<string, string>;
    const _content = with_payload ? { "content-type": "application/json" } : {} as Record<string, string>;
    return {
      ..._token,
      ..._content,
    }
  }

  async json_request<T>(url: string, init: RequestInit): Promise<T> {
    const res = await this.fetch(`${this.baseURL}${url}`, init);
    const body = await this.get_json_body<T>(res);
    return body;
  }

  async get<T>(token: string | null, url: string,): Promise<T> {
    return await this.json_request<T>(url, {
      headers: this.json_headers(token, false),
    });
  }

  async delete<T>(token: string | null, url: string): Promise<T> {
    return await this.json_request<T>(url, {
      method: "DELETE",
      headers: this.json_headers(token, false),
    });
  }

  async post<T>(token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "POST",
      headers: this.json_headers(token, true),
      body: JSON.stringify(payload)
    })
  }

  async put<T>(token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "PUT",
      headers: this.json_headers(token, true),
      body: JSON.stringify(payload)
    })
  }

  async patch<T>(token: string | null, url: string, payload: any): Promise<T> {
    return await this.json_request<T>(url, {
      method: "PATCH",
      headers: this.json_headers(token, true),
      body: JSON.stringify(payload)
    })
  }


  async login(payload: import("./defs/api/login/POST/Payload").Payload): Promise<import("./defs/api/login/POST/Output").Output> {
    return await this.post(null, "/login", payload)
  }

  async register(token: string, payload: import("./defs/api/register/POST/Payload").Payload): Promise<import("./defs/api/register/POST/Output").Output> {
    return await this.post(token, "/register", payload)
  }
}


export class Accounts {

  client: Client;
  
  files: AccountFiles;

  constructor(client: Client) {
    this.client = client;
    this.files = new AccountFiles(client);
  }

  async list(token: string, query: import("./defs/api/accounts/GET/Query").Query): Promise<import("./defs/api/accounts/GET/Output").Output> {
    return await this.client.get(token, `/accounts${qss(query)}}`);
  }

  async get(token: string, id: string): Promise<import("./defs/api/accounts/[account]/GET/Output").Output> {
    return await this.client.get(token, `/accounts/${id}`);
  }

  async post(token: string, payload: import("./defs/api/accounts/POST/Payload").Payload): Promise<import("./defs/api/accounts/POST/Output").Output> {
    return await this.client.post(token, `/accounts`, payload);
  }
}

export class Users {
  client: Client;
  accounts: UserAccounts;

  constructor(client: Client) {
    this.client = client;
    this.accounts = new UserAccounts(client);
  }

  async list(token: string, query: import("./defs/api/users/GET/Query").Query): Promise<import("./defs/api/accounts/GET/Output").Output> {
    return await this.client.get(token, `/users${qss(query)}`);
  }

  async get(token: string, userId: string): Promise<import("./defs/api/users/[user]/GET/Output").Output> {
    return await this.client.get(token, `/users/${userId}`);
  }

  async post(token: string, payload: import("./defs/api/users/POST/Payload").Payload): Promise<import("./defs/api/users/POST/Output").Output> {
    return await this.client.post(token, `/users`, payload);
  }
}

export class UserAccounts {
  client: Client;
  constructor(client: Client) {
    this.client = client;
  }
  /*
  async list(token: string, userId: string, query: import("./defs/api/users/[user]GET/Query").Query): Promise<import("./defs/api/accounts/GET/Output").Output> {
    return await this.client.get(`/users/${userId}${qss(query)}`, token);
  }
  */
}


export class AccountFiles {
  
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }


  async list(token: string, accountId: string, query: import("./defs/api/accounts/[account]/files/GET/Query").Query): Promise<import("./defs/api/accounts/[account]/files/GET/Output").Output> {
    return await this.client.get(token, `/accounts/${accountId}/files${qss(query)}`);
  }

  async get(token: string, accountId: string, fileId: string): Promise<import("./defs/api/accounts/[account]/files/[file]/GET/Output").Output> {
    return await this.client.get(token, `/accounts/${accountId}/files/${fileId}`);
  }

  // async post(token: string, accountId: string, query: import("./defs/api/accounts/[account]/files/POST/Query").Query, data: Readable): Promise<import("./defs/api/accounts/[account]/files/POST/Output").Output> {
  //    return await this.client.post(token, `/users`, payload);
  // }
}