import test, { type ExecutionContext } from "ava";
import { Client } from "../client";
import { LogLevel } from "../log-level";
import { ConsoleLogger } from "../logger";
import { IValidation, validateEquals } from "typia";

const base_url = process.env.OPENSTREAM_FRONT_OPENSTREAM_API_BASE_URL || "https://api.local.openstream.fm";
const token = process.env.OPENSTREAM_FRONT_OPENSTREAM_TOKEN!;

const ua = null;
const ip = "0.0.0.0";

const logger = new ConsoleLogger(LogLevel.DEBUG);

const client = new Client(base_url, { logger })

const assert_validate = <T>(t: ExecutionContext<unknown>, v: IValidation<T>) => {
  if(v.success) {
    t.true(v.success)
  } else {
    throw Error(v.errors.map(error => {
      return `${error.path} - expected: ${error.expected} - got: ${error.value}`
    }).join("\n"))
  }
}

test.todo("client integration tests");

// TODO: implement this with fake database
// TODO: tsc is going crazy with max stack size exceeded error on build

// test.before("integration - env token", t => {
//   t.assert(token != null, "env.OPENSTREAM_FRONT_OPENSTREAM_TOKEN must be present");
// })

// test("integration - users.list", async t => {
//   const output = await client.users.list(ip, ua, token, { limit: 1, skip: 1 });
//   assert_validate(t, validateEquals<import("$api/users/GET/Output").Output>(output));
// })

// test("integration - accounts.list", async t => {
//   const output = await client.accounts.list(ip, ua, token, { limit: 1, skip: 1 });
//   assert_validate(t, validateEquals<import("$api/accounts/GET/Output").Output>(output));
// })

// test("integration - stations.list", async t => {
//   const output = await client.stations.list(ip, ua, token, { limit: 1, skip: 1 });
//   assert_validate(t, validateEquals<import("$api/stations/GET/Output").Output>(output));
// })

// test("integration - devices.list", async t => {
//   const output = await client.devices.list(ip, ua, token, { limit: 1, skip: 1 });
//   assert_validate(t, validateEquals<import("$api/me/devices/GET/Output").Output>(output));
// })

