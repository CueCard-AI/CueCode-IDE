use harness_stub::default_addr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    harness_stub::run_server(default_addr()).await
}
