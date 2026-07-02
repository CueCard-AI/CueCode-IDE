use crate::envelope::{ChpEnvelope, CHP_VERSION};
use crate::message_type;
use serde_json::json;

pub fn session_start(request_id: &str, work_dirs: &[&str]) -> ChpEnvelope {
    ChpEnvelope::new(
        message_type::SESSION_START,
        json!({
            "work_dirs": work_dirs,
            "intent": "fix",
            "runtime_mode": "cloud",
            "client": {
                "app": "cuecode",
                "app_version": "0.1.0",
                "platform": "test"
            }
        }),
    )
    .with_request_id(request_id)
}

pub fn session_started(session_id: &str, request_id: &str) -> ChpEnvelope {
    ChpEnvelope::new(
        message_type::SESSION_STARTED,
        json!({
            "negotiated_version": CHP_VERSION,
            "capabilities": {
                "transports": ["websocket"],
                "supports_resume": true,
                "supports_spawn_lane": false,
                "max_tool_result_bytes": 262144
            },
            "session_id": session_id
        }),
    )
    .with_session_id(session_id)
    .with_request_id(request_id)
    .with_seq(1)
}

pub fn turn_start(session_id: &str, request_id: &str, user_message_id: &str, text: &str) -> ChpEnvelope {
    ChpEnvelope::new(
        message_type::TURN_START,
        json!({
            "user_message_id": user_message_id,
            "content": [{ "type": "text", "text": text }]
        }),
    )
    .with_session_id(session_id)
    .with_request_id(request_id)
}

pub fn turn_stream_chunk(
    session_id: &str,
    seq: u64,
    user_message_id: &str,
    text: &str,
) -> ChpEnvelope {
    ChpEnvelope::new(
        message_type::TURN_STREAM,
        json!({
            "user_message_id": user_message_id,
            "update": {
                "type": "assistant_message_chunk",
                "text": text
            }
        }),
    )
    .with_session_id(session_id)
    .with_seq(seq)
}

pub fn tool_request_read_file(
    session_id: &str,
    seq: u64,
    tool_call_id: &str,
    path: &str,
) -> ChpEnvelope {
    ChpEnvelope::new(
        message_type::TOOL_REQUEST,
        json!({
            "tool_call_id": tool_call_id,
            "name": "read_file",
            "arguments": {
                "path": path
            }
        }),
    )
    .with_session_id(session_id)
    .with_seq(seq)
}

pub fn tool_result_ok(
    session_id: &str,
    request_id: &str,
    tool_call_id: &str,
    content: &str,
) -> ChpEnvelope {
    ChpEnvelope::new(
        message_type::TOOL_RESULT,
        json!({
            "tool_call_id": tool_call_id,
            "success": true,
            "content": content
        }),
    )
    .with_session_id(session_id)
    .with_request_id(request_id)
}

pub fn turn_end(session_id: &str, seq: u64, user_message_id: &str) -> ChpEnvelope {
    ChpEnvelope::new(
        message_type::TURN_END,
        json!({
            "user_message_id": user_message_id,
            "stop_reason": "end_turn"
        }),
    )
    .with_session_id(session_id)
    .with_seq(seq)
}
