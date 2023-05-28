// This file was automatically generated from its Rust definition, do not manually edit

import type * as ensure_customer from "./ensure-customer/endpoint"
import type * as generate_client_token from "./generate-client-token/endpoint"

export interface PaymentsClient {
  ensure_customer(query: ensure_customer.Query): Promise<ensure_customer.Response>;
  generate_client_token(query: generate_client_token.Query): Promise<generate_client_token.Response>;
}