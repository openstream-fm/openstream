import StatusCode from "http-status-codes";
import fetch from "node-fetch";
import http from "http";
import https from "https";
export class ClientError extends Error {
    status;
    code;
    constructor(status, code, message) {
        super(message);
        this.status = status;
        this.code = code;
    }
}
export class Client {
    baseURL;
    agent;
    constructor(baseURL) {
        this.baseURL = baseURL.trim().replace(/\/+$/g, "");
        this.agent = this.baseURL.startsWith("https") ? new https.Agent() : new http.Agent();
    }
    async fetch(url, init = {}) {
        return await fetch(`${this.baseURL}/${url}`, {
            ...init,
            agent: this.agent,
        }).catch(e => {
            throw new ClientError(StatusCode.BAD_GATEWAY, "ERR_GATEWAY_FETCH", "Gateway unavailable");
        });
    }
    async get_json_body(res) {
        const body = await res.json().catch(e => {
            throw new ClientError(StatusCode.BAD_GATEWAY, "ERR_GATEWAY_JSON", "Gateway error");
        });
        if (body?.error) {
            let message = String(body.error?.message || "Internal server error");
            let code = String(body.error?.code || "ERR_GATEWAY_MISSING_CODE");
            throw new ClientError(res.status, message, code);
        }
        return body;
    }
    async json_request(url, init) {
        const res = await this.fetch(`${this.baseURL}${url}`, init);
        const body = await this.get_json_body(res);
        return body;
    }
    async get(url, token) {
        return await this.json_request(url, {
            headers: {
                "x-access-token": token
            }
        });
    }
    async delete(url, token) {
        return await this.json_request(url, {
            method: "DELETE",
            headers: {
                "x-access-token": token
            }
        });
    }
    async post(url, token, payload) {
        return await this.json_request(url, {
            method: "POST",
            headers: {
                "x-access-token": token,
                "content-type": "application/json",
            },
            body: JSON.stringify(payload)
        });
    }
    async put(url, token, payload) {
        return await this.json_request(url, {
            method: "PUT",
            headers: {
                "x-access-token": token,
                "content-type": "application/json",
            },
            body: JSON.stringify(payload)
        });
    }
    async patch(url, token, payload) {
        return await this.json_request(url, {
            method: "PATCH",
            headers: {
                "x-access-token": token,
                "content-type": "application/json",
            },
            body: JSON.stringify(payload)
        });
    }
}
export class Accounts {
    client;
    constructor(client) {
        this.client = client;
    }
}
