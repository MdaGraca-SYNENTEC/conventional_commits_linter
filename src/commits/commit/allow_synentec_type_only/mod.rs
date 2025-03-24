use regex::Regex;

use super::*;

pub(super) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref SYNENTEC_TYPE_REGEX: Regex = Regex::new(&format!(
            r"(?i)^{OPTIONAL_PRECEDING_WHITESPACE}{SYNENTEC_TYPE}{OPTIONAL_EXCLAMATION}{OPTIONAL_EMPTY_SCOPE_OR_SCOPE}{OPTIONAL_EXCLAMATION}:",
        ))
        .unwrap();
    }

    match SYNENTEC_TYPE_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NonSynentecType),
    }
}
