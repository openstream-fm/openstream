pub mod connection;
use drop_tracer::DropTracer;
use prex::router::builder::Builder;
use shutdown::Shutdown;

pub fn router(deployment_id: String, drop_tracer: DropTracer, shutdown: Shutdown) -> Builder {
  let mut router = prex::prex();

  router.get(
    "/ws/stats/connection",
    connection::WsConnectionHandler {
      deployment_id,
      drop_tracer,
      shutdown,
    },
  );

  router
}
