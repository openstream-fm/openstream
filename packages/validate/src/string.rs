use crate::error::ValidationError;

pub trait ValidateString: Sized {
  fn validate_string(self, params: ValidateStringParams) -> Result<Self, ValidationError>;
}

pub struct ValidateStringParams {
  pub field: &'static str,
  pub trim: Option<bool>,
  pub minlen: Option<usize>,
  pub maxlen: Option<usize>,
}

impl ValidateString for String {
  fn validate_string(self, params: ValidateStringParams) -> Result<Self, ValidationError> {
    let mut string = self.as_str();
    if matches!(params.trim, None | Some(true)) {
      string = string.trim();
    }

    if let Some(min) = params.minlen {
      if string.chars().count() < min {
        return Err(ValidationError {
          field: params.field,
          message: format!("is too short, min length is {min}"),
        });
      }
    }

    if let Some(max) = params.maxlen {
      if string.chars().count() > max {
        return Err(ValidationError {
          field: params.field,
          message: format!("is too long, max length is {max}"),
        });
      }
    }

    Ok(string.to_string())
  }
}

impl ValidateString for Option<String> {
  fn validate_string(self, params: ValidateStringParams) -> Result<Self, ValidationError> {
    match self {
      None => Ok(None),
      Some(string) => match string.trim() {
        "" => Ok(None),
        string => Ok(Some(string.to_string().validate_string(params)?)),
      },
    }
  }
}

impl ValidateString for Option<Option<String>> {
  fn validate_string(self, params: ValidateStringParams) -> Result<Self, ValidationError> {
    match self {
      None => Ok(None),
      Some(opt) => Ok(Some(opt.validate_string(params)?)),
    }
  }
}
