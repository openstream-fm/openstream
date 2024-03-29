import test from "ava";
import type { Request, Response } from "express";
import { type SessionData, decrypt, encrypt, session, get_cookie_session } from "./session.js";
import { type Config } from "./config.js";
import { ConsoleLogger } from "./logger.js";
import { LogLevel } from "./log-level.js";
import crypto from "crypto";

// @ts-ignore
const config: Config = {
  session: {
    secret: "secret",
    cookie_name: "cookie_name",
    max_age_days: 1,
  },
};

const logger = new ConsoleLogger(LogLevel.SILENT);

test("encrypt and decrypt", (t) => {
  const key = crypto.scryptSync(config.session.secret, "salt", 24);
  const value = "value";
  const encrypted = encrypt(value, key, logger);
  const decrypted = decrypt(encrypted, key, logger);
  t.is(decrypted, value);
});

test("get_cookie_session - success with valid cookie name and value", (t) => {
  const key = Buffer.alloc(24);
  const req: Request = {
    cookies: {
      [config.session.cookie_name]: encrypt(
        JSON.stringify({ device_id: "deviceid", user: { _id: "id", token: "token", media_key: "key" }, admin: null }),
        key,
        logger
      ),
    },
  } as Request;
  const sessionData: SessionData = { device_id: "deviceid", user: { _id: "id", token: "token", media_key: "key" }, admin: null };
  const result = get_cookie_session(req, config.session.cookie_name, key, logger);
  t.deepEqual(result, sessionData);
});

test("get_cookie_session - ignores invalid cookie", (t) => {
  const key = Buffer.alloc(24);
  const req: Request = {
    cookies: {
      [config.session.cookie_name]: "invalid",
    },
  } as Request;
  const sessionData: SessionData = { device_id: "deviceid", user: null, admin: null };
  // @ts-ignore
  const result = get_cookie_session(req, config.session.cookie_name, key, logger);
  t.true(typeof result.device_id === "string"),
  t.true(result.device_id.length === 24)
  t.true(result.user === null);
});

test("get_cookie_session - ignores malformed cookie", (t) => {
  const key = Buffer.alloc(24);
  const req: Request = {
    cookies: {
      [config.session.cookie_name]: "malformed",
    },
  } as Request;
  const result = get_cookie_session(req, config.session.cookie_name, key, logger);
  t.true(typeof result.device_id === "string");
  t.true(result.user === null);
});

test.todo("res.set_session - calls cookie")
// , (t) => {
//   // @ts-ignore
//   const req: Request = {};
//   // @ts-ignore
//   const res: Response = {};
  
//   const middle = session(config, logger);

//   middle(req, res, () => {
//     let cookie_name = "";
//     // @ts-ignore
//     res.cookie = (name) => {
//       cookie_name = name;
//     }

//     const data: SessionData = { user: { _id: "id", token: "token", media_key: "key" } };
//     res.set_session(data);
    
//     t.is(cookie_name, config.session.cookie_name);
//   })
// });

test.todo("res.clear_session - calls clear cookie")
// , (t) => {
//   // @ts-ignore
//   const res: Response = {};
//   // @ts-ignore
//   const req: Requeest = {};


//   const middle = session(config, logger);

//   middle(req, res, () => {

//     let cookie_name = "";
    
//     // @ts-ignore
//     res.clearCookie = (name) => {
//       cookie_name = name;
//     }

//     res.clear_session();

//     t.is(cookie_name, config.session.cookie_name);
//   })
// });

test.todo("req.cookie_session - loads cookie session data")
// (t) => {
  
//   let key = crypto.scryptSync(config.session.secret, "salt", 24);
  
//   // @ts-ignore
//   const req: Request = {
//     headers: { "cookie": `${config.session.cookie_name}=${encrypt(JSON.stringify({ user: { _id: "id", token: "token", media_key: "key" } }), key, logger)}`  }
//   } as Request;
  
//   // @ts-ignore
//   const res = {} as Response;
//   const next = () => {};
  
//   const middleware = session(config, logger);
//   middleware(req, res, next);
//   t.deepEqual(req.cookie_session, { user: { _id: "id", token: "token", media_key: "key" } });
// });