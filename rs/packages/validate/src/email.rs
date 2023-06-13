use crate::error::ValidationError;
use once_cell::sync::Lazy;
use regex_static::{lazy_regex, Regex};

static MAIL_REGEX: Lazy<Regex> = lazy_regex!(
  r#"^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$"#
);
// r#"^\S+@\S+\.\S+$"#
// r"^[a-z0-9]([a-z0-9_\-\.\+]+)?@[a-z0-9]([a-z0-9_\-\.]*[a-z0-9])?\.[a-z0-9_\-\.]*[a-z0-9]$"
// r"^[a-z0-9]([a-z0-9\-_\.]+)?@[a-z0-9][a-z0-9_\-\.]+[a-z0-9]\.[a-z0-9\-_\.]{2,}[a-z0-9]$"

pub fn is_valid_email(address: &str) -> bool {
  MAIL_REGEX.is_match(address)
}

pub trait ValidateEmail: Sized {
  fn validate_email(self, params: ValidateEmailParams) -> Result<Self, ValidationError>;
}

pub struct ValidateEmailParams {
  field: &'static str,
  maxlen: Option<usize>,
}

impl ValidateEmail for String {
  fn validate_email(self, params: ValidateEmailParams) -> Result<Self, ValidationError> {
    let email = self.trim();
    if !is_valid_email(email) {
      return Err(ValidationError {
        field: params.field,
        message: String::from("is not a valid email address"),
      });
    }

    if let Some(max) = params.maxlen {
      if email.chars().count() > max {
        return Err(ValidationError {
          field: params.field,
          message: format!("is too long, max length is {max}"),
        });
      }
    }
    Ok(email.to_string())
  }
}

impl ValidateEmail for Option<String> {
  fn validate_email(self, params: ValidateEmailParams) -> Result<Self, ValidationError> {
    match self {
      None => Ok(None),
      Some(email) => match email.trim() {
        "" => Ok(None),
        email => Ok(Some(email.to_string().validate_email(params)?)),
      },
    }
  }
}

impl ValidateEmail for Option<Option<String>> {
  fn validate_email(self, params: ValidateEmailParams) -> Result<Self, ValidationError> {
    match self {
      None => Ok(None),
      Some(opt) => Ok(Some(opt.validate_email(params)?)),
    }
  }
}
