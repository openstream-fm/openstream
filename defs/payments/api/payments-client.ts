// This file was automatically generated from its Rust definition, do not manually edit

import type * as generate_client_token from "./generate-client-token/endpoint"
import type * as ensure_customer from "./ensure-customer/endpoint"
import type * as save_payment_method from "./save-payment-method/endpoint"

export interface PaymentsClient {
  generate_client_token(query: generate_client_token.Query): Promise<generate_client_token.Response>;
  ensure_customer(query: ensure_customer.Query): Promise<ensure_customer.Response>;
  save_payment_method(query: save_payment_method.Query): Promise<save_payment_method.Response>;
}