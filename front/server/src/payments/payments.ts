import { Router } from "express"
import { Config } from "../config";
import { Logger } from "../logger";
import { BraintreePaymentsClient } from "./braintree-payments-client";
import { add_all } from "../defs/payments/api/router";
import { catch_handler, validate_rethrow } from "./error";
import { json_catch_handler } from "../error";

export const payments_api = ({ config, logger: _logger }: { config: Exclude<Config["payments"], undefined> , logger: Logger }) => {
  const logger = _logger.scoped("payments");

  const client = new BraintreePaymentsClient(config.credentials);
  
  const api = Router();

  add_all(api, validate_rethrow, client);

  api.use(json_catch_handler);

  api.use(catch_handler({ logger }))

  return api;
}