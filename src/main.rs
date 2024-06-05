extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType, Box as GtkBox};

use std::process::{Command, Stdio};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GTK
    gtk::init()?;

    // Create a new window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Application Box");
    window.set_default_size(400, 300);

    // Create a box container
    let box_container = GtkBox::new(gtk::Orientation::Vertical, 0);
    window.add(&box_container);

    // Spawn the application
    let mut command = Command::new("gedit"); // Using gedit text editor on Linux
    let _child = command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Show the window
    window.show_all();

    // Connect the close signal
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Start the GTK main loop
    gtk::main();

    Ok(())
}
