use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Match {
    start_in_before: usize,
    start_in_after: usize,
    length: usize,
    end_in_before: usize,
    end_in_after: usize,
}

impl Match {
    fn new(start_in_before: usize, start_in_after: usize, length: usize) -> Self {
        let end_in_before = start_in_before.saturating_add(length).saturating_sub(1);
        let end_in_after = start_in_after.saturating_add(length).saturating_sub(1);
        Self {
            start_in_before,
            start_in_after,
            length,
            end_in_before,
            end_in_after,
        }
    }
}

#[derive(Debug)]
struct SearchRange {
    start_in_before: usize,
    end_in_before: usize,
    start_in_after: usize,
    end_in_after: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Equal,
    Insert,
    Delete,
    Replace,
}

struct DiffOperation {
    action: Operation,
    start_in_before: usize,
    end_in_before: Option<usize>,
    start_in_after: usize,
    end_in_after: Option<usize>,
}

/// HTML diffing utility that compares HTML content and generates
/// a visual diff with <ins> and <del> tags
#[derive(Debug)]
pub struct HtmlDiff {
    whitespace_regex: Regex,
    tag_regex: Regex,
    char_regex: Regex,
}

impl HtmlDiff {
    pub fn new() -> Self {
        Self {
            whitespace_regex: Regex::new(r"\s").unwrap(),
            tag_regex: Regex::new(r"^\s*<[^>]+>\s*$").unwrap(),
            char_regex: Regex::new(r"[\w\#@]").unwrap(),
        }
    }

    fn is_whitespace(&self, char: char) -> bool {
        self.whitespace_regex.is_match(&char.to_string())
    }

    fn is_tag(&self, token: &str) -> bool {
        self.tag_regex.is_match(token)
            || token.starts_with("<!--")
            || token.ends_with("-->")
            || token.ends_with("/>")
    }

    fn is_start_of_tag(&self, char: char) -> bool {
        char == '<'
    }

    fn is_start_of_entity(&self, char: char) -> bool {
        char == '&'
    }

    fn is_end_of_tag(&self, char: char) -> bool {
        char == '>'
    }

    fn is_end_of_entity(&self, char: char) -> bool {
        char == ';'
    }

    pub fn html_to_tokens(&self, html: &str) -> Vec<String> {
        let mut mode = "char";
        let mut current_word = String::new();
        let mut words = Vec::new();

        for char in html.chars() {
            match mode {
                "tag" => {
                    if self.is_end_of_tag(char) {
                        current_word.push('>');
                        words.push(current_word);
                        current_word = String::new();
                        mode = if self.is_whitespace(char) {
                            "whitespace"
                        } else {
                            "char"
                        };
                    } else {
                        current_word.push(char);
                    }
                }
                "entity" => {
                    if self.is_end_of_entity(char) {
                        current_word.push(';');
                        words.push(current_word);
                        current_word = String::new();
                        mode = if self.is_whitespace(char) {
                            "whitespace"
                        } else {
                            "char"
                        };
                    } else {
                        current_word.push(char);
                    }
                }
                "char" => {
                    if self.is_start_of_tag(char) {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                        current_word.push('<');
                        mode = "tag";
                    } else if self.is_start_of_entity(char) {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                        current_word.push('&');
                        mode = "entity";
                    } else if self.is_whitespace(char) {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                        current_word.push(char);
                        mode = "whitespace";
                    } else if self.char_regex.is_match(&char.to_string()) {
                        current_word.push(char);
                    } else {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                        current_word.push(char);
                    }
                }
                "whitespace" => {
                    if self.is_start_of_tag(char) {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                        current_word.push('<');
                        mode = "tag";
                    } else if self.is_start_of_entity(char) {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                        current_word.push('&');
                        mode = "entity";
                    } else if self.is_whitespace(char) {
                        current_word.push(char);
                    } else {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                        current_word.push(char);
                        mode = "char";
                    }
                }
                _ => {}
            }
        }

        if !current_word.is_empty() {
            words.push(current_word);
        }

        words
    }

