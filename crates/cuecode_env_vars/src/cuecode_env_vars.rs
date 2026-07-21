pub use env_var::{EnvVar, bool_env_var, env_var, read_bool_env};
use std::sync::LazyLock;

/// Whether CueCode is running in stateless mode.
/// When true, CueCode will use in-memory databases instead of persistent storage.
///
/// Prefer `CUECODE_STATELESS`; `ZED_STATELESS` is accepted for backward compatibility.
pub static CUECODE_STATELESS: LazyLock<bool> =
    LazyLock::new(|| read_bool_env("CUECODE_STATELESS", "ZED_STATELESS"));

/// Deprecated alias for [`CUECODE_STATELESS`].
pub use CUECODE_STATELESS as ZED_STATELESS;
