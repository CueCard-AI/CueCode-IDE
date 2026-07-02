use cuecode_chp::{message_type, ChpEnvelope};
use futures::StreamExt;
use url::Url;

pub struct ChpClient {
    stream: async_tungstenite::WebSocketStream<async_tungstenite::tokio::ConnectStream>,
}

impl ChpClient {
    pub async fn connect(harness_url: &str) -> anyhow::Result<Self> {
        let url = Url::parse(harness_url)?;
        let (stream, _) = async_tungstenite::tokio::connect_async(url.as_str()).await?;
        Ok(Self { stream })
    }

    pub async fn send(&mut self, envelope: &ChpEnvelope) -> anyhow::Result<()> {
        let text = envelope.to_json()?;
        self.stream
            .send(async_tungstenite::tungstenite::Message::Text(text.into()))
            .await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<ChpEnvelope> {
        loop {
            let message = self
                .stream
                .next()
                .await
                .ok_or_else(|| anyhow::anyhow!("websocket closed"))??;
            match message {
                async_tungstenite::tungstenite::Message::Text(text) => {
                    return ChpEnvelope::from_json(&text);
                }
                async_tungstenite::tungstenite::Message::Ping(payload) => {
                    self.stream
                        .send(async_tungstenite::tungstenite::Message::Pong(payload))
                        .await?;
                }
                async_tungstenite::tungstenite::Message::Close(_) => {
                    anyhow::bail!("websocket closed by server");
                }
                _ => {}
            }
        }
    }
}

pub async fn run_m0_roundtrip(harness_url: &str) -> anyhow::Result<()> {
    let mut client = ChpClient::connect(harness_url).await?;

    client
        .send(&cuecode_chp::session_start("req_m0_1", &["/tmp/project"]))
        .await?;
    let started = client.recv().await?;
    if started.msg_type != message_type::SESSION_STARTED {
        anyhow::bail!("expected session.started, got {}", started.msg_type);
    }
    let session_id = started
        .session_id
        .clone()
        .ok_or_else(|| anyhow::anyhow!("missing session_id"))?;

    let turn = cuecode_chp::turn_start(
        &session_id,
        "req_turn_1",
        "umsg_m0",
        "Read README.md",
    );
    client.send(&turn).await?;

    let tool_call_id = loop {
        let message = client.recv().await?;
        if message.msg_type == message_type::TOOL_REQUEST {
            break message.payload["tool_call_id"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("tool.request missing tool_call_id"))?
                .to_string();
        }
    };

    client
        .send(&cuecode_chp::tool_result_ok(
            &session_id,
            "req_tool_1",
            &tool_call_id,
            "# CueCode\n",
        ))
        .await?;

    let end = client.recv().await?;
    if end.msg_type != message_type::TURN_END {
        anyhow::bail!("expected turn.end, got {}", end.msg_type);
    }

    Ok(())
}