    fn create_index(
        &self,
        find_these: &[String],
        in_these: &[String],
    ) -> HashMap<String, Vec<usize>> {
        let mut index = HashMap::new();
        for token in find_these {
            index.insert(token.clone(), Vec::new());
            for (i, t) in in_these.iter().enumerate() {
                if t == token {
                    index.get_mut(token).unwrap().push(i);
                }
            }
        }
        index
    }

    fn find_match(
        &self,
        before_tokens: &[String],
        after_tokens: &[String],
        index: &HashMap<String, Vec<usize>>,
        start_in_before: usize,
        end_in_before: usize,
        start_in_after: usize,
        end_in_after: usize,
    ) -> Option<Match> {
        let _ = after_tokens;
        let mut best_match_in_before = start_in_before;
        let mut best_match_in_after = start_in_after;
        let mut best_match_length = 0;

        let mut match_length_at = HashMap::new();

        for index_in_before in start_in_before..end_in_before {
            let mut new_match_length_at = HashMap::new();
            let looking_for = &before_tokens[index_in_before];

            if let Some(locations_in_after) = index.get(looking_for) {
                for &index_in_after in locations_in_after {
                    if index_in_after < start_in_after {
                        continue;
                    }
                    if index_in_after >= end_in_after {
                        break;
                    }

                    let previous_length = if index_in_after > 0 {
                        *match_length_at.get(&(index_in_after - 1)).unwrap_or(&0)
                    } else {
                        0
                    };
                    let new_match_length: usize = previous_length + 1;
                    new_match_length_at.insert(index_in_after, new_match_length);

                    if new_match_length > best_match_length {
                        best_match_in_before =
                            index_in_before.saturating_sub(new_match_length.saturating_sub(1));
                        best_match_in_after =
                            index_in_after.saturating_sub(new_match_length.saturating_sub(1));
                        best_match_length = new_match_length;
                    }
                }
            }

            match_length_at = new_match_length_at;
        }

        if best_match_length != 0 {
            Some(Match::new(
                best_match_in_before,
                best_match_in_after,
                best_match_length,
            ))
        } else {
            None
        }
    }

    fn find_matching_blocks(
        &self,
        before_tokens: &[String],
        after_tokens: &[String],
    ) -> Vec<Match> {
        let mut matching_blocks = Vec::new();
        let index = self.create_index(before_tokens, after_tokens);

        let mut stack = vec![SearchRange {
            start_in_before: 0,
            end_in_before: before_tokens.len(),
            start_in_after: 0,
            end_in_after: after_tokens.len(),
        }];

        while let Some(current) = stack.pop() {
            if let Some(match_) = self.find_match(
                before_tokens,
                after_tokens,
                &index,
                current.start_in_before,
                current.end_in_before,
                current.start_in_after,
                current.end_in_after,
            ) {
                // Push right range first (LIFO)
                if match_.end_in_before + 1 < current.end_in_before
                    && match_.end_in_after + 1 < current.end_in_after
                {
                    stack.push(SearchRange {
                        start_in_before: match_.end_in_before + 1,
                        end_in_before: current.end_in_before,
                        start_in_after: match_.end_in_after + 1,
                        end_in_after: current.end_in_after,
                    });
                }

                matching_blocks.push(match_.clone());

                // Push left range
                if current.start_in_before < match_.start_in_before
                    && current.start_in_after < match_.start_in_after
                {
                    stack.push(SearchRange {
                        start_in_before: current.start_in_before,
                        end_in_before: match_.start_in_before,
                        start_in_after: current.start_in_after,
                        end_in_after: match_.start_in_after,
                    });
                }
            }
        }

        matching_blocks.sort_by(|a, b| a.start_in_before.cmp(&b.start_in_before));
        matching_blocks
    }

