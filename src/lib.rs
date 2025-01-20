pub mod htmldiff;
pub use htmldiff::HtmlDiff;
#[cfg(feature = "extism")]
pub mod plugin;

/// Restores HTML content from diff format using HtmlDiff
///
/// # Arguments
/// * `diff_content` - A string containing the diff content
///
/// # Returns
/// A string with the restored HTML content
pub fn restore_from_diff(diff_content: &str) -> String {
    let lines: Vec<&str> = diff_content.split('\n').collect();
    let mut result = String::new();
    let mut deletions: Vec<String> = Vec::new();
    let mut insertions: Vec<String> = Vec::new();
    let diff = HtmlDiff::new();

    fn process_changes(
        diff: &HtmlDiff,
        result: &mut String,
        deletions: &mut Vec<String>,
        insertions: &mut Vec<String>,
    ) {
        let max_len = std::cmp::max(deletions.len(), insertions.len());
        for i in 0..max_len {
            let before = if i < deletions.len() {
                // Remove any existing HTML tags from deletions
                deletions[i].replace("<del>", "").replace("</del>", "")
            } else {
                String::new()
            };
            let after = if i < insertions.len() {
                // Remove any existing HTML tags from insertions
                insertions[i].replace("<ins>", "").replace("</ins>", "")
            } else {
                String::new()
            };
            result.push_str(&diff.diff(&before, &after));
            result.push('\n');
        }
        deletions.clear();
        insertions.clear();
    }

    for line in lines {
        if line.starts_with(' ') {
            // Context line, process any pending changes
            process_changes(&diff, &mut result, &mut deletions, &mut insertions);
            result.push_str(&line[1..]);
            result.push('\n');
        } else if line.starts_with('-') {
            // Deletion line - strip any existing HTML tags
            let content = line[1..].replace("<del>", "").replace("</del>", "");
            deletions.push(content);
        } else if line.starts_with('+') {
            // Addition line - strip any existing HTML tags
            let content = line[1..].replace("<ins>", "").replace("</ins>", "");
            insertions.push(content);
        }
    }

    // Process any remaining changes
    process_changes(&diff, &mut result, &mut deletions, &mut insertions);

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_from_diff() {
        let diff_content = "-A command line tool for generating and applying unified diffs between HTML files. The tool helps visualize and track changes in HTML documents by producing clean, semantic markup that clearly shows additions, deletions, and modifications.
+A command line tool for generating and applying unified diffs changed HTML files. The tool helps visualize and track changes in HTML documents producing clean, semantic markup that clearly shows , deletions, and modifications. adding text and more
+
+
+update 
+
+new lines
";
        let result = restore_from_diff(diff_content);
        println!("{}", result);
    }
}
