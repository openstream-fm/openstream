import test from "ava";
import { load, load_from_string, merge_env } from "./config";
import type { Config } from "./config";
import { ConsoleLogger } from "./logger";
import { LogLevel } from "./log-level";
import path from "path"
import { fileURLToPath } from "url"

const __dirname = path.dirname(fileURLToPath(import.meta.url))

const defaultConfig: Config = {
  openstream: {
    api_base_url: "https://api.openstream.com",
    token: "test_token",
  },
  session: {
    secret: "test_secret",
    max_age_days: 30,
    cookie_name: "test_cookie",
  },
  studio: {
    enabled: true,
    port: 3000,
  },
  source_port: {
    local: 8000,
    test: 8000,
    srv1: 18000,
    srv2: 28000,
    default: 8000,
  }
};

test("load_from_string should load config from JSON string", (t) => {
  const jsonString = JSON.stringify(defaultConfig);
  const config = load_from_string(jsonString, "json");
  t.deepEqual(config, defaultConfig);
});

test("load_from_string should load config from TOML string", (t) => {
  const tomlString = `
[openstream]
api_base_url = "https://api.openstream.com"
token = "test_token"

[session]
secret = "test_secret"
max_age_days = 30
cookie_name = "test_cookie"

[studio]
enabled = true
port = 3000

[source_port]
local = 8000
staging = 8000
srv1 = 18000
srv2 = 28000
default = 8000
`;
  const config = load_from_string(tomlString, "toml");
  t.deepEqual(config, defaultConfig);
});

test("merge_env should merge environment variables with config", (t) => {
  const env = {
    OPENSTREAM_FRONT_OPENSTREAM_API_BASE_URL: "https://api.openstream.com/env",
    OPENSTREAM_FRONT_OPENSTREAM_TOKEN: "test_token_env",
    OPENSTREAM_FRONT_SESSION_SECRET: "test_secret_env",
    OPENSTREAM_FRONT_SESSION_MAX_AGE_DAYS: "40",
    OPENSTREAM_FRONT_SESSION_COOKIE_NAME: "test_cookie_env",
    OPENSTREAM_FRONT_STUDIO_ENABLED: "false",
    OPENSTREAM_FRONT_STUDIO_PORT: "4000",
    OPENSTREAM_FRONT_SOURCE_PORT_LOCAL: "8000",
    OPENSTREAM_FRONT_SOURCE_PORT_STAGING: "8000",
    OPENSTREAM_FRONT_SOURCE_PORT_SRV1: "18000",
    OPENSTREAM_FRONT_SOURCE_PORT_SRV2: "28000",
    OPENSTREAM_FRONT_SOURCE_PORT_DEFAULT: "8000",
  };

  const expectedConfig: Config = {
    openstream: {
      api_base_url: "https://api.openstream.com/env",
      token: "test_token_env",
    },
    session: {
      secret: "test_secret_env",
      max_age_days: 40,
      cookie_name: "test_cookie_env",
    },
    studio: {
      enabled: false,
      port: 4000,
    },
    source_port: {
      local: 8000,
      test: 8000,
      srv1: 18000,
      srv2: 28000,
      default: 8000,
    }
  };

  const mergedConfig = merge_env(defaultConfig, { env });
  t.deepEqual(mergedConfig, expectedConfig);
});

test("merge_env should throw error for invalid number environment variable", (t) => {
  const env = {
    OPENSTREAM_FRONT_SESSION_MAX_AGE_DAYS: "invalid_number",
  };

  t.throws(() => merge_env(defaultConfig, { env }), {
    message: /env.OPENSTREAM_FRONT_SESSION_MAX_AGE_DAYS should be a number/,
  });
});

test("merge_env should throw error for invalid boolean environment variable", (t) => {
  const env = {
    OPENSTREAM_FRONT_STUDIO_ENABLED: "invalid_boolean",
  };

  t.throws(() => merge_env(defaultConfig, { env }), {
    message: /env OPENSTREAM_FRONT_STUDIO_ENABLED should be a boolean/,
  });
});

