use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CheckboxProgress {
    pub checked: usize,
    pub total: usize,
}

impl CheckboxProgress {
    pub fn all_checked(&self) -> bool {
        self.total > 0 && self.checked == self.total
    }

    pub fn label(&self) -> String {
        if self.total == 0 {
            "-".to_string()
        } else {
            format!("{}/{}", self.checked, self.total)
        }
    }
}

static CHECKBOX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*-\s*\[( |x|X)\]").expect("valid checkbox regex"));

pub fn count_checkboxes(source: &str) -> CheckboxProgress {
    let mut checked = 0;
    let mut total = 0;
    for captures in CHECKBOX_RE.captures_iter(source) {
        total += 1;
        if captures.get(1).is_some_and(|m| m.as_str() != " ") {
            checked += 1;
        }
    }
    CheckboxProgress { checked, total }
}

pub fn count_checkboxes_in_file(path: &std::path::Path) -> anyhow::Result<CheckboxProgress> {
    let source = std::fs::read_to_string(path)?;
    Ok(count_checkboxes(&source))
}
