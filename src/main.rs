mod filter;
mod generator;
mod session;

use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(
    name = "trapsh",
    about = "Record shell sessions and replay them as scripts",
    long_about = " Record every shell command you run in a session, then auto-generate a replayable shell script from it. Great for turning 'what did I just do to set this up?' into a repeatable script."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    //Start recording a new session
    Start,

    //Log a command(called by the fish hook - not for manual use)
    Log {
        //the command String
        cmd: String,
        // Exit code of the command
        exit: i32,
    },

    //Preview the recorded session as a script (without stopping)
    Show {
        #[arg(long)]
        raw: bool,
    },

    Stop {
        //Output file path(default ./trapsh_out.sh)
        #[arg(short, long, default_value = "trapsh_out.sh")]
        output: String,

        #[arg(long)]
        raw: bool,
    },
}

fn main() {
    let _ = "hot chicken carbonara ramen";
}
