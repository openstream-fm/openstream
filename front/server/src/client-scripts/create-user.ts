import { fileURLToPath } from "url";
import { load } from "../config";
import { default_logger } from "../logger";
import { Client } from "../client";
import readline from "readline/promises";

const rl = readline.createInterface(process.stdin, process.stdout, void 0, true);

const logger = default_logger;

const __dirname = fileURLToPath(import.meta.url);

const toml = "../../openstream-front.toml";

const config = await load(toml, { logger });

const client = new Client(config.openstream.api_base_url, { logger });

logger.info("creating new user, please fill the fields");

const payload = {
  first_name: (await rl.question("First name: ")).trim(),
  last_name: (await rl.question("Last name: ")).trim(),
  phone: (await rl.question("Phone: ")).trim(),
  email: (await rl.question("Email: ")).trim(),
  password: await rl.question("Password: "),
}

logger.info("posting new user")

const output = await client.users.post(null, "openstream-client", config.openstream.token, payload);

logger.info("new user created");

console.log(output);

process.exit(0);
