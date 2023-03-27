fn main() {
    
}

fn process_document() {

}

fn paragraph(input: &str) -> String {
    build_html("p", input)
}

fn line_break() -> String {
    single_tag("br")
}

fn parse_header(input: &str) -> String {
    let hashtags_count = input.trim().match_indices("#").count();
    if hashtags_count < 7 {
        let text = &input.trim()[hashtags_count..];
        build_html(format!("h{}", hashtags_count).as_str(), text.trim())
    } else {
        build_html("p", input)
    }
}

fn parse_unordered_list(input: &str) -> String {
    let line_vec: Vec<&str> = input.split("\n").collect();

    let mut output = "<ul>".to_string();
    for line in line_vec {
        let mut chars = line.chars();
        chars.next();
        let text = chars.as_str();
        if !text.is_empty() {
            output.push_str(&build_list_item(&text.trim()));
        }
    }
    output.push_str("</ul>");
    output
}

fn parse_ordered_list(input: &str) -> String {
    let line_vec: Vec<&str> = input.split("\n").collect();
    let mut output = "<ol>".to_string();
    for line in line_vec {
        let mut chars = line.chars();
        chars.next();
        chars.next();
        let text = chars.as_str();
        if !text.is_empty() {
            output.push_str(&build_list_item(&text.trim()));
        }
    }
    output.push_str("</ol>");
    output

}

fn build_list_item(input: &str) -> String {
    let mut output = "<li>".to_string();
    output.push_str(input);
    output.push_str("</li>");
    output
}

fn build_html(tag: &str, text: &str) -> String {
    format!("<{}>{}</{}>", tag, text, tag)
}

fn build_html_with_attrs(tag: &str, text: &str, attrs: Vec<(&str, &str)>) -> String {
    let mut format = format!("<{}", tag);

    for (key, value) in attrs {
        let attr_str = format!(" {}=\"{}\"", key, value);
        format.push_str(attr_str.as_str());
    } 

    let end = format!(">{}</{}>", text, tag);
    format.push_str(end.as_str());
    format
}

fn single_tag(tag: &str) -> String {
    format!("<{}>", tag)
}

fn link(title: &str, href: &str) -> String {
    let attrs = vec![("href", href)];
    build_html_with_attrs("a", title, attrs)
}

#[test]
fn test_header() {
    assert_eq!(parse_header("# Header 1"), "<h1>Header 1</h1>");
    assert_eq!(parse_header("## Header 2"), "<h2>Header 2</h2>");
    assert_eq!(parse_header("### Header 3"), "<h3>Header 3</h3>");
    assert_eq!(parse_header("#### Header 4"), "<h4>Header 4</h4>");
    assert_eq!(parse_header("##### Header 5"), "<h5>Header 5</h5>");
    assert_eq!(parse_header("###### Header 6"), "<h6>Header 6</h6>");
    assert_eq!(parse_header("####### Not a Header"), "<p>####### Not a Header</p>");
    assert_eq!(parse_header("    # Header 1"), "<h1>Header 1</h1>");
}

#[test]
fn test_unordered_list() {
    assert_eq!(parse_unordered_list("* bullet point 1\n* bullet point 2\n* bullet point 3\n"), "<ul><li>bullet point 1</li><li>bullet point 2</li><li>bullet point 3</li></ul>");
    assert_eq!(parse_unordered_list("- bullet point 1\n- bullet point 2\n- bullet point 3\n"), "<ul><li>bullet point 1</li><li>bullet point 2</li><li>bullet point 3</li></ul>");
    assert_eq!(parse_unordered_list("+ bullet point 1\n+ bullet point 2\n+ bullet point 3\n"), "<ul><li>bullet point 1</li><li>bullet point 2</li><li>bullet point 3</li></ul>");
    assert_eq!(parse_unordered_list("+ bullet **point** 1\n+ bullet +point 2\n+ bullet -point 3\n"), "<ul><li>bullet **point** 1</li><li>bullet +point 2</li><li>bullet -point 3</li></ul>");
}

#[test]
fn test_line_break() {
    assert_eq!(line_break(), "<br>");
}

#[test]
fn test_paragraph() {
    assert_eq!(paragraph("this is a test paragraph"), "<p>this is a test paragraph</p>");
}

#[test]
fn test_link() {
    assert_eq!(link("title", "http://google.com"), "<a href=\"http://google.com\">title</a>");
}

#[test]
fn test_build_html() {
    assert_eq!(build_html("tag", "text"), "<tag>text</tag>");
    let attrs = vec![
        ("class1", "name1"),
        ("class2", "name2")
    ];
    assert_eq!(build_html_with_attrs("tag", "text", attrs), "<tag class1=\"name1\" class2=\"name2\">text</tag>");
}

#[test]
fn test_ordered_list() {
    assert_eq!(parse_ordered_list("1. bullet point 1\n2. bullet point 2\n3. bullet point 3\n"), "<ol><li>bullet point 1</li><li>bullet point 2</li><li>bullet point 3</li></ol>");
}
