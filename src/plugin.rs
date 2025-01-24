#[cfg(feature = "extism")]
use crate::restore_from_diff;
#[cfg(feature = "extism")]
use base64;
#[cfg(feature = "extism")]
use extism_pdk::*;
use serde::{Deserialize, Serialize};

use crate::htmldiff::HtmlDiff;

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

    // Decode base64 strings
    let before = String::from_utf8(base64::decode(&input.before)?)?;
    let after = String::from_utf8(base64::decode(&input.after)?)?;

    let hd = HtmlDiff::new();
    let diff_result = hd.diff(&before, &after);

    // Encode result as base64
    let encoded_result = base64::encode(diff_result);

    // Return as base64 encoded string
    Ok(encoded_result)
}
