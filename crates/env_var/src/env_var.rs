use gpui_shared_string::SharedString;

#[derive(Clone)]
pub struct EnvVar {
    pub name: SharedString,
    /// Value of the environment variable. Also `None` when set to an empty string.
    pub value: Option<String>,
}

impl EnvVar {
    pub fn new(name: SharedString) -> Self {
        let value = std::env::var(name.as_str()).ok();
        if value.as_ref().is_some_and(|v| v.is_empty()) {
            Self { name, value: None }
        } else {
            Self { name, value }
        }
    }

    pub fn or(self, other: EnvVar) -> EnvVar {
        if self.value.is_some() { self } else { other }
    }
}

/// Reads an environment variable, falling back to a legacy name when the primary is unset.
pub fn read_env_var(primary: &str, fallback: &str) -> EnvVar {
    EnvVar::new(primary.into()).or(EnvVar::new(fallback.into()))
}

/// Returns true when either `primary` or `fallback` is set to a non-empty value.
pub fn read_bool_env(primary: &str, fallback: &str) -> bool {
    read_env_var(primary, fallback).value.is_some()
}

/// Creates a `LazyLock<EnvVar>` expression for use in a `static` declaration.
#[macro_export]
macro_rules! env_var {
    ($name:expr) => {
        ::std::sync::LazyLock::new(|| $crate::EnvVar::new(($name).into()))
    };
}

/// Generates a `LazyLock<bool>` expression for use in a `static` declaration. Checks if the
/// environment variable exists and is non-empty.
#[macro_export]
macro_rules! bool_env_var {
    ($name:expr) => {
        ::std::sync::LazyLock::new(|| $crate::EnvVar::new(($name).into()).value.is_some())
    };
}
