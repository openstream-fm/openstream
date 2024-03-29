import { type RequestHandler, Router } from "express"
import type { Config } from "../config.js";
import type { Logger } from "../logger.js";
import { BraintreePaymentsClient } from "./braintree-payments-client.js";
import { add_all } from "../defs/payments/api/router.js";
import body_parser from "body-parser";
import { access_token_error, catch_handler, not_found_handler, validate_rethrow } from "./error.js";
import { PAYMENTS_ACCESS_TOKEN_HEADER } from "../constants.js";

export const access_token_auth = (access_token: string): RequestHandler => { 
  return (req, res, next) => { 
    try {
      const token = req.header(PAYMENTS_ACCESS_TOKEN_HEADER);
      if(token == null) throw access_token_error("access-token-not-present");
      else if (token !== access_token) throw access_token_error("access-token-mismatch");
      next();
    } catch(e) {
      next(e);
    }
  }
};

export const payments_api = ({ config, logger: _logger }: { config: Exclude<Config["payments"], undefined> , logger: Logger }) => {
  const logger = _logger.scoped("payments");

  const access_token = config.access_token;

  const client = new BraintreePaymentsClient(config.credentials);
  
  const api = Router();

  api.use(access_token_auth(access_token));

  api.use(body_parser.json());

  add_all(api, client, validate_rethrow);

  api.use(not_found_handler)

  api.use(catch_handler({ logger }));

  return api;
}