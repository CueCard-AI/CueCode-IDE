use std::env;

pub const DEFAULT_HARNESS_URL: &str = "ws://127.0.0.1:8787/v1/chp/connect";

pub fn harness_url_from_env() -> String {
    env::var("CUECODE_HARNESS_URL").unwrap_or_else(|_| DEFAULT_HARNESS_URL.to_string())
}

pub fn agent_runtime_from_env() -> AgentRuntime {
    match env::var("CUECODE_AGENT_RUNTIME")
        .unwrap_or_else(|_| "local".to_string())
        .to_lowercase()
        .as_str()
    {
        "cloud" | "byok_cloud" => AgentRuntime::Cloud,
        _ => AgentRuntime::Local,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentRuntime {
    Local,
    Cloud,
}
