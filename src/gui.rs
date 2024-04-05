extern crate gtk;

use gtk::glib::DateTime;
use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk::Orientation;

use crate::note::Note;

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
        let note = [Note::new(
            1, entry_clone.text().to_string(), "Now".to_string()
        )];
        if let Err(err) = Note::save_notes(&note, "notes.json") {
            eprintln!("Error saving notes: {}", err);
        } else {
            println!("Notes saved successfully!");
        }
    });

    vbox.add(&entry);
    vbox.add(&add_button);

    window.set_child(Some(&vbox));

    window.show_all();

    if let Ok(notes) = Note::load_notes("notes.json") {
        for note in notes {
            let note_window = ApplicationWindow::builder()
            .application(app)
            .title(&format!("Note {}", note.id))
            .default_width(300)
            .default_height(200)
            .build();

        let note_label = gtk::Label::new(Some(&note.text));
        note_window.set_child(Some(&note_label));
        note_window.show_all();
        }
    } else {
        eprintln!("Error loading notes.");
    }
}