// [dependencies]
use crate::api::*;
use crate::md::*;
use crate::ui::*;
use gtk::prelude::*;
use gtk::{Application, Button, Orientation, ScrolledWindow, TextView, ToggleButton, WrapMode};
use nano_blog_client::*;
use reqwest::StatusCode;
use std::rc::Rc;

mod api;
mod html;
mod md;
mod ui;

fn main() {
    // Create main application
    let nano_blog = Application::builder()
        .application_id("org.gtk-rs.Nano-Blog")
        .build();

    // Connect to application
    nano_blog.connect_activate(|application| {
        // Create main window and main vertical box
        let (main_window, main_vbox) = build_window(application);

        // Add a labeled entry box for blog title.
        let title = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &main_vbox,
            set_text: "".to_string(),
            label_text: "Title:".to_string(),
            placeholder: "Title for your post.".to_string(),
            tooltip: "Enter the title for your post.".to_string(),
            width: 32,
            fill: false,
            visible: true,
        })));

        // Add a labeled entry box for blog headline.
        let headline = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &main_vbox,
            set_text: "".to_string(),
            label_text: "Headline:".to_string(),
            placeholder: "Headline for your post.".to_string(),
            tooltip: "Enter the headline for your post.".to_string(),
            width: 32,
            fill: false,
            visible: true,
        })));

        // Add a labeled entry box for blog tags.
        let tags = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &main_vbox,
            set_text: "".to_string(),
            label_text: "Tags:".to_string(),
            placeholder: "Comma separated list of tags".to_string(),
            tooltip: "Enter list of tags to be associated with your post".to_string(),
            width: 32,
            fill: false,
            visible: true,
        })));

        // Add horizontal box for formatting guide
        let formatting_box = create_box(Orientation::Horizontal, true);
        main_vbox.pack_start(&formatting_box, false, true, 5);

        // Add a label for blog content and formatting guide
        formatting_box.pack_start(
            &create_label(
                "Body of your post: (Markdown format)",
                "Uses Markdown syntax for formatting.",
            ),
            true,
            false,
            0,
        );

        // Add button to bring up formatting guidelines
        let guideline_button = Button::with_label("MD Guide");
        guideline_button.set_halign(gtk::Align::End);
        formatting_box.pack_start(&guideline_button, true, true, 1);

        // Create a scrolled window to enable scrolling
        let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_vbox.pack_start(&scrolled_window, true, true, 5);

        // Add a TextView to collect blog content
        let body = TextView::new(); // TextView::new();
        body.set_wrap_mode(WrapMode::WordChar);
        body.set_tooltip_text(Some("Enter blog content."));
        scrolled_window.add(&body);

        // Create RC for body
        let body = Rc::new(body);

        // Retrieve server details from config file
        let (cf_author, cf_user, cf_pass, cf_host, cf_port) = read_server_config();

        // Add a horizontal box layout for authors
        let author_box = create_box(Orientation::Horizontal, true);
        main_vbox.pack_start(&author_box, false, true, 5);

        // Create horizontal box layout for main window buttons.
        let button_box = create_box(Orientation::Horizontal, true);
        main_vbox.pack_start(&button_box, false, false, 10);

        // Add a labeled entry box for blog author
        let author = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &author_box,
            set_text: cf_author.to_string(),
            label_text: "Author:".to_string(),
            placeholder: "Public Penname".to_string(),
            tooltip: "Enter name of author".to_string(),
            width: 16,
            fill: true,
            visible: true,
        })));

