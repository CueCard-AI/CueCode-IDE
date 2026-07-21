mod envelope;
pub mod error;
pub mod message_type;
pub mod messages;

pub use envelope::{ChpEnvelope, CHP_VERSION, CHP_WS_SUBPROTOCOL};
pub use error::ChpErrorCode;
pub use message_type::*;
pub use messages::*;

pub fn fixtures_dir() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures")
}
