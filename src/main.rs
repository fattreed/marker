fn main() {
    
}

fn process_docment() {

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

fn parse_bullet(input: &str) -> String {
    let line_vec: Vec<&str> = input.split("\n").collect();

    let mut output = "<ul>".to_string();
    for line in line_vec.iter() {
        let text = line.replace(&['*', '-', '+'], "");
        if !text.is_empty() {
            output.push_str(&build_list_item(&text.trim()));
        }
    }
    output.push_str("</ul>");
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
fn test_bullet() {
    assert_eq!(parse_bullet("* bullet point 1\n* bullet point 2\n* bullet point 3\n"), "<ul><li>bullet point 1</li><li>bullet point 2</li><li>bullet point 3</li></ul>");
    assert_eq!(parse_bullet("- bullet point 1\n- bullet point 2\n- bullet point 3\n"), "<ul><li>bullet point 1</li><li>bullet point 2</li><li>bullet point 3</li></ul>");
    assert_eq!(parse_bullet("+ bullet point 1\n+ bullet point 2\n+ bullet point 3\n"), "<ul><li>bullet point 1</li><li>bullet point 2</li><li>bullet point 3</li></ul>");
}
