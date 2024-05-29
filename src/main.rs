use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    // Initialize GTK application
    let application = Application::builder()
        .application_id("com.example.myapp")
        .build();

    // Connect to "activate" signal to open a new window
    application.connect_activate(|app| {
        // Create a new window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .default_width(320)
            .default_height(200)
            .build();

        // Show the window
        window.show();
    });

    // Run the application
    application.run();
}
