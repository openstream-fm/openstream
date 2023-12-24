pub mod connection;
use prex::router::builder::Builder;
use shutdown::Shutdown;

pub fn router(deployment_id: String, shutdown: Shutdown) -> Builder {
  let mut router = prex::prex();

  router.get(
    "/ws/stats/connection",
    connection::WsConnectionHandler {
      deployment_id: deployment_id.clone(),
      shutdown: shutdown.clone(),
    },
  );

  router
}
