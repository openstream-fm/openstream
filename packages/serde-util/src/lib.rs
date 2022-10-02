pub mod number;
pub use number::*;
pub mod datetime;
pub mod bytes;
pub mod password;
pub mod voidable;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
