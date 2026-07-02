use cuecode_cloud::run_m0_roundtrip;
use harness_stub::{default_addr, run_server};
use std::time::Duration;

#[tokio::test]
async fn m0_roundtrip_against_stub_server() {
    let addr = default_addr();
    let server = tokio::spawn(async move {
        run_server(addr).await.expect("stub server");
    });
    tokio::time::sleep(Duration::from_millis(200)).await;

    let harness_url = format!("ws://{addr}/v1/chp/connect");
    run_m0_roundtrip(&harness_url)
        .await
        .expect("m0 roundtrip");

    server.abort();
}
