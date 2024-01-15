mod traits;

pub use modify_derive::Modify;

/// Modifies the struct based on the provided `modify` parameters. Automatically implemented when deriving modify.
/// See the [repository](https://github.com/biblius/modify) for a full list of possible modifiers.
pub trait Modify {
  /// Apply the provided modifiers to self
  fn modify(&mut self);
}
