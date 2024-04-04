mod gui;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = Application::builder()
    .application_id("com.asas.stickynotes")
    .build();

    app.connect_activate(|app| {
        gui::build_ui(&app)
    });

    app.run();
}
