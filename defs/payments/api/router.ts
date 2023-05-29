// This file was automatically generated from its Rust definition, do not manually edit
// @ts-ignore
import typia from "typia"
import { PaymentsClient } from "./payments-client"
import type * as generate_client_token from "./generate-client-token/endpoint"
import type * as ensure_customer from "./ensure-customer/endpoint"
import type * as save_payment_method from "./save-payment-method/endpoint"

type Request = { body: unknown };
type Response = { json: (object: any) => void };
type Next = (e?: any) => void;
type Router = { post: (path: string, fn: (req: Request, res: Response, next: Next) => void) => void };
type ValidateRethrow = <T>(fn: () => T) => T;

const handler = <T>(fn: (req: Request) => Promise<T>) => {
  return async (req: Request, res: Response, next: Next) => {
    try {
      const output = await fn(req);
      res.json(output);
    } catch(e: any) {
      next(e)
    }
  }
}
export const add_all = (router: Router, client: PaymentsClient, validate_rethrow: ValidateRethrow) => {
  router.post("/generate-client-token", handler(async req => {
   const payload = validate_rethrow(() => typia.assertEquals<generate_client_token.Query>(req.body));
   return await client.generate_client_token(payload)
  }))

  router.post("/ensure-customer", handler(async req => {
   const payload = validate_rethrow(() => typia.assertEquals<ensure_customer.Query>(req.body));
   return await client.ensure_customer(payload)
  }))

  router.post("/save-payment-method", handler(async req => {
   const payload = validate_rethrow(() => typia.assertEquals<save_payment_method.Query>(req.body));
   return await client.save_payment_method(payload)
  }))

}