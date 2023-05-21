import express, { RequestHandler } from "express";
import type { Config } from "./config";
import type { Logger } from "./logger";
import { studio_api } from "./api/studio-api";

import path from "path";
import { env } from "./env";
import { sveltekit_dev_proxy } from "./sveltekit-dev-proxy";
import { color } from "./color";
import { kit } from "./kit";

import { fileURLToPath } from "url";
import { admin_api } from "./api/admin-api";
const __dirname = path.dirname(fileURLToPath(import.meta.url));

export const start = async ({ config, logger }: { config: Config, logger: Logger }) => {

  process.title = "openstream-front";

  if(config.studio.enabled) {

    let app = express();

    app.use("/api", studio_api({ config, logger }))
    app.use(express.static(path.resolve(__dirname, "../../static/studio"), { etag: true, dotfiles: "allow" }))

    if(env.SVELTEKIT_APP_DEV) {
      app.use(sveltekit_dev_proxy(env.SVELTEKIT_APP_PORT))
    } else {
      process.env.APP_API_PORT = String(config.studio.port);
      const { handler }: { handler: RequestHandler } = await import("" + "../../app/build/handler.js")
      app.use(kit(handler));
    }

    app.listen(config.studio.port, () => {
      logger.scoped("start").info(`studio server bound to port ${color.yellow(config.studio!.port)}`);
    });
  }

  if(config.admin.enabled) {

    let app = express();

    app.use("/api", admin_api({ config, logger }))
    app.use(express.static(path.resolve(__dirname, "../../static/admin"), { etag: true, dotfiles: "allow" }))

    if(env.SVELTEKIT_ADMIN_DEV) {
      app.use(sveltekit_dev_proxy(env.SVELTEKIT_ADMIN_PORT))
    } else {
      process.env.ADMIN_API_PORT = String(config.admin.port);
      const { handler }: { handler: RequestHandler } = await import("" + "../../admin/build/handler.js")
      app.use(kit(handler));
    }

    app.listen(config.admin.port, () => {
      logger.scoped("start").info(`admin server bound to port ${color.yellow(config.admin!.port)}`);
    });
  }
}