use diff_html_rs::htmldiff::HtmlDiff;
use pretty_assertions::assert_eq;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_simple_text_insertion() {
    let diff = HtmlDiff::new();
    let result = diff.diff("<p>Hello World</p>", "<p>Hello New World</p>");
    assert!(result.contains("<p>Hello <ins>New </ins>World</p>"));
}

#[test]
fn test_text_deletion() {
    let diff = HtmlDiff::new();
    let result = diff.diff("<p>Hello World</p>", "<p>Hello</p>");
    assert!(result.contains("<p>Hello<del> World</del></p>"));
}

#[test]
fn test_text_replacement() {
    let diff = HtmlDiff::new();
    let result = diff.diff("<p>Old Content</p>", "<p>New Content</p>");
    assert!(result.contains("<p><del>Old</del><ins>New</ins> Content</p>"));
}

#[test]
fn test_multiple_changes_same_element() {
    let diff = HtmlDiff::new();
    let result = diff.diff("<p>The quick brown fox</p>", "<p>A fast red fox</p>");
    assert!(result.contains("<p><del>The</del><ins>A</ins> <del>quick</del><ins>fast</ins> <del>brown</del><ins>red</ins> fox</p>"));
}

#[test]
fn test_multiple_paragraphs() {
    let diff = HtmlDiff::new();
    let result = diff.diff(
        "<div><p>First</p><p>Second</p></div>",
        "<div><p>First</p><p>Modified Second</p></div>",
    );
    assert!(result.contains("<div><p>First</p><p><ins>Modified </ins>Second</p></div>"));
}

#[test]
fn test_html_attributes_and_nested_structure() {
    let diff = HtmlDiff::new();
    let result = diff.diff(
        "<div class=\"container\"><p id=\"p1\">Content</p></div>",
        "<div class=\"wrapper\"><p id=\"p1\" style=\"color:red\">Modified Content</p></div>",
    );
    assert!(result.contains("<div class=\"container\"><p id=\"p1\"><div class=\"wrapper\"><p id=\"p1\" style=\"color:red\"><ins>Modified </ins>Content</p></div>"));
}

#[test]
fn test_word_with_chars_changes() {
    let diff = HtmlDiff::new();
    let result = diff.diff(
        "<p>Some content that will be removed.</p>",
        "<p>Some new content that was added.</p>",
    );
    assert!(result.contains("<p>Some<ins> new</ins> content that <del>will</del><ins>was</ins> <del>be removed</del><ins>added</ins>.</p>"));
}

#[test]
fn test_html_comment_handling() {
    let diff = HtmlDiff::new();
    let result = diff.diff(
        "<!-- Old comment --><p>Content</p>",
        "<!-- New comment --><p>Content</p>",
    );
    assert!(result.contains("<!-- Old comment --><!-- New comment --><p>Content</p>"));
}

#[test]
fn test_self_closing_tags() {
    let diff = HtmlDiff::new();
    let result = diff.diff(
        "<p>First line<br>Second line</p>",
        "<p>First line<hr>Second line</p>",
    );
    assert!(result.contains("<p>First line<br><hr>Second line</p>"));
}

// #[test]
// fn test_html_entities() {
//     let diff = HtmlDiff::new();
//     let result = diff.diff(
//         "<p>5 < 10 & 10 > 5</p>",
//         "<p>5 > 10 & 10 < 5</p>"
//     );
//     assert!(result.contains("<p>5 <del><</del><ins>></ins> 10 & 10 <del>></del><ins><</ins> 5</p>"));
// }
