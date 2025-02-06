import express, { type RequestHandler } from "express";
import type { Config } from "./config.js";
import type { Logger } from "./logger.js";
import { studio_api } from "./api/studio-api.js";

import path from "path";
import { env } from "./env.js";
import { color } from "./color.js";
import { kit } from "./kit.js";

import { fileURLToPath } from "url";
import { admin_api } from "./api/admin-api.js";
import { payments_api } from "./payments/payments.js";
import { get_client } from "./client.server.js";
const __dirname = path.dirname(fileURLToPath(import.meta.url));

export const start = async ({ config, logger }: { config: Config, logger: Logger }) => {

  process.title = "openstream-front";

  const client = get_client(config.openstream.api_base_url);

  if(config.studio?.enabled) {

    let app = express();
    
    app.use("/api", studio_api({ client, config, logger }))
    app.use(express.static(path.resolve(__dirname, "../../../static/studio"), { etag: true, dotfiles: "allow" }))

    // TODO: remove this
    if(env.SVELTEKIT_APP_DEV) {
      app.use((req, res) => {
        res.redirect(302, `http://${req.hostname}:${env.SVELTEKIT_APP_PORT}${req.url}`)
      })
      // app.use(sveltekit_dev_proxy(env.SVELTEKIT_APP_PORT))
    } else {
      process.env.APP_API_PORT = String(config.studio.port);
      const { handler }: { handler: RequestHandler } = await import("" + "../../app/build/handler.js")
      app.use(kit(handler));
    }

    await new Promise<void>((resolve, reject) => {
      app.listen(config.studio!.port, (...args: any[]) => {
        if(args[0]) return reject(args[0])
        logger.scoped("start").info(`studio server bound to port ${color.yellow(config.studio!.port)}`);
        resolve();
      });
    })
  }

  if(config.admin?.enabled) {

    let app = express();

    app.use("/api", admin_api({ client, config, logger }))
    app.use(express.static(path.resolve(__dirname, "../../../static/admin"), { etag: true, dotfiles: "allow" }))

    // TODO: remove this
    if(env.SVELTEKIT_ADMIN_DEV) {
      app.use((req, res) => {
        res.redirect(302, `http://${req.hostname}:${env.SVELTEKIT_ADMIN_PORT}${req.url}`)
      })
      //app.use(sveltekit_dev_proxy(env.SVELTEKIT_ADMIN_PORT))
    } else {
      process.env.ADMIN_API_PORT = String(config.admin.port);
      const { handler }: { handler: RequestHandler } = await import("" + "../../admin/build/handler.js")
      app.use(kit(handler));
    }

    await new Promise<void>((resolve, reject) => {
      app.listen(config.admin!.port, (...args: any[]) => {
        if(args[0]) return reject(args[0])
        logger.scoped("start").info(`admin server bound to port ${color.yellow(config.admin!.port)}`);
        resolve();
      });
    })
  }

  if(config.payments?.enabled) {

    let app = express();

    app.use(payments_api({ config: config.payments, logger }))

    await new Promise<void>((resolve, reject) => {
      app.listen(config.payments!.port, (...args: any[]) => {
        if(args[0]) return reject(args[0])
        logger.scoped("start").info(`payments server bound to port ${color.yellow(config.payments!.port)}`);
        resolve();
      });
    });
  }

  // embed
  if(config.embed?.enabled){
    process.env.EMBED_API_BASE_URL = config.openstream.api_base_url;
    const app = express();
    const { handler }: { handler: RequestHandler } = await import("" + "../../embed/build/handler.js")
    app.use(kit(handler));

    await new Promise<void>((resolve, reject) => {
      app.listen(config.embed!.port, (...args: any[]) => {
        if(args[0]) return reject(args[0])
        logger.scoped("start").info(`embed server bound to port ${color.yellow(config.embed!.port)}`);
        resolve();
      });
    })
  }
}