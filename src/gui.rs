extern crate gtk;

use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk::Orientation;

pub fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
    .application(app)
    .title("Sticky Notes")
    .default_width(300)
    .default_height(200)
    .build();

    let entry = gtk::Entry::builder()
    .placeholder_text("Enter note here")
    .margin(12)
    .build();

    let vbox = gtk::Box::builder()
    .orientation(Orientation::Vertical)
    .spacing(6)
    .margin(12)
    .build();

    let add_button = gtk::Button::builder()
    .label("Add Note")
    .build();

    let entry_clone = entry.clone();


    add_button.connect_clicked(move |_| {
        println!("Note added: {}", entry_clone.text().as_str());
    });

    vbox.add(&entry);
    vbox.add(&add_button);

    window.set_child(Some(&vbox));

    window.show_all();
}