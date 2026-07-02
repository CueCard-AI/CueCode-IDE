use serde::{Deserialize, Serialize};

pub const CHP_VERSION: &str = "1.0";
pub const CHP_WS_SUBPROTOCOL: &str = "cuecode-harness.chp.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChpEnvelope {
    pub chp_version: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lane_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    pub payload: serde_json::Value,
}

impl ChpEnvelope {
    pub fn new(msg_type: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            chp_version: CHP_VERSION.to_string(),
            msg_type: msg_type.into(),
            session_id: None,
            seq: None,
            request_id: None,
            lane_id: None,
            timestamp: None,
            payload,
        }
    }

    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn with_seq(mut self, seq: u64) -> Self {
        self.seq = Some(seq);
        self
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json(text: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(text)?)
    }
}
