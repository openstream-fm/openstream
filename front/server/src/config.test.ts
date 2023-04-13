/// config.test.ts
import test from "ava";
import { load_from_string, merge_env, load, type Config } from "./config";
import { fileURLToPath } from "url";
import path from "path";
import { ConsoleLogger } from "./logger";
import { LogLevel } from "./log-level";

const exampleConfig = `
[openstream]
api_base_url = "https://api.example.com"
token = "example_token"

# [public]
# stream_public_url = "https://stream.example.com"
# storage_public_url = "https://storage.example.com"

[mongodb]
url = "mongodb://localhost:27017"

[session]
secret = "example_secret"
# domain = "example.com"
max_age_days = 30
cookie_name = "example_cookie"

[studio]
enabled = true
port = 3000
# public_base_url = "https://studio.example.com"
`;

const exampleEnv = {
  OPENSTREAM_FRONT_OPENSTREAM_API_BASE_URL: "https://api.env.com",
  OPENSTREAM_FRONT_OPENSTREAM_TOKEN: "env_token",
  // OPENSTREAM_FRONT_PUBLIC_STREAM_PUBLIC_URL: "https://stream.env.com",
  // OPENSTREAM_FRONT_PUBLIC_STORAGE_PUBLIC_URL: "https://storage.env.com",
  OPENSTREAM_FRONT_MONGODB_URL: "mongodb://env:27017",
  OPENSTREAM_FRONT_SESSION_SECRET: "env_secret",
  OPENSTREAM_FRONT_SESSION_DOMAIN: "env.com",
  OPENSTREAM_FRONT_SESSION_MAX_AGE_DAYS: "20",
  OPENSTREAM_FRONT_SESSION_COOKIE_NAME: "env_cookie",
  OPENSTREAM_FRONT_STUDIO_ENABLED: "false",
  OPENSTREAM_FRONT_STUDIO_PORT: "4000",
  // OPENSTREAM_FRONT_STUDIO_PUBLIC_BASE_URL: "https://studio.env.com",
};

test("load_from_string - success on valid input", (t) => {
  const config = load_from_string(exampleConfig);
  t.is(config.openstream.api_base_url, "https://api.example.com");
  t.is(config.openstream.token, "example_token");
  // t.is(config.public.stream_public_url, "https://stream.example.com");
  // t.is(config.public.storage_public_url, "https://storage.example.com");
  t.is(config.mongodb.url, "mongodb://localhost:27017");
  t.is(config.session.secret, "example_secret");
  // t.is(config.session.domain, "example.com");
  t.is(config.session.max_age_days, 30);
  t.is(config.session.cookie_name, "example_cookie");
  t.is(config.studio.enabled, true);
  t.is(config.studio.port, 3000);
  // t.is(config.studio.public_base_url, "https://studio.example.com");
});

test("merge_env - success on valid input", (t) => {
  const config: Config = load_from_string(exampleConfig);
  const mergedConfig = merge_env(config, { env: exampleEnv });
  t.is(mergedConfig.openstream?.api_base_url, "https://api.env.com");
  t.is(mergedConfig.openstream?.token, "env_token");
  // t.is(mergedConfig.public?.stream_public_url, "https://stream.env.com");
  // t.is(mergedConfig.public?.storage_public_url, "https://storage.env.com");
  t.is(mergedConfig.mongodb?.url, "mongodb://env:27017");
  t.is(mergedConfig.session?.secret, "env_secret");
  // t.is(mergedConfig.session?.domain, "env.com");
  t.is(mergedConfig.session?.max_age_days, 20);
  t.is(mergedConfig.session?.cookie_name, "env_cookie");
  t.is(mergedConfig.studio?.enabled, false);
  t.is(mergedConfig.studio?.port, 4000);
  // t.is(mergedConfig.studio?.public_base_url, "https://studio.env.com");
});

test("load - success loading sample config file", (t) => {
  const __dirname = path.dirname(fileURLToPath(import.meta.url));
  t.notThrows(() => load(`${__dirname}/../config.sample.toml`, { logger: new ConsoleLogger(LogLevel.SILENT) }));
});

/// config.test.ts (continued)

test("load_from_string - with missing properties", (t) => {
  const incompleteConfig = `
[openstream]
api_base_url = "https://api.example.com"

# [public]
# stream_public_url = "https://stream.example.com"

[mongodb]
url = "mongodb://localhost:27017"

[session]
secret = "example_secret"
domain = "example.com"
max_age_days = 30
`;

  t.throws(() => {
    load_from_string(incompleteConfig);
  });
});

test("merge_env - throws with invalid number value", (t) => {
  const config: Config = load_from_string(exampleConfig);
  const invalidEnv = { ...exampleEnv, OPENSTREAM_FRONT_SESSION_MAX_AGE_DAYS: "invalid" };

  t.throws(() => {
    merge_env(config, { env: invalidEnv });
  });
});

test("merge_env - throws with invalid boolean value", (t) => {
  const config: Config = load_from_string(exampleConfig);
  const invalidEnv = { ...exampleEnv, OPENSTREAM_FRONT_STUDIO_ENABLED: "invalid" };
  t.throws(() => {
    merge_env(config, { env: invalidEnv });
  });
});

test("load - throws with non-existent file", (t) => {
  const logger = new ConsoleLogger(LogLevel.SILENT);
  const error: any = t.throws(() => {
    load("./non_existent.toml", { logger, env: exampleEnv });
  });

  t.is(error?.code, "ENOENT");
});

test("load_from_string - throws with invalid TOML format", (t) => {
  const invalidToml = `
[openstream]
api_base_url = "https://api.example.com"
token = "example_token"

# [public]
# stream_public_url = "https://stream.example.com"
# storage_public_url = "https://storage.example.com"

[mongodb]
url = "mongodb://localhost:27017"

[session]
secret = "example_secret"
# domain = "example.com"
max_age_days = 30
cookie_name = "example_cookie"

[studio]
enabled = true
port = 3000
public_base_url = "https://studio.example.com"
invalid_key = invalid_value
`;

  t.throws(() => {
    load_from_string(invalidToml);
  });
});