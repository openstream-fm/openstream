use env_logger::Target;

pub fn init() {
  let mut logger = env_logger::Builder::from_default_env();
  logger.target(Target::Stdout);
  logger.init();
}
