use cuecode_chp::{
    message_type, session_started, tool_request_read_file, turn_end, turn_stream_chunk, ChpEnvelope,
    CHP_VERSION,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SessionPhase {
    Idle,
    AwaitingToolResult {
        user_message_id: String,
        tool_call_id: String,
    },
}

#[derive(Debug, Default)]
pub struct StubEngine {
    sessions: HashMap<String, SessionPhase>,
    next_seq: u64,
}

impl StubEngine {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::default(),
            next_seq: 2,
        }
    }

    pub fn handle(&mut self, envelope: &ChpEnvelope) -> anyhow::Result<Vec<ChpEnvelope>> {
        if envelope.chp_version != CHP_VERSION {
            anyhow::bail!("unsupported chp_version {}", envelope.chp_version);
        }

        match envelope.msg_type.as_str() {
            message_type::SESSION_START => self.handle_session_start(envelope),
            message_type::TURN_START => self.handle_turn_start(envelope),
            message_type::TOOL_RESULT => self.handle_tool_result(envelope),
            message_type::KEEPALIVE => Ok(vec![]),
            other => anyhow::bail!("unexpected message type in M0 stub: {other}"),
        }
    }

    fn handle_session_start(&mut self, envelope: &ChpEnvelope) -> anyhow::Result<Vec<ChpEnvelope>> {
        let session_id = format!("sess_{}", uuid::Uuid::new_v4());
        self.sessions.insert(session_id.clone(), SessionPhase::Idle);
        let request_id = envelope
            .request_id
            .clone()
            .unwrap_or_else(|| "req_start".to_string());
        Ok(vec![session_started(&session_id, &request_id)])
    }

    fn handle_turn_start(&mut self, envelope: &ChpEnvelope) -> anyhow::Result<Vec<ChpEnvelope>> {
        let session_id = envelope
            .session_id
            .clone()
            .ok_or_else(|| anyhow::anyhow!("turn.start missing session_id"))?;
        if !self.sessions.contains_key(&session_id) {
            anyhow::bail!("unknown session {session_id}");
        }

        let user_message_id = envelope
            .payload
            .get("user_message_id")
            .and_then(|value| value.as_str())
            .unwrap_or("umsg_stub")
            .to_string();

        let tool_call_id = format!("tc_{}", uuid::Uuid::new_v4());
        let seq_stream = self.next_seq();
        let seq_tool = self.next_seq();

        self.sessions.insert(
            session_id.clone(),
            SessionPhase::AwaitingToolResult {
                user_message_id: user_message_id.clone(),
                tool_call_id: tool_call_id.clone(),
            },
        );

        Ok(vec![
            turn_stream_chunk(
                &session_id,
                seq_stream,
                &user_message_id,
                "I'll read README.md for you.",
            ),
            tool_request_read_file(&session_id, seq_tool, &tool_call_id, "README.md"),
        ])
    }

    fn handle_tool_result(&mut self, envelope: &ChpEnvelope) -> anyhow::Result<Vec<ChpEnvelope>> {
        let session_id = envelope
            .session_id
            .clone()
            .ok_or_else(|| anyhow::anyhow!("tool.result missing session_id"))?;

        let phase = self
            .sessions
            .get(&session_id)
            .ok_or_else(|| anyhow::anyhow!("unknown session {session_id}"))?;

        let SessionPhase::AwaitingToolResult { user_message_id, .. } = phase.clone() else {
            anyhow::bail!("tool.result unexpected in phase {:?}", phase);
        };

        self.sessions.insert(session_id.clone(), SessionPhase::Idle);
        let seq = self.next_seq();
        Ok(vec![turn_end(
            &session_id,
            seq,
            &user_message_id,
        )])
    }

    fn next_seq(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cuecode_chp::session_start;

    #[test]
    fn m0_message_sequence() {
        let mut engine = StubEngine::new();
        let started = engine
            .handle(&session_start("req_1", &["/tmp/project"]))
            .expect("session.start");
        assert_eq!(started.len(), 1);
        assert_eq!(started[0].msg_type, message_type::SESSION_STARTED);
        let session_id = started[0].session_id.clone().expect("session id");

        let turn = ChpEnvelope::new(
            message_type::TURN_START,
            serde_json::json!({
                "user_message_id": "umsg_1",
                "content": [{ "type": "text", "text": "read readme" }]
            }),
        )
        .with_session_id(session_id.clone());

        let mid = engine.handle(&turn).expect("turn.start");
        assert_eq!(mid.len(), 2);
        assert_eq!(mid[1].msg_type, message_type::TOOL_REQUEST);

        let tool_call_id = mid[1].payload["tool_call_id"]
            .as_str()
            .expect("tool call id")
            .to_string();

        let tool_result = ChpEnvelope::new(
            message_type::TOOL_RESULT,
            serde_json::json!({
                "tool_call_id": tool_call_id,
                "success": true,
                "content": "# hello"
            }),
        )
        .with_session_id(session_id)
        .with_request_id("req_tool");

        let end = engine.handle(&tool_result).expect("tool.result");
        assert_eq!(end.len(), 1);
        assert_eq!(end[0].msg_type, message_type::TURN_END);
    }
}
