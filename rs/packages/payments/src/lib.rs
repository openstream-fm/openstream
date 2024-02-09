use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

pub mod client;
pub mod error;
pub mod query;
pub use client::PaymentsClient;

pub trait Query: Send + Sync + Serialize + DeserializeOwned + ts_rs::TS + 'static {
  const PATH: &'static str;
  type Response: Send + Sync + Serialize + DeserializeOwned + ts_rs::TS + 'static;
}

#[macro_export]
macro_rules! export {
  ($type:ty) => {
    #[cfg(test)]
    #[test]
    fn export_path() {
      $crate::export_query::<$type>();
    }
  };
}

pub fn export_query<T: Query>() {
  use ts_rs::TS;

  let base = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../../defs/payments/api")
    .join(T::PATH.trim_start_matches('/'));

  std::fs::create_dir_all(&base).expect("create_dir_all()");

  T::export_to(base.join("Query.ts")).unwrap();
  T::Response::export_to(base.join("Response.ts")).unwrap();

  // path
  {
    let type_and_value = serde_json::to_string(T::PATH).expect("json::to_string");

    let contents = [
      String::from(
        "// This file was automatically generated from its Rust definition, do not manually edit",
      ),
      String::from(""),
      format!("export type Path = {type_and_value};"),
      String::from(""),
      format!("export const path: Path = {type_and_value};"),
    ]
    .join("\n");

    std::fs::write(base.join("Path.ts"), contents).expect("std::fs::write");
  };

  // index
  {
    let contents = [
      String::from(
        "// This file was automatically generated from its Rust definition, do not manually edit",
      ),
      String::from(""),
      String::from(r#"export { path } from "./Path.js" "#),
      String::from(r#"export type { Path } from "./Path.js" "#),
      String::from(r#"export type { Query } from "./Query.js" "#),
      String::from(r#"export type { Response } from "./Response.js" "#),
    ]
    .join("\n");

    std::fs::write(base.join("endpoint.ts"), contents).expect("std::fs::write");
  }
}

#[cfg(test)]
mod test {
  use super::Query;
  use crate::query::ensure_customer::EnsureCustomer;
  use crate::query::generate_client_token::GenerateClientToken;
  use crate::query::save_payment_method::SavePaymentMethod;
  use ts_rs::TS;

  fn interface_endpoint_import<T: Query>() -> String {
    let name = T::PATH.trim_matches('/').replace('-', "_");
    format!(
      r#"import type * as {} from "./{}/endpoint.js""#,
      name,
      T::PATH.trim_start_matches('/')
    )
  }

  fn interface_endpoint_fn<T: Query>() -> String {
    let name = T::PATH.trim_matches('/').replace('-', "_");
    format!("  {name}(query: {name}.Query): Promise<{name}.Response>;")
  }

  macro_rules! export_client {
    ($($type:ty)*) => {
      let manifest = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
      let router_target = manifest.join("../../../defs/payments/api/router.ts");
      let interface_target = manifest.join("../../../defs/payments/api/payments-client.ts");

      std::fs::create_dir_all(manifest.join("../../../defs/payments/api")).expect("std::fs::create_dir_all()");

      let mut contents = vec![
        String::from(
          "// This file was automatically generated from its Rust definition, do not manually edit",
        ),
        String::from(""),
      ];

      $(
        contents.push(interface_endpoint_import::<$type>());
      )*

      contents.push(String::from(""));

      contents.push(String::from("export interface PaymentsClient {"));

      $(
        contents.push(interface_endpoint_fn::<$type>());
      )*

      contents.push(String::from("}"));

      std::fs::write(interface_target, contents.join("\n")).expect("std::fs::write");

      let mut contents = vec![
        String::from("// This file was automatically generated from its Rust definition, do not manually edit"),
        String::from("// @ts-ignore"),
        String::from(r#"import typia from "typia""#),
        String::from(r#"import type { PaymentsClient } from "./payments-client.js""#),
      ];

      $(
        contents.push(interface_endpoint_import::<$type>());
      )*

      contents.append(&mut vec![
        String::from(""),
        String::from("type Request = { body: unknown };"),
        String::from("type Response = { json: (object: any) => void };"),
        String::from("type Next = (e?: any) => void;"),
        String::from("type Router = { post: (path: string, fn: (req: Request, res: Response, next: Next) => void) => void };"),
        String::from("type ValidateRethrow = <T>(fn: () => T) => T;"),
        String::from(""),
        String::from("const handler = <T>(fn: (req: Request) => Promise<T>) => {"),
        String::from("  return async (req: Request, res: Response, next: Next) => {"),
        String::from("    try {"),
        String::from("      const output = await fn(req);"),
        String::from("      res.json(output);"),
        String::from("    } catch(e: any) {"),
        String::from("      next(e)"),
        String::from("    }"),
        String::from("  }"),
        String::from("}"),

        String::from("export const add_all = (router: Router, client: PaymentsClient, validate_rethrow: ValidateRethrow) => {")
      ]);

      $(

        let fn_name = <$type>::PATH.trim_start_matches('/').replace('-', "_");
        let path = serde_json::to_string(<$type>::PATH).unwrap();
        contents.push(format!("  router.post({}, handler(async req => {{", path));
        contents.push(format!("   const payload = validate_rethrow(() => typia.assertEquals<{}.Query>(req.body));", fn_name));
        contents.push(format!("   return await client.{}(payload)", fn_name));
        contents.push(format!("  }}))"));
        contents.push(format!(""));
      )*

      contents.push(String::from("}"));

      std::fs::write(router_target, contents.join("\n")).expect("std::fs::write");

    }
  }

  #[test]
  fn export_client_interface() {
    export_client!(
      GenerateClientToken
      EnsureCustomer
      SavePaymentMethod
    );
  }

  #[test]
  fn export_error() {
    let manifest = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let base = manifest.join("../../../defs/payments/api");

    crate::error::PaymentsErrorPayload::export_to(
      base.join(format!("{}.ts", crate::error::PaymentsErrorPayload::name())),
    )
    .unwrap();

    crate::error::PaymentsError::export_to(
      base.join(format!("{}.ts", crate::error::PaymentsError::name())),
    )
    .unwrap();

    crate::error::PaymentsErrorKind::export_to(
      base.join(format!("{}.ts", crate::error::PaymentsErrorKind::name())),
    )
    .unwrap();
  }
}
