import express from "express";
import type { Config } from "./config";
import type { Logger } from "./logger";
import { appApi } from "./api/appApi";

import path from "path";
import { env } from "./env";
import { sveltekitDevProxy } from "./sveltekit-dev-proxy";
import { color } from "./color";

export const start = async ({ config, logger }: { config: Config, logger: Logger }) => {

  process.title = "openstream-front";

  if(config.app) {

    let app = express();

    app.use("/api", appApi({ config, logger }))

    app.use(express.static(path.resolve(__dirname, "../../../static"), { etag: true }))

    if(env.SVELTEKIT_APP_DEV) {
      app.use(sveltekitDevProxy(env.SVELTEKIT_APP_PORT))
    } else {
      // nasty hack to stop typescript to transform the import into a require
      // @ts-ignore
      process.env.APP_API_PORT = String(config.app.port);
      const { handler } = await (new Function("", 'return import("../../app/build/handler.js")'))();
      app.use(handler);
    }

    app.listen(config.app.port, () => {
      logger.info(`api server bound to port ${color.yellow(config.app!.port)}`);
    });
  }
}