test("load_from_string should load config from JSON string with missing properties", (t) => {
  const partialJsonString = JSON.stringify({
    openstream: {
      api_base_url: "https://api.openstream.com",
    },
    session: {
      secret: "test_secret",
      max_age_days: 30,
    },
  });

  const env = {
    OPENSTREAM_FRONT_OPENSTREAM_TOKEN: "test_token",
    OPENSTREAM_FRONT_SESSION_COOKIE_NAME: "test_cookie",
    OPENSTREAM_FRONT_STUDIO_ENABLED: "true",
    OPENSTREAM_FRONT_STUDIO_PORT: "3000",
    OPENSTREAM_FRONT_SOURCE_PORT_LOCAL: "8000",
    OPENSTREAM_FRONT_SOURCE_PORT_STAGING: "8000",
    OPENSTREAM_FRONT_SOURCE_PORT_SRV1: "18000",
    OPENSTREAM_FRONT_SOURCE_PORT_SRV2: "28000",
    OPENSTREAM_FRONT_SOURCE_PORT_DEFAULT: "8000",
  };

  const config = load_from_string(partialJsonString, "json", { env });
  t.deepEqual(config, defaultConfig);
});

test("load_from_string should load config from TOML string with missing properties", (t) => {
  const partialTomlString = `
[openstream]
api_base_url = "https://api.openstream.com"

[session]
secret = "test_secret"
max_age_days = 30
`;

  const env = {
    OPENSTREAM_FRONT_OPENSTREAM_TOKEN: "test_token",
    OPENSTREAM_FRONT_SESSION_COOKIE_NAME: "test_cookie",
    OPENSTREAM_FRONT_STUDIO_ENABLED: "true",
    OPENSTREAM_FRONT_STUDIO_PORT: "3000",
    OPENSTREAM_FRONT_SOURCE_PORT_LOCAL: "8000",
    OPENSTREAM_FRONT_SOURCE_PORT_STAGING: "8000",
    OPENSTREAM_FRONT_SOURCE_PORT_SRV1: "18000",
    OPENSTREAM_FRONT_SOURCE_PORT_SRV2: "28000",
    OPENSTREAM_FRONT_SOURCE_PORT_DEFAULT: "8000",
  };

  const config = load_from_string(partialTomlString, "toml", { env });
  t.deepEqual(config, defaultConfig);
});

test("load_from_string should throw error for invalid JSON string", (t) => {
  const invalidJsonString = `{"openstream": {"api_base_url": "https://api.openstream.com", "token": "test_token"},}`;

  t.throws(() => load_from_string(invalidJsonString, "json"));
});

test("load_from_string should throw error for invalid TOML string", (t) => {
  const invalidTomlString = `
[openstream]
api_base_url = "https://api.openstream.com"
token = "test_token"
invalid_key = "invalid_value"
`;

  t.throws(() => load_from_string(invalidTomlString, "toml"));
});

test("load_from_string should throw error for invalid property type in JSON string", (t) => {
  const invalidJsonString = JSON.stringify({
    openstream: {
      api_base_url: "https://api.openstream.com",
      token: "test_token",
    },
    session: {
      secret: "test_secret",
      max_age_days: "invalid_type",
      cookie_name: "test_cookie",
    },
    studio: {
      enabled: true,
      port: 3000,
    },
  });

  t.throws(() => load_from_string(invalidJsonString, "json"));
});

test("load_from_string should throw error for invalid property type in TOML string", (t) => {
  const invalidTomlString = `
[openstream]
api_base_url = "https://api.openstream.com"
token = "test_token"

[session]
secret = "test_secret"
max_age_days = "invalid_type"
cookie_name = "test_cookie"

[studio]
enabled = true
port = 3000
`;

  t.throws(() => load_from_string(invalidTomlString, "toml"));
});

test("load - toml - can load default config file", t => {
  t.notThrows(() => {
    load(path.resolve(__dirname + "../../openstream-front.sample.toml"), { logger: new ConsoleLogger(LogLevel.SILENT), env: {} });
  })
})

test("load - json - can load default config file", t => {
  t.notThrows(() => {
    load(path.resolve(__dirname + "../../openstream-front.sample.toml"), { logger: new ConsoleLogger(LogLevel.SILENT), env: {} });
  })
})

test("config - toml and json default config files are identical", t => {
  const toml = load(path.resolve(__dirname + "../../openstream-front.sample.toml"), { logger: new ConsoleLogger(LogLevel.SILENT), env: {} });
  const json = load(path.resolve(__dirname + "../../openstream-front.sample.json"), { logger: new ConsoleLogger(LogLevel.SILENT), env: {} });
  // remove symbols (CommentJSON)
  t.deepEqual(toml, { ...json });
})