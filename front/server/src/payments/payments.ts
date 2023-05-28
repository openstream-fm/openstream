import { Router } from "express"
import { Config } from "../config";
import { Logger } from "../logger";
import braintree from "braintree";
import { assert_never } from "../assert_never";

export const payments_api = ({ config, logger: _logger }: { config: Exclude<Config["payments"], undefined> , logger: Logger }) => {
  const logger = _logger.scoped("payments");

  const {
    credentials,
    access_token: _access_token,
  } = config;
 
  let environment: braintree.Environment;
  if(credentials.environment === "sandbox") {
    environment = braintree.Environment.Sandbox;
  } else if (credentials.environment === "production") {
    environment = braintree.Environment.Production;
  } else {
    return assert_never(credentials.environment, "payments_api: config.credentials.environment");
  }

  const client = new braintree.BraintreeGateway({
    environment,
    merchantId: credentials.merchant_id,
    publicKey: credentials.public_key,
    privateKey: credentials.private_key,
  });


  const api = Router();
  return api;
}