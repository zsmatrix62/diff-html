#[cfg(feature = "extism")]
use crate::restore_from_diff;
#[cfg(feature = "extism")]
use extism_pdk::*;
#[cfg(feature = "extism")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "extism")]
#[derive(Serialize, Deserialize)]
struct DiffInput {
    before: String,
    after: String,
}

#[cfg(feature = "extism")]
#[derive(Serialize, Deserialize)]
struct DiffOutput {
    result: String,
}

#[cfg(feature = "extism")]
#[plugin_fn]
pub fn DiffHtml(input: String) -> FnResult<String> {
    // Parse input JSON
    let input: DiffInput = serde_json::from_str(&input)?;

    // Generate diff lines using diff lib
    let changes = diff::lines(&input.before, &input.after);
    let unidiff_content: String = changes
        .iter()
        .map(|change| match change {
            diff::Result::Left(s) => format!("-{}\n", s),
            diff::Result::Both(s, _) => format!("{}\n", s),
            diff::Result::Right(s) => format!("+{}\n", s),
        })
        .collect();

    let diff_result = restore_from_diff(&unidiff_content);

    // Return as plain string
    Ok(diff_result)
}
