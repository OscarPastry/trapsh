use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};

//A single recorded command entry.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub cmd: String,
    pub exit: i32,
    pub timestamp: i64, //unix timestamp
    pub dir: String,
}

//Returns the path to the active session file
// example ~/.local/share/trapsh/current.jsonl
pub fn session_path() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
    base.join("trapsh").join("current.jsonl")
}

//Return the path to lock file that marks an active session.
pub fn lock_path() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
    base.join("trapsh").join(".active")
}

//Return true if a session is currently active
pub fn is_active() -> bool {
    lock_path().exists()
}

//Start a new session Creates the data dir, lock file and clears any old session.
pub fn start() -> anyhow::Result<()> {
    let session_file = session_path();
    let lock_file = lock_path();

    //Create data dir if it doesn't exist
    if let Some(parent) = session_file.parent() {
        fs::create_dir_all(parent)?;
    }

    //Clear previous session data
    if session_file.exists() {
        fs::remove_file(&session_file)?;
    }
    //create lock file
    fs::write(&lock_file, "")?;
    println!(">>Session started. Run 'trapsh stop' when you're done.");

    Ok(())
}
