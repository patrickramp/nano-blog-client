// Module to send html file to server API
use reqwest;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::{Client, StatusCode};
use std::fs::File;
use std::io::prelude::*;

// Set client
fn get_client() -> Client {
    let agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Client::builder()
        .user_agent(agent)
        .build()
        .expect("Failed to create client")
}

fn create_form(html_file_path: &str) -> Form {
    // Get the file name
    let filename = html_file_path.split('/').last().unwrap().to_string();
    // Open file and read its contents
    let file = File::open(&html_file_path).expect("Failed to open file");
    // Convert the file contents to a vector of bytes
    let file_content: Vec<u8> = file.bytes().map(|result| result.unwrap()).collect();
    // Clone the filename to a 'static string
    let static_filename = filename.clone();

    // Get the image name
    //let image_name = html_file_path[2..].to_string();
    // Open image and read its contents
    //let image_file = File::open(&image_name).expect("Failed to open image");
    // Convert the image contents to a vector of bytes
    //let image_content: Vec<u8> = image_file.bytes().map(|result| result.unwrap()).collect();
    // Clone the image name to a 'static string
    //let static_image_name = image_name.clone();

    // Build form containing the file bytes
    println!("File name: {}", static_filename);
    //println!("Image name: {}", static_image_name);

    //
    let form = Form::new().part(
        static_filename.clone(),
        Part::stream(file_content).file_name(static_filename),
    );
    //  .part(static_image_name, Part::stream(image_content));

    form
}

// Function to send html file to server API
#[tokio::main]
pub async fn post_to_server(
    author: &str,
    username: &str,
    password: &str,
    hostname: &str,
    port: &str,
    html_file_path: &str,
) -> Option<StatusCode> {
    // Specify the URL where you want to send the multipart request
    let url = format!("{}:{}", hostname, port); // Update with your server URL
                                                // Create client
    let client = get_client();
    // Create form
    let form = create_form(html_file_path);
    println!("Sending {:?}", form);
    // Send the multipart request with authorization header

    let _response = match client
        .post(format!("{}{}", url, "/api/post"))
        .header(
            "authorization",
            format!("{}:{}:{}", author, username, password),
        )
        .multipart(form)
        .send()
        .await
    {
        Ok(response) => {
            if response.status()
                == reqwest::StatusCode::from_u16(200).expect("Failed to get status code")
            {
                println!("File sent successfully!\n{:?}", response);
                return Some(StatusCode::OK);
            } else {
                println!("Failed to send file!\n{:?}", response);
                return Some(StatusCode::BAD_REQUEST);
            }
        }
        Err(e) => {
            println!("Failed to send request: {}", e);
            return Some(StatusCode::BAD_REQUEST);
        }
    };
}