    fn calculate_operations(
        &self,
        before_tokens: &[String],
        after_tokens: &[String],
    ) -> Vec<DiffOperation> {
        let mut operations = Vec::new();
        let mut position_in_before = 0;
        let mut position_in_after = 0;

        let mut matches = self.find_matching_blocks(before_tokens, after_tokens);
        matches.push(Match::new(before_tokens.len(), after_tokens.len(), 0));

        for match_ in matches {
            let match_starts_at_current_position_in_before =
                position_in_before == match_.start_in_before;
            let match_starts_at_current_position_in_after =
                position_in_after == match_.start_in_after;

            let action = if !match_starts_at_current_position_in_before
                && !match_starts_at_current_position_in_after
            {
                Operation::Replace
            } else if match_starts_at_current_position_in_before
                && !match_starts_at_current_position_in_after
            {
                Operation::Insert
            } else if !match_starts_at_current_position_in_before
                && match_starts_at_current_position_in_after
            {
                Operation::Delete
            } else {
                Operation::Equal
            };

            if action != Operation::Equal {
                operations.push(DiffOperation {
                    action,
                    start_in_before: position_in_before,
                    end_in_before: if action == Operation::Insert {
                        None
                    } else {
                        Some(match_.start_in_before.saturating_sub(1))
                    },
                    start_in_after: position_in_after,
                    end_in_after: if action == Operation::Delete {
                        None
                    } else {
                        Some(match_.start_in_after.saturating_sub(1))
                    },
                });
            }

            if match_.length != 0 {
                operations.push(DiffOperation {
                    action: Operation::Equal,
                    start_in_before: match_.start_in_before,
                    end_in_before: Some(match_.end_in_before),
                    start_in_after: match_.start_in_after,
                    end_in_after: Some(match_.end_in_after),
                });
            }

            position_in_before = match_.end_in_before.saturating_add(1);
            position_in_after = match_.end_in_after.saturating_add(1);

            // Ensure positions don't exceed token array bounds
            position_in_before = position_in_before.min(before_tokens.len());
            position_in_after = position_in_after.min(after_tokens.len());
        }

        operations
    }

    fn consecutive_where<F>(&self, start: usize, content: &[String], predicate: F) -> Vec<String>
    where
        F: Fn(&str) -> bool,
    {
        let mut last_matching_index = None;
        for (i, token) in content.iter().enumerate().skip(start) {
            if predicate(token) {
                last_matching_index = Some(i);
            } else {
                break;
            }
        }

        if let Some(end) = last_matching_index {
            content[start..=end].to_vec()
        } else {
            Vec::new()
        }
    }

    fn wrap(&self, tag: &str, content: &[String]) -> String {
        let mut rendering = String::new();
        let mut position = 0;
        let length = content.len();

        while position < length {
            let non_tags = self.consecutive_where(position, content, |token| !self.is_tag(token));
            position += non_tags.len();
            if !non_tags.is_empty() {
                rendering.push_str(&format!("<{}>{}</{}>", tag, non_tags.join(""), tag));
            }

            if position >= length {
                break;
            }

            let tags = self.consecutive_where(position, content, |token| self.is_tag(token));
            position += tags.len();
            rendering.push_str(&tags.join(""));
        }

        rendering
    }

    pub fn diff(&self, before: &str, after: &str) -> String {
        // Normalize input by handling different string delimiters
        let before = before
            .trim_matches('`')
            .trim_matches('"')
            .trim_matches('\'');
        let after = after.trim_matches('`').trim_matches('"').trim_matches('\'');

        if before == after {
            return before.to_string();
        }

        // Handle HTML content regardless of delimiters
        let before_tokens = self.html_to_tokens(before);
        let after_tokens = self.html_to_tokens(after);
        let operations = self.calculate_operations(&before_tokens, &after_tokens);

        let mut rendering = String::new();
        for op in operations {
            match op.action {
                Operation::Equal => {
                    rendering.push_str(
                        &before_tokens[op.start_in_before..=op.end_in_before.unwrap()].join(""),
                    );
                }
                Operation::Insert => {
                    let val = &after_tokens[op.start_in_after..=op.end_in_after.unwrap()];
                    rendering.push_str(&self.wrap("ins", val));
                }
                Operation::Delete => {
                    let val = &before_tokens[op.start_in_before..=op.end_in_before.unwrap()];
                    rendering.push_str(&self.wrap("del", val));
                }
                Operation::Replace => {
                    let before_val = &before_tokens[op.start_in_before..=op.end_in_before.unwrap()];
                    let after_val = &after_tokens[op.start_in_after..=op.end_in_after.unwrap()];
                    rendering.push_str(&self.wrap("del", before_val));
                    rendering.push_str(&self.wrap("ins", after_val));
                }
            }
        }

        rendering
    }
}
