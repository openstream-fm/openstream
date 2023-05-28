import braintree from "braintree";
import type { PaymentsClient } from "../defs/payments/api/payments-client";
import { assert_never } from "../assert_never";

export const map_environment = (kind: "sandbox" | "production"): braintree.Environment => {
  if(kind === "sandbox") {
    return braintree.Environment.Sandbox;
  } else if (kind === "production") {
    return braintree.Environment.Production;
  } else {
    return assert_never(kind, "Braintree environment kind");
  }
}

export class BraintreePaymentsClient implements PaymentsClient {
  
  // @ts-ignore
  gateway: braintree.BraintreeGateway;

  constructor(config: {
    environment: "sandbox" | "production"
    merchant_id: string
    public_key: string
    private_key: string
  }) {
    this.gateway = new braintree.BraintreeGateway({
      environment: map_environment(config.environment),
      merchantId: config.merchant_id,
      publicKey: config.public_key,
      privateKey: config.private_key,
    })
  }

  generate_client_token: PaymentsClient["generate_client_token"] = async (query) => {
    throw new Error("unimplemented");
  }

  ensure_customer: PaymentsClient["ensure_customer"] = async (query) => {
    throw new Error("unimplemented");
  }
}