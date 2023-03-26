fn main() {
    
}

fn process_docment() {

}

fn parse_header(input: String) -> String {
    let hashtags_count = input.match_indices("#").count();
    let text = input[hashtags_count..].trim();
    build_html(format!("h{}", hashtags_count), text.to_string())
}

fn parse_bullet(input: String) -> String {
    let bullet
}

fn build_html(tag: String, text: String) -> String {
    format!("<{}>{}</{}>", tag, text, tag)
}

#[test]
fn test_header() {
    assert_eq!(parse_header("# Header 1".to_string()), "<h1>Header 1</h1>".to_string());
    assert_eq!(parse_header("## Header 2".to_string()), "<h2>Header 2</h2>".to_string());
    assert_eq!(parse_header("### Header 3".to_string()), "<h3>Header 3</h3>".to_string());
    assert_eq!(parse_header("#### Header 4".to_string()), "<h4>Header 4</h4>".to_string());
    assert_eq!(parse_header("##### Header 5".to_string()), "<h5>Header 5</h5>".to_string());
    assert_eq!(parse_header("###### Header 6".to_string()), "<h6>Header 6</h6>".to_string());
}

#[test]
fn test_bullet() {
    assert_eq!(parse_bullet("* bullet point"), "<ul><li>bullet point</li><ul>");
}
