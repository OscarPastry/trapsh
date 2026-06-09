use std::result;

use crate::session::Entry;

const NOISY_COMMAND: &[&str] = &[
    "ls",
    "ll",
    "lsd",
    "l",
    "pwd",
    "clear",
    "cls",
    "history",
    "history -c",
    "history -w",
    "history -r",
    "man",
    "help",
    "which",
    "where",
    "type",
    "whatis",
    "cat",
    "less",
    "more",
    "head",
    "tail",
    "echo",
    "printf",
    "date",
    "cal",
    "uptime",
    "exit",
    "logout",
    "top",
    "htop",
    "ps",
    "free",
    "df",
    "du",
];

//Prefixes that indicate a noisy command even with arguments.
const NOISY_PREFIX: &[&str] = &[
    "git log",
    "git status",
    "git diff",
    "git branch",
    "git checkout",
    "git stash list",
    "cargo check",
];

//Editor commands - opening a file is not reproducible.
const EDITOR_COMMANDS: &[&str] = &["vim", "nvim", "emacs", "nano", "code", "subl", "atom"];

pub struct FilteredEntry{
    pub cmd: String,
    pub dir: String,
}


//Apply noise filtering to a list of raw entries.
// Returns only the commands worth keeping in a replay script.
pub fn filter(entries: Vec<Entry>) -> Vec<FilteredEntry>{
    let mut result: Vec<FilteredEntry> = Vec::new();

    for entry in entries{
        //Drop failed commands
        if entry.exit != 0{
            continue;
        }
        let cmd = entry.cmd.trim().to_string();

        let cmd.is_empty(){
            continue;
        }

    }
}
