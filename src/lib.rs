// [dependencies.gtk]
use std::io::{Read, Write};
mod html;
mod md;
mod ui;
use crate::html::add_headers;
use crate::md::convert_markdown_to_html;
use crate::ui::show_popup;
use chrono::Local;
use gtk::prelude::*;
use gtk::{Entry, TextView};
use std::fs::File;
use std::process::Command;
use std::{io, thread};

//  Function to get the body text from the TextView buffer and convert it to a String.
pub fn textview_to_string(body: &TextView) -> String {
    let start = body.buffer().unwrap().start_iter();
    let end = body.buffer().unwrap().end_iter();
    let body = body
        .buffer()
        .unwrap()
        .text(&start, &end, true)
        .expect("Failed to get body text")
        .to_string();
    return body;
}
// Function to convert content buffers to a String
pub fn content_to_string(
    title: &Entry,
    headline: &Entry,
    tags: &Entry,
    body: &String,
    author: &Entry,
) -> (String, String, String, String, String, String) {
    let title = title.buffer().text();
    let headline = headline.buffer().text();
    let tags = tags.buffer().text();
    // Get the body text from the TextView buffer and convert it to a String.
    let body = body.to_string();
    let author = author.buffer().text();
    let safe_title = title.replace(" ", "-");
    let now = Local::now().format("%Y-%m-%d").to_string();
    // Check if posts directory exists, if not, create it
    if !std::path::Path::new("posts").exists() {
        std::fs::create_dir("posts").expect("Failed to create posts directory");
    }
    let html_file_path = format!("./posts/{}-{}.html", &now, &safe_title);
    return (title, headline, tags, body, author, html_file_path);
}

// Function to return server config from Entry buffers
pub fn return_server_config(
    username: &Entry,
    password: &Entry,
    hostname: &Entry,
    port: &Entry,
) -> (String, String, String, String) {
    let username = username.buffer().text();
    let password = password.buffer().text();
    let hostname = hostname.buffer().text();
    let port = port.buffer().text();
    return (username, password, hostname, port);
}

// Function to write content to a file
fn write_to_file(content: &str, output_path: &str) -> io::Result<()> {
    let mut output_file = File::create(output_path)?;
    write!(output_file, "{}", content).expect("Failed to write to staging file");
    Ok(())
}

// Function to read server config from server.conf file if none exists, return default values
pub fn read_server_config() -> (String, String, String, String, String) {
    if let Ok(mut server_config) = File::open("./config/server.conf") {
        let mut contents = String::new();
        server_config
            .read_to_string(&mut contents)
            .expect("Failed to read server config file");
        for _ in contents.lines() {
            let lines: Vec<&str> = contents.split('\n').collect();
            let author = lines[0].to_string();
            let username = lines[1].to_string();
            let password = lines[2].to_string();
            let hostname = lines[3].to_string();
            let port = lines[4].to_string();
            return (author, username, password, hostname, port);
        }
    }
    return (
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    );
}

// Function to write config to server.conf file
pub fn update_server_config(
    author: &str,
    username: &str,
    password: &str,
    hostname: &str,
    port: &str,
) {
    // Create ./config directory if it doesn't exist
    if !std::path::Path::new("./config").exists() {
        std::fs::create_dir("./config").expect("Failed to create ./config directory!");
    }
    // Open server.conf file
    if let Ok(mut server_config) = File::create("./config/server.conf") {
        // convert to string
        write!(
            server_config,
            "{}\n{}\n{}\n{}\n{}",
            author, username, password, hostname, port
        )
        .expect("Failed to write to ./server.conf file!");
        println!("Server config updated successfully!");
    }
}

pub fn write_to_staging(
    title: &str,
    headline: &str,
    tags: &str,
    body: &str,
    author: &str,
    html_file_path: &str,
) -> io::Result<()> {
    // Check if all fields are filled
    return if title.is_empty()
        || headline.is_empty()
        || body.is_empty()
        || tags.is_empty()
        || author.is_empty()
    {
        eprintln!("Missing one or more content fields!");
        show_popup("Must fill all content fields!");
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Missing content fields!",
        ))
    } else {
        // Create ./staging directory if it doesn't exist
        if !std::path::Path::new("./staging").exists() {
            std::fs::create_dir("./staging").expect("Failed to create ./staging directory!");
        }
        let staging_file_path = "./staging/staging.md";
        // Write body to staging.md
        write_to_file(&body, "./staging/staging.md").expect("Failed to write body to staging file");
        // Convert markdown to html
        let html_body = convert_markdown_to_html(&staging_file_path)
            .expect("Failed to convert markdown to HTML");
        // Add headers to html
        let html_full = add_headers(&title, &headline, &tags, &author, &html_body.to_string());
        // Write html to html_file
        write_to_file(&html_full, html_file_path).expect("Failed to write to HTML file!");
        println!("HTML file created: {}", html_file_path);
        Ok(())
    };
}

// Function to preview HTML file in browser
pub fn preview_in_browser(html_file_path: &str) {
    let html_local_path = html_file_path.to_string();
    thread::spawn(move || {
        Command::new("xdg-open")
            .arg(&html_local_path)
            .output()
            .expect("Failed to open in browser!");
    });
}

// Function to read staging.md
pub fn read_staging() -> String {
    let mut staging_file = File::open("./staging/staging.md").expect("Failed to open staging file");
    let mut content = String::new();
    staging_file
        .read_to_string(&mut content)
        .expect("Failed to read staging file");
    content
}