//        // Create entry box for upload image
//        let _image = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
//            parent: &author_box,
//            set_text: "".to_string(),
//            label_text: "Image:".to_string(),
//            placeholder: "Enter path to image".to_string(),
//            tooltip: "Enter image path".to_string(),
//            width: 16,
//            fill: true,
//            visible: true,
//        })));

        // Create toggle button to show / hide server details
        let show_button = ToggleButton::with_label("Server Details");
        show_button.set_halign(gtk::Align::Start);
        show_button.set_tooltip_text(Some("Show server details."));
        button_box.pack_start(&show_button, true, true, 1);

        // Create a preview blog button with label.
        let preview_button = Button::with_label("Preview Blog");
        preview_button.set_halign(gtk::Align::End);
        preview_button.set_tooltip_text(Some("Preview blog in browser."));
        button_box.pack_start(&preview_button, false, false, 1);

        // Create a post blog button with label.
        let post_button = Button::with_label("Post Blog");
        post_button.set_halign(gtk::Align::End);
        post_button.set_tooltip_text(Some("Post blog to webserver."));
        post_button.set_sensitive(true);
        button_box.pack_start(&post_button, false, false, 1);

        // Show all widgets in main window
        main_window.show_all();

        //// Create FIRST horizontal config box layout.
        let first_config_box = create_box(Orientation::Horizontal, false);
        main_vbox.pack_start(&first_config_box, false, true, 0);

        //// Create second horizontal box layout.
        let second_config_box = create_box(Orientation::Horizontal, false);
        main_vbox.pack_start(&second_config_box, false, true, 0);

        //// Create third horizontal box layout.
        let third_config_box = create_box(Orientation::Horizontal, false);
        main_vbox.pack_start(&third_config_box, false, true, 0);

        // Add a labeled entry box for API host.
        let username = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &first_config_box,
            set_text: cf_user,
            label_text: "User:".to_string(),
            placeholder: "username".to_string(),
            tooltip: "Enter your username".to_string(),
            width: 10,
            fill: true,
            visible: true,
        })));

        // Add labeled entry box for password.
        let password = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &first_config_box,
            set_text: cf_pass,
            label_text: "Passwd:".to_string(),
            placeholder: "password".to_string(),
            tooltip: "Enter your password".to_string(),
            width: 10,
            fill: true,
            visible: false,
        })));

        // Add a labeled entry box for API host.
        let hostname = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &second_config_box,
            set_text: cf_host,
            label_text: "Host:".to_string(),
            placeholder: "http://your-host.com".to_string(),
            tooltip: "Enter your hostname".to_string(),
            width: 10,
            fill: true,
            visible: true,
        })));

        // Add a labeled entry box for API port.
        let port = Rc::new(add_entry_to_parent(EntryConfig::from(EntryConfig {
            parent: &second_config_box,
            set_text: cf_port,
            label_text: ":".to_string(),
            placeholder: "8080".to_string(),
            tooltip: "port #".to_string(),
            width: 6,
            fill: false,
            visible: true,
        })));

        // Create save server config button with label.
        let save_button = Button::with_label("Save Configuration");
        save_button.set_halign(gtk::Align::End);
        save_button.set_tooltip_text(Some("Save server details to config file."));
        save_button.set_sensitive(true);
        third_config_box.pack_start(&save_button, false, false, 1);

        // Create Clear server config button with label.
        let clear_button = Button::with_label("Clear Saved Configuration");
        clear_button.set_halign(gtk::Align::End);
        clear_button.set_tooltip_text(Some("Clear server details from config file."));
        clear_button.set_sensitive(true);
        third_config_box.pack_start(&clear_button, false, false, 1);

        // Create checkbox for save password
        let save_password = _create_checkbox("Save Password");
        save_password.set_halign(gtk::Align::End);
        first_config_box.pack_start(&save_password, false, false, 1);

        // Create check box for save authors
        let save_author = _create_checkbox("Save Author");
        save_author.set_halign(gtk::Align::End);
        third_config_box.pack_start(&save_author, false, true, 1);

        // Connect signal handler for formatting guidelines popup
        guideline_button.connect_clicked(|_| {
            show_popup(MD_SYNTAX_GUIDE);
            });

        // Clone references
        let author_clone = Rc::clone(&author);
        let username_clone = Rc::clone(&username);
        let password_clone = Rc::clone(&password);
        let hostname_clone = Rc::clone(&hostname);
        let port_clone = Rc::clone(&port);
        
        // Connect signal handler for save server config button
        save_button.connect_clicked(move |_| {
            // Create variables
            let author_set: String;
            let password_set: String;
            // Logic for saving server details
            if save_author.is_active() {
                author_set = author_clone.text().to_string();
            } else {
                author_set = String::new();
            }
            if save_password.is_active() {
                password_set = password_clone.text().to_string();
            } else {
                password_set = String::new();
            }

            update_server_config(
                &author_set,
                &username_clone.text(),
                &password_set,
                &hostname_clone.text(),
                &port_clone.text(),
            )
        });

        // Connect signal handler for clear server config button
        clear_button.connect_clicked(move |_| {
            update_server_config("", "", "", "", "");
        });

        // Connect signal handler for toggle button to toggle visibility of second_hbox
        show_button.connect_toggled(move |toggle_button| {
            // Toggle visibility of server details
            if toggle_button.is_active() {
                toggle_button.set_label("Hide Server Details");
                main_window.show_all();
            } else if !toggle_button.is_active() {
                toggle_button.set_label("Show Server Details");
                first_config_box.set_visible(false);
                second_config_box.set_visible(false);
                third_config_box.set_visible(false);
            }
        });

        // Clone references
        let title_clone = Rc::clone(&title);
        let headline_clone = Rc::clone(&headline);
        let tags_clone = Rc::clone(&tags);
        let body_clone = Rc::clone(&body);
        let author_clone = Rc::clone(&author);

        // Connect signal handler for preview button
        preview_button.connect_clicked(move |_| {
            // Get body text from the TextView buffer and convert it to a String.
            let body = textview_to_string(&body_clone);
            // Get the headline text from the Entry buffer and convert it to a String.
            let (title, headline, tags, body, author, html_file_path) = content_to_string(
                &title_clone.clone(),
                &headline_clone.clone(),
                &tags_clone.clone(),
                &body,
                &author_clone.clone(),
            );
            // Write to staging, if successful, show preview in browser
            match write_to_staging(&title, &headline, &tags, &body, &author, &html_file_path) {
                Ok(_) => {
                    preview_in_browser(&html_file_path);
                }
                Err(e) => eprintln!("Failed to write to staging: {}", e),
            }
        });

        // Connect signal handler for post button
        post_button.connect_clicked(move |_| {
            // Get body text from the TextView buffer and convert it to a String.
            let body = textview_to_string(&body);
            // Get the headline text from the Entry buffer and convert it to a String.
            let (title, headline, tags, body, author, html_file_path) =
                content_to_string(&title, &headline, &tags, &body, &author);
            // Get server details from Entry buffers and convert it to a String.
            let (username, password, hostname, port) =
                return_server_config(&username, &password, &hostname, &port);
            // Write content to staging, if successful, check server config, if successful, post to server
            match write_to_staging(
                &title.clone(),
                &headline.clone(),
                &tags.clone(),
                &body.clone(),
                &author.clone(),
                &html_file_path,
            ) {
                Ok(_) => {
                    println!("HTML created, posting to server...");
                    match post_to_server(
                        &author,
                        &username,
                        &password,
                        &hostname,
                        &port,
                        &html_file_path,
                    )
                    .expect("Failed to post to server")
                    {
                        StatusCode::OK => {
                            show_popup("Successfully posted to server!");
                        }
                        _ => show_popup("Failed to post to server. Please check server configuration and try again."),
                    }
                }

                Err(e) => show_popup(format!("Failed to create HTML: {}", e).as_str()),
            };
        });
    });
    nano_blog.run();
}
