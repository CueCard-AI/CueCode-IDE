use cuecode_chp::{ChpEnvelope, fixtures_dir};
use pretty_assertions::assert_eq;
use std::fs;
use std::path::Path;

#[test]
fn fixture_session_start_roundtrip() {
    roundtrip_fixture("session_start.json");
}

#[test]
fn fixture_session_started_roundtrip() {
    roundtrip_fixture("session_started.json");
}

#[test]
fn fixture_turn_start_roundtrip() {
    roundtrip_fixture("turn_start.json");
}

#[test]
fn fixture_tool_request_roundtrip() {
    roundtrip_fixture("tool_request_read_file.json");
}

#[test]
fn fixture_tool_result_roundtrip() {
    roundtrip_fixture("tool_result_ok.json");
}

#[test]
fn fixture_turn_end_roundtrip() {
    roundtrip_fixture("turn_end.json");
}

fn roundtrip_fixture(name: &str) {
    let path = Path::new(fixtures_dir()).join(name);
    let text = fs::read_to_string(path).expect("read fixture");
    let envelope = ChpEnvelope::from_json(&text).expect("parse fixture");
    let again = ChpEnvelope::from_json(&envelope.to_json().expect("serialize")).expect("reparse");
    assert_eq!(envelope, again);
}
