// Module to define visual elements
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{
    Application, ApplicationWindow, Box, ButtonsType, CheckButton, CssProvider, DialogFlags, Entry,
    Label, MessageDialog, MessageType, Orientation, StyleContext, Window,
    STYLE_PROVIDER_PRIORITY_USER,
};

#[allow(dead_code)]
static DEFAULT_WIDTH: i32 = 256;
static DEFAULT_HEIGHT: i32 = 512;
const CSS: &str = include_str!("./css/style.css");

#[allow(dead_code)]
// Struct to hold data for entry box config
pub struct EntryConfig<'a> {
    pub parent: &'a gtk::Box,
    pub set_text: String,
    pub label_text: String,
    pub placeholder: String,
    pub tooltip: String,
    pub width: i32,
    pub fill: bool,
    pub visible: bool,
}

// Function to create main window
#[allow(dead_code)]
pub fn build_window(application: &Application) -> (ApplicationWindow, gtk::Box) {
    // main window

    let main_window = ApplicationWindow::builder()
        .type_(gtk::WindowType::Toplevel)
        .application(application)
        .title("Nano-Blog")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .border_width(10)
        .hide_titlebar_when_maximized(true)
        .build();

    // Create header bar
    let header = gtk::HeaderBar::new();
    header.set_title(Some("Nano-Blog"));
    header.set_show_close_button(true);
    main_window.set_titlebar(Some(&header));

    //Create manin vertical box layout.
    let main_vbox = gtk::Box::new(Vertical, 5);
    main_window.set_child(Some(&main_vbox));

    // Add CSS to main window
    let screen = GtkWindowExt::screen(&main_window).expect("Failed to get screen");
    // Set CSS provider
    let provider = CssProvider::new();
    let _ = provider
        .load_from_data(CSS.as_bytes())
        .expect("Failed to load CSS from file");
    // Add the provider to main window style context screen
    StyleContext::add_provider_for_screen(&screen, &provider, STYLE_PROVIDER_PRIORITY_USER);

    return (main_window, main_vbox);
}

pub fn show_popup(message: &str) {
    // Create a simple message dialog as a pop-up
    let dialog = MessageDialog::new(
        None::<&Window>,
        DialogFlags::MODAL,
        MessageType::Info,
        ButtonsType::Ok,
        message,
    );

    // Connect the response signal to close the dialog
    dialog.connect_response(|dialog, _| {
        dialog.close();
    });

    // Show the dialog
    dialog.run();
}

#[allow(dead_code)]
// Function to create a box
pub fn create_box(orientation: Orientation, visible: bool) -> gtk::Box {
    let new_box = gtk::Box::new(orientation, 5);
    // Determine if the box should be visible
    return if visible {
        new_box.set_visible(true);
        new_box
    } else {
        new_box.set_visible(false);
        new_box
    };
}

#[allow(dead_code)]
// Function to create a label
pub fn create_label(text: &str, tooltip: &str) -> Label {
    let label = Label::new(None);
    label.set_text(text);
    label.set_tooltip_text(Some(tooltip));
    label
}

#[allow(dead_code)]
// Function to create entry box
pub fn create_entry(set_text: &str, placeholder: &str, width: i32, visible: bool) -> Entry {
    let entry = Entry::new();
    entry.set_text(set_text);
    entry.set_placeholder_text(Option::from(placeholder));
    entry.set_width_chars(width);
    entry.set_visibility(visible);
    entry
}

#[allow(dead_code)]
// Function to create a labeled entry
pub fn create_labeled_entry(
    label: &str,
    set_text: &str,
    placeholder: &str,
    tooltip: &str,
    width: i32,
    visible: bool,
) -> (Entry, gtk::Box) {
    let vbox = Box::new(Vertical, 5);
    let hbox = Box::new(Orientation::Horizontal, 5);
    vbox.pack_start(&hbox, false, false, 0);
    hbox.pack_start(&create_label(label, tooltip), false, false, 3);
    let entry = create_entry(set_text, placeholder, width, visible);
    hbox.pack_end(&entry, true, true, 0);
    return (entry, vbox);
}

#[allow(dead_code)]
// Function to add child box to parent box
pub fn add_entry_to_parent(config: EntryConfig) -> Entry {
    let (title_text, labeled_entry): (Entry, Box) = create_labeled_entry(
        &config.label_text,
        &config.set_text,
        &config.placeholder,
        &config.tooltip,
        config.width,
        config.visible,
    );
    config
        .parent
        .pack_start(&labeled_entry, config.fill, config.fill, 0);
    return title_text;
}

//#[allow(dead_code)]
pub fn _create_checkbox(label: &str) -> CheckButton {
    let checkbox = CheckButton::with_label(label);
    return checkbox;
}
