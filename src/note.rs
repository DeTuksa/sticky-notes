use serde::{ Deserialize, Serialize };
use std::fs;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: usize,
    pub text: String,
    pub created_at: String
}

impl Note {
    pub fn new(id: usize, text: String, created_at: String) -> Self {
        Note {
            id,
            text,
            created_at
        }
    }

    pub fn save_notes(notes: &[Note], filename: &str) -> io::Result<()> {
        let serialised = serde_json::to_string_pretty(notes)?;
        fs::write(filename, serialised)?;
        Ok(())
    }

    pub fn load_notes(filename: &str) -> io::Result<Vec<Note>> {
        let contents = fs::read_to_string(filename)?;
        let notes: Vec<Note> = serde_json::from_str(&contents)?;
        Ok(notes)
    }
}