use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub use test_macros::async_test;

#[allow(clippy::declare_interior_mutable_const)]
const TEST_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("error building test runtime")
});

pub fn test_runtime() -> Lazy<Runtime> {
  TEST_RUNTIME
}
