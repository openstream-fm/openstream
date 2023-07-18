use crate::auth::AccessScope;

pub struct GetAccountOperationParams {
  pub account: String,
}

pub struct GetAccountOperation {
  pub access_scope: AccessScope,
  pub params: GetAccountOperationParams,
}

pub struct GetAccountOperationOutput {
  pub account: crate::public::account::PublicAccount,
}
