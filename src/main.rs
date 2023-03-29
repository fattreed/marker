use std::fs;
use std::collections::HashMap;

fn main() {
    process_document("test.md");
}

fn process_document(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    process_string(contents.as_str())
}

fn process_string(md: &str) -> String {
    let lines: Vec<_> = md.split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|l| l.replace("\r", ""))
        .filter(|l| *l != "")
        .collect();

    let mut html = "".to_string();
    let mut begin_list = false;
    let mut last_char: Option<char> = None;
    let mut last_line = "".to_string();

    for line in lines {
        let current_char = line.chars().next();
        if last_char == Some('*') && current_char != Some('*') && last_line != "***" {
            html.push_str("</ul>")
        } else if last_char == Some('+') && current_char != Some('+') {
            html.push_str("</ul>")
        } else if last_char == Some('-') && current_char != Some('-') && last_line != "---" {
            html.push_str("</ul>")
        }
        match current_char {
            Some('#') => {
                let header = header(line.as_str());
                html.push_str(header.as_str());
            }
            Some('*') => {
                if line == "***" {
                    html.push_str(horizontal_rule().as_str());
                } else {
                    if last_char != Some('*') {
                        html.push_str("<ul>");
                    }
                    html.push_str(unordered_list_item(line.as_str()).as_str());
                }
            }
            Some('+') => {
                if last_char != Some('+') {
                    html.push_str("<ul>");
                }
                html.push_str(unordered_list_item(line.as_str()).as_str());
            }
            Some('-') => {
                if line == "---" {
                    html.push_str(horizontal_rule().as_str());
                } else {
                    if last_char != Some('-') {
                        html.push_str("<ul>");
                    }
                    html.push_str(unordered_list_item(line.as_str()).as_str());
                }
            }
            Some('_') => {
                if line == "___" {
                    html.push_str(horizontal_rule().as_str());
                }
            }
            _ => {
                if line == "" {
                    break;
                } else {
                    let p = paragraph(line.as_str());
                    html.push_str(p.as_str());
                }
            }
        };
        last_char = current_char;
        last_line = line;
    }
    print!("{:?}", html);
    html.to_string().escape_default().to_string()
}

fn paragraph(input: &str) -> String {
    build_html("p", input)
}

fn line_break() -> String {
    single_tag("br")
}

fn header(input: &str) -> String {
    let mut chars = input.trim().chars();
    let mut hashtag_count = 0;
    while chars.next() == Some('#') {
        hashtag_count += 1;
    }
    if hashtag_count < 7 && input.replace("#", "").chars().next() == Some(' '){
        let text = chars.as_str().replace("#", "");
        build_html(format!("h{}", hashtag_count).as_str(), text.trim())
    } else {
        build_html("p", input)
    }
}

fn unordered_list_item(line: &str) -> String {
    let mut chars = line.chars();
    chars.next();
    let text = chars.as_str();
    build_list_item(text.trim())
}

fn ordered_list_item(line: &str) -> String {
    let mut chars = line.chars();
    chars.next();
    chars.next();
    let text = chars.as_str();
    build_list_item(text.trim())
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

fn horizontal_rule() -> String {
    single_tag("hr")
}

fn replace_links(input: &str) -> String {
    let elements: Vec<_> = input
        .split("[")
        .collect::<Vec<_>>();
    
    let mut links: HashMap<&str, &str> = HashMap::new();

    if elements.iter().count() < 1 {
        return input.to_string();
    }

    for element in elements[1..].iter() {
        let title_vec = element
            .split("]")
            .collect::<Vec<_>>();
         
        if title_vec.iter().count() < 1 {
            break;
        }

        let title = title_vec[0];

        let href_vec = title
            .split("(")
            .collect::<Vec<_>>();

        let hrefs = href_vec
            .into_iter()
            .map(|l| {
                let l_vec = l.split(")").collect::<Vec<_>>();
                if l_vec.iter().count() > 1 {
                    Some(l_vec[0])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for href in hrefs {
            if let Some(href_str) = href {
                links.insert(title, href_str);
            }
        }
    }

    let mut output = input.to_string();
    for (key, value) in links {
        output = output.replace(format!("[{}]({})", key, value).as_str(), 
                                format!("<a href=\"{}\">{}</a>", value, key).as_str());
    }
    output
}

#[test]
fn test_processing() {
    let md = fs::read_to_string("test.md")
        .expect("Should have been able to read the file");

    let html = fs::read_to_string("test.html")
        .expect("Should have been able to read the file");

    assert_eq!(process_string(md.as_str()), html.as_str());
}

#[test]
fn test_header() {
    assert_eq!(header("# Header 1"), "<h1>Header 1</h1>");
    assert_eq!(header("## Header 2"), "<h2>Header 2</h2>");
    assert_eq!(header("### Header 3"), "<h3>Header 3</h3>");
    assert_eq!(header("#### Header 4"), "<h4>Header 4</h4>");
    assert_eq!(header("##### Header 5"), "<h5>Header 5</h5>");
    assert_eq!(header("###### Header 6"), "<h6>Header 6</h6>");
    assert_eq!(header("####### Not a Header"), "<p>####### Not a Header</p>");
    assert_eq!(header("    # Header 1"), "<h1>Header 1</h1>");
    assert_eq!(header("#Header 1"), "<p>#Header 1</p>");
    assert_eq!(header("# Header 1 #"), "<h1>Header 1</h1>");
}

#[test]
fn test_unordered_list() {
    assert_eq!(unordered_list_item("* bullet point"), "<li>bullet point</li>");
    assert_eq!(unordered_list_item("- bullet point"), "<li>bullet point</li>");
    assert_eq!(unordered_list_item("+ bullet point"), "<li>bullet point</li>");
    assert_eq!(unordered_list_item("+ bullet **point**"), "<li>bullet **point**</li>");
    assert_eq!(unordered_list_item("+ bullet +point"), "<li>bullet +point</li>");
    assert_eq!(unordered_list_item("+ bullet -point"), "<li>bullet -point</li>");
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
    assert_eq!(ordered_list_item("1. bullet point 1"), "<li>bullet point 1</li>");
}

#[test]
fn test_horizontal_rule() {
    assert_eq!(horizontal_rule(), "<hr>");
}

#[test]
fn test_replace_links() {
    let text = "this is a [link](http://google.com) to google and this is a [link](http://twitter.com) to elon's mom.";
    assert_eq!(replace_links(text), 
               "this is a <a href=\"http://google.com\">link</a> to google and this is <a href=\"http://twitter.com\">link</a> to elon's mom.");

    let no_links = "this has no links";
    assert_eq!(replace_links(no_links), no_links);
}
