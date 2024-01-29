pub mod post {
  use std::net::IpAddr;

  use async_trait::async_trait;
  use constants::validate::*;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::account::{Account, Limit, Limits, PublicAccount};
  use db::email_verification_code::EmailVerificationCode;
  use db::metadata::Metadata;
  // TODO: payments
  // use db::payment_method::{PaymentMethod, PaymentMethodKind};
  // use payments::query::save_payment_method::SavePaymentMethodResponse;
  use db::plan::Plan;
  use db::user::{PublicUser, User};
  use db::user_account_relation::{UserAccountRelation, UserAccountRelationKind};
  use db::{run_transaction, Model};
  use modify::Modify;
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use ts_rs::TS;
  use user_agent::{UserAgent, UserAgentExt};
  use validate::email::is_valid_email;
  use validator::Validate;

  use crate::error::ApiError;
  use crate::json::JsonHandler;
  use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(mongodb::error::Error),
    #[error("account name is empty")]
    AccountNameEmpty,
    #[error("first name is empty")]
    FirstNameEmpty,
    #[error("last name is empty")]
    LastNameEmpty,
    #[error("email is empty")]
    EmailEmpty,
    #[error("email is invalid")]
    EmailInvalid,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("email already exists")]
    EmailExists,
    #[error("email is too long")]
    EmailTooLong,
    #[error("first name is too long")]
    FirstNameTooLong,
    #[error("last name is too long")]
    LastNameTooLong,
    #[error("phone is too long")]
    PhoneTooLong,
    #[error("account name is too long")]
    AccountNameTooLong,
    #[error("password too long")]
    PasswordTooLong,
    #[error("language too long")]
    LanguageTooLong,
    #[error("device id invalid")]
    DeviceIdInvalid,
    #[error("plan not found: {0}")]
    PlanNotFound(String),
    #[error("email verification code mismatch")]
    EmailCodeMismatch,
    #[error("email verification code expired")]
    EmailCodeExpired,

    #[error("payments_ensure_method: {0}")]
    PaymentsEnsureCustomer(#[source] payments::error::PerformError),
    #[error("payments_save_payment_method: {0}")]
    PaymentSavePaymentMethod(#[source] payments::error::PerformError),
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::AccountNameEmpty => {
          ApiError::PayloadInvalid(String::from("Account name is required"))
        }
        HandleError::EmailEmpty => ApiError::PayloadInvalid(String::from("Email is required")),
        HandleError::FirstNameEmpty => {
          ApiError::PayloadInvalid(String::from("First name is required"))
        }
        HandleError::LastNameEmpty => {
          ApiError::PayloadInvalid(String::from("Last name is required"))
        }
        HandleError::EmailInvalid => ApiError::PayloadInvalid(String::from("Email is invalid")),
        HandleError::PasswordTooShort => {
          ApiError::PayloadInvalid(String::from("Password must have 8 characters or more"))
        }
        HandleError::EmailExists => ApiError::UserEmailExists,
        HandleError::FirstNameTooLong => {
          ApiError::PayloadInvalid(String::from("First name must be of 50 characters or less"))
        }
        HandleError::LanguageTooLong => {
          ApiError::PayloadInvalid(String::from("Language must be of 10 characters or less"))
        }
        HandleError::LastNameTooLong => {
          ApiError::PayloadInvalid(String::from("Last name must be of 50 characters or less"))
        }
        HandleError::AccountNameTooLong => ApiError::PayloadInvalid(String::from(
          "Account name must be of 30 characters or less",
        )),
        HandleError::PhoneTooLong => {
          ApiError::PayloadInvalid(String::from("Phone must be of 20 characters or less"))
        }
        HandleError::EmailTooLong => {
          ApiError::PayloadInvalid(String::from("Email must be of 40 characters or less"))
        }
        HandleError::PasswordTooLong => {
          ApiError::PayloadInvalid(String::from("Password must be of 80 characters or less"))
        }
        HandleError::DeviceIdInvalid => {
          ApiError::PayloadInvalid(String::from("device_id is invalid"))
        }
        HandleError::PlanNotFound(id) => {
          ApiError::PayloadInvalid(format!("Plan with id {id} not found"))
        }
        HandleError::EmailCodeMismatch => {
          ApiError::BadRequestCustom("Email verification code doesn't match".into())
        }
        HandleError::EmailCodeExpired => {
          ApiError::BadRequestCustom("Email verification code has expired".into())
        }
        HandleError::PaymentsEnsureCustomer(e) | HandleError::PaymentSavePaymentMethod(e) => {
          e.into()
        }
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/auth/user/register/POST/")]
  #[macros::schema_ts_export]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_USER_FIRST_NAME_MAX_LEN",
        message = "First name is either too short or too long"
      ),
      non_control_character(message = "First name contains invalid characters")
    )]
    first_name: String,

    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_USER_LAST_NAME_MAX_LEN",
        message = "Last name is either too short or too long"
      ),
      non_control_character(message = "Last name contains invalid characters")
    )]
    last_name: String,

    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_ACCOUNT_NAME_MAX_LEN",
        message = "Account name is either too short or too long"
      ),
      non_control_character(message = "Account name contains invalid characters")
    )]
    account_name: String,

    plan_id: String,

    #[modify(trim)]
    #[validate(
      email(message = "Email is invalid"),
      length(
        min = 1,
        max = "VALIDATE_USER_EMAIL_MAX_LEN",
        message = "Email is either too short or too long"
      ),
      non_control_character(message = "Email contains invalid characters")
    )]
    email: String,

    #[validate(length(
      min = "VALIDATE_USER_PASSWORD_MIN_LEN",
      max = "VALIDATE_USER_PASSWORD_MAX_LEN",
      message = "Password is either too short or too long"
    ))]
    password: String,

    #[modify(trim)]
    #[validate(
      phone(message = "Phone is invalid"),
      length(max = "VALIDATE_USER_PHONE_MAX_LEN", message = "Phone is too long"),
      non_control_character(message = "Phone name contains invalid characters")
    )]
    phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      length(
        max = "VALIDATE_USER_LANGUAGE_MAX_LEN",
        message = "Language is too long"
      ),
      non_control_character(message = "Language contains invalid characters")
    )]
    language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_system_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    account_user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    account_system_metadata: Option<Metadata>,

    email_verification_code: String,

    device_id: String,
    // TODO: payments
    // payment_method_nonce: String,
    // payment_device_data: Option<String>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    pub ip: IpAddr,
    pub user_agent: UserAgent,
    pub access_token_scope: Option<AccessTokenScope>,
    pub payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/auth/user/register/POST/")]
  #[macros::schema_ts_export]
  pub struct Output {
    pub user: PublicUser,
    pub account: PublicAccount,
    pub token: String,
    pub media_key: String,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub payments_client: payments::PaymentsClient,
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let ip = req.isomorphic_ip();
      let user_agent = req.parse_ua();
      let access_token_scope = request_ext::get_optional_access_token_scope(&req).await?;
      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input {
        ip,
        user_agent,
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        ip,
        user_agent,
        access_token_scope,
        payload,
      } = input;

      let Payload {
        plan_id,
        email,
        password,
        phone,
        language,
        first_name,
        last_name,
        account_name,
        account_user_metadata,
        account_system_metadata,
        user_user_metadata,
        user_system_metadata,
        email_verification_code,
        device_id,
        // TODO: payments
        // payment_method_nonce,
        // payment_device_data,
      } = payload;

      if !AccessToken::is_device_id_valid(&device_id) {
        return Err(HandleError::DeviceIdInvalid);
      }

      let email = email.trim().to_lowercase();
      let first_name = first_name.trim().to_string();
      let last_name = last_name.trim().to_string();
      let account_name = account_name.trim().to_string();

      let phone = match phone {
        None => None,
        Some(phone) => match phone.trim() {
          "" => None,
          phone => Some(phone.to_string()),
        },
      };

      let language = match language {
        None => None,
        Some(lang) => match lang.trim() {
          "" => None,
          lang => Some(lang.to_string()),
        },
      };

      let account_user_metadata = account_user_metadata.unwrap_or_default();
      let account_system_metadata = account_system_metadata.unwrap_or_default();
      let user_user_metadata = user_user_metadata.unwrap_or_default();
      let user_system_metadata = user_system_metadata.unwrap_or_default();

      if email.is_empty() {
        return Err(HandleError::EmailEmpty);
      }

      if !is_valid_email(&email) {
        return Err(HandleError::EmailInvalid);
      }

      let email_exists = User::email_exists(&email).await?;
      if email_exists {
        return Err(HandleError::EmailExists);
      }

      let verification_code_document = {
        let hash = crypt::sha256(&email_verification_code);
        let filter = doc! {
          EmailVerificationCode::KEY_EMAIL: &email,
          EmailVerificationCode::KEY_HASH: hash,
        };

        match EmailVerificationCode::get(filter).await? {
          None => return Err(HandleError::EmailCodeMismatch),
          Some(doc) => {
            if doc.is_expired() {
              return Err(HandleError::EmailCodeExpired);
            }
            doc
          }
        }
      };

      if first_name.is_empty() {
        return Err(HandleError::FirstNameEmpty);
      }

      if last_name.is_empty() {
        return Err(HandleError::LastNameEmpty);
      }

      if account_name.is_empty() {
        return Err(HandleError::AccountNameEmpty);
      }

      if password.len() < 8 {
        return Err(HandleError::PasswordTooShort);
      }

      if password.len() > 80 {
        return Err(HandleError::PasswordTooLong);
      }

      if first_name.len() > 50 {
        return Err(HandleError::FirstNameTooLong);
      }

      if last_name.len() > 50 {
        return Err(HandleError::FirstNameTooLong);
      }

      if email.len() > 40 {
        return Err(HandleError::EmailTooLong);
      }

      if account_name.len() > 40 {
        return Err(HandleError::AccountNameTooLong);
      }

      if let Some(ref phone) = phone {
        if phone.len() > 20 {
          return Err(HandleError::PhoneTooLong);
        }
      }

      if let Some(ref lang) = language {
        if lang.len() > 10 {
          return Err(HandleError::LanguageTooLong);
        }
      }

      let plan = match Plan::get_by_id(&plan_id).await? {
        Some(plan) => plan,
        None => return Err(HandleError::PlanNotFound(plan_id)),
      };

      if plan.deleted_at.is_some() || !plan.is_user_selectable {
        return Err(HandleError::PlanNotFound(plan_id));
      }

      let email_exists = User::email_exists(&email).await?;
      if email_exists {
        return Err(HandleError::EmailExists);
      }

      let user_id = User::uid();

      // TODO: payments
      // let customer_id = {
      //   let query = payments::query::ensure_customer::EnsureCustomer {
      //     customer_id: user_id.clone(),
      //     first_name: first_name.clone(),
      //     last_name: last_name.clone(),
      //     email: email.clone(),
      //   };

      //   let res = self
      //     .payments_client
      //     .perform(query)
      //     .await
      //     .map_err(HandleError::PaymentsEnsureCustomer)?;

      //   res.customer_id
      // };

      // let payment_method_response = {
      //   let query = payments::query::save_payment_method::SavePaymentMethod {
      //     customer_id,
      //     payment_method_nonce,
      //     device_data: payment_device_data,
      //   };

      //   let payment_method_response = self
      //     .payments_client
      //     .perform(query)
      //     .await
      //     .map_err(HandleError::PaymentSavePaymentMethod)?;

      //   #[allow(clippy::let_and_return)]
      //   payment_method_response
      // };
      // let payment_method_id = PaymentMethod::uid();

      let password = crypt::hash(password);

      let now = DateTime::now();

      let user = User {
        id: user_id.clone(),
        email,
        phone,
        first_name,
        last_name,
        password: Some(password),
        language: None,
        user_metadata: user_user_metadata,
        system_metadata: user_system_metadata,
        created_at: now,
        updated_at: now,
        deleted_at: None,
      };

      let limits = Limits {
        stations: Limit {
          total: plan.limits.stations,
          used: 0,
        },
        listeners: Limit {
          total: plan.limits.listeners,
          used: 0,
        },
        transfer: Limit {
          total: plan.limits.transfer,
          used: 0,
        },
        storage: Limit {
          total: plan.limits.storage,
          used: 0,
        },
      };

      let account = Account {
        id: Account::uid(),
        plan_id,
        // TODO: payments
        // payment_method_id: Some(payment_method_id.clone()),
        payment_method_id: None,
        limits,
        name: account_name,
        user_metadata: account_user_metadata,
        system_metadata: account_system_metadata,
        created_at: now,
        updated_at: now,
        deleted_at: None,
      };

      let relation = UserAccountRelation {
        id: UserAccountRelation::uid(),
        user_id: user.id.clone(),
        account_id: account.id.clone(),
        kind: UserAccountRelationKind::Owner,
        created_at: now,
      };

      let key = AccessToken::random_key();
      let media_key = AccessToken::random_media_key();

      let token = AccessToken {
        id: AccessToken::uid(),
        hash: crypt::sha256(&key),
        media_hash: crypt::sha256(&media_key),
        scope: Scope::User {
          user_id: user.id.clone(),
        },
        generated_by: GeneratedBy::Register {
          ip,
          user_agent,
          device_id,
        },
        last_used_at: None,
        hits: 0,
        created_at: now,
        deleted_at: None,
      };

      // TODO: payments
      // let payment_method = {
      //   let SavePaymentMethodResponse {
      //     payment_method_token,
      //     card_type,
      //     last_4,
      //     expiration_month,
      //     expiration_year,
      //   } = payment_method_response;

      //   PaymentMethod {
      //     id: payment_method_id,
      //     user_id,
      //     kind: PaymentMethodKind::Card {
      //       token: payment_method_token,
      //       card_type,
      //       last_4,
      //       expiration_month,
      //       expiration_year,
      //     },
      //     created_at: now,
      //     updated_at: now,
      //     deleted_at: None,
      //   }
      // };

      run_transaction!(session => {
        let email_exists = tx_try!(User::email_exists_with_session(user.email.as_str(), &mut session).await);
        if email_exists {
          return Err(HandleError::EmailExists)
        }

        tx_try!(User::insert_with_session(&user, &mut session).await);
        tx_try!(Account::insert_with_session(&account, &mut session).await);
        tx_try!(UserAccountRelation::insert_with_session(&relation, &mut session).await);
        tx_try!(AccessToken::insert_with_session(&token, &mut session).await);
        // TODO: payments
        // tx_try!(PaymentMethod::insert_with_session(&payment_method, &mut session).await);
        tx_try!(EmailVerificationCode::update_by_id_with_session(&verification_code_document.id, doc! { "$set": { EmailVerificationCode::KEY_USED_AT: now } }, &mut session).await)
      });

      let public_scope = match access_token_scope {
        Some(token) => token.as_public_scope(),
        None => db::PublicScope::User,
      };

      let out = Output {
        user: user.into_public(public_scope),
        account: account.into_public(public_scope),
        token: format!("{}-{}", token.id, key),
        media_key: format!("{}-{}", token.id, media_key),
      };

      Ok(out)
    }
  }
}
