use email_address::EmailAddress;

pub fn is_valid_email(email: &str) -> bool {
  EmailAddress::is_valid(email)
}
