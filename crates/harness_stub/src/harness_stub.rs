mod engine;
mod server;

pub use engine::StubEngine;
pub use server::run_server;

pub fn default_addr() -> std::net::SocketAddr {
    let port = std::env::var("HARNESS_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8787);
    std::net::SocketAddr::from(([0, 0, 0, 0], port))
}
