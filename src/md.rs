// Module to convert markdown to HTML
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[allow(dead_code)]
pub const MD_SYNTAX_GUIDE: &str = "MARKDOWN SYNTAX GUIDE:\n
Headings:  # H1,  ## H2,  ### H3\n
Bold:  **bold text** \n
Italic:  *italicized text* \n
Strikethrough:  ~~strikethrough~~ \n
Blockquote:  > blockquote\n
Inline Code:  `code`\n
Code Block:  ```code```\n
Unordered List:  - item\n
Ordered List:  1. item\n
Horizontal Rule:  ---  or  ***\n
Link:  [title](https://www.example.com)\n
Image:  ![alt text](./images/image.jpg)";

// Function to write content to a file

#[allow(dead_code)]
fn write_to_file(content: String, output_path: &str) -> io::Result<()> {
    let mut output_file = File::create(output_path)?;
    write!(output_file, "{}", content)?;
    Ok(())
}

#[allow(dead_code)]
fn parse_markdown(markdown_content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown_content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[allow(dead_code)]
pub fn convert_markdown_to_html(staging_file_path: &str) -> io::Result<String> {
    // Read the contents of the staging.md file
    let input_file = File::open(staging_file_path)?;
    let reader = BufReader::new(input_file);
    let mut markdown_content = String::new();
    for line in reader.lines() {
        markdown_content.push_str(&line.expect("Failed to read from md file"));
        markdown_content.push('\n');
    }
    // Parse the markdown content and write it to the output HTML file
    let html_body = parse_markdown(&markdown_content);

    // Print a success message
    Ok((html_body).to_string())
}
