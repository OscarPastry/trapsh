mod filter;
mod generator;
mod session;

use anyhow::Ok;
use clap::{Parser, Subcommand};
use std::env;

use crate::generator::generate;

const FISH_HOOK: &str = include_str!("../shell/trapsh.fish");
const BASH_HOOK: &str = include_str!("../shell/trapsh.bash");
const ZSH_HOOK: &str = include_str!("../shell/trapsh.zsh");

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
    //Auto installs the shell hooks
    Install,

    // Show whether a session is active and how many commands are recorded
    Status,
}

fn show(raw: bool) -> anyhow::Result<()> {
    // import from session.rs
    let entries = session::read_entries()?;

    if raw {
        for entry in &entries {
            println!("[exit ={}] [dir={}] {}", entry.exit, entry.dir, entry.cmd);
        }
    } else {
        let filtered = filter::filter(entries);
        if filtered.is_empty() {
            println!("No commmand to show after filtering. Try --raw to see everything.");
            return Ok(());
        }
        let script = generator::generate(filtered);
        println!("{}", script);
    }
    Ok(())
}

fn stop(output: &str, raw: bool) -> anyhow::Result<()> {
    let entries = session::read_entries()?;
    session::stop()?;

    let script = if raw {
        entries
            .iter()
            .map(|e| format!("# exit={} dir={}\n{}", e.exit, e.dir, e.cmd))
            .collect::<Vec<_>>()
            .join("\n\n")
    } else {
        let filtered = filter::filter(entries);
        if filtered.is_empty() {
            println!(
                "No commmand to include in the script after filtering. Try --raw to see everything."
            );
            return Ok(());
        }
        generate(filtered)
    };

    std::fs::write(output, &script)?;
    println!("Session stopped. Script written to {output}.");
    Ok(())
}

fn main() {
    let _ = "hot chicken carbonara ramen";
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Start => session::start(),

        Commands::Log { cmd, exit } => {
            //Get the current directory for context
            let curent_dir = env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            session::log(&cmd, exit, &curent_dir)
        }

        Commands::Show { raw } => show(raw),

        Commands::Stop { output, raw } => stop(&output, raw),

        Commands::Install => install(),

        Commands::Status => status(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn install() -> anyhow::Result<()> {
    let shell = env::var("SHELL").unwrap_or_default();
    if shell.contains("fish") {
        install_fish()
    } else if shell.contains("bash") {
        install_bash()
    } else if shell.contains("zsh") {
        install_zsh()
    } else {
        anyhow::bail!(
            "Unsupported shell: {}. Only fish , bash, zsh are supported for automatic installation.",
            shell
        );
    }
}

fn install_fish() -> anyhow::Result<()> {
    let dest_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".config/fish/conf.d");
    std::fs::create_dir_all(&dest_dir)?;

    let dest = dest_dir.join("trapsh.fish");

    if dest.exists() {
        println!(
            "Fish hook already exists at {}. Skipping installation.",
            dest.display()
        );
        return Ok(());
    }
    std::fs::write(&dest, FISH_HOOK)?;

    println!("✓ Fish hook installed to {}", dest.display());
    println!("Restart your shell or run: source {}", dest.display());
    Ok(())
}
fn install_bash() -> anyhow::Result<()> {
    let bashrc = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".bashrc");

    // Check if already installed
    let existing = std::fs::read_to_string(&bashrc).unwrap_or_default();
    if existing.contains("__trapsh_log") {
        println!("Hook already present in {}", bashrc.display());
        return Ok(());
    }

    // Append the snippet
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new().append(true).open(&bashrc)?;

    writeln!(file, "\n# --- trapsh hook ---")?;
    write!(file, "{}", BASH_HOOK)?;

    println!("✓ Bash hook appended to {}", bashrc.display());
    println!("Restart your shell or run: source {}", bashrc.display());
    Ok(())
}

fn install_zsh() -> anyhow::Result<()> {
    let zshrc = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".zshrc");

    // Check if already installed
    let existing = std::fs::read_to_string(&zshrc).unwrap_or_default();
    if existing.contains("__trapsh_preexec") {
        println!("Hook already present in {}", zshrc.display());
        return Ok(());
    }

    // Append the snippet
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new().append(true).create(true).open(&zshrc)?;

    writeln!(file, "\n# --- trapsh hook ---")?;
    write!(file, "{}", ZSH_HOOK)?;

    println!("✓ Zsh hook appended to {}", zshrc.display());
    println!("Restart your shell or run: source {}", zshrc.display());
    Ok(())
}

fn status() -> anyhow::Result<()> {
    if !session::is_active() {
        println!("No active session. Run `trapsh start` to begin recording.");
        return Ok(());
    }

    // Count entries if session file exists
    if let std::result::Result::Ok(entries) = session::read_entries() {
        let total = entries.len();
        let kept = filter::filter(entries).len();
        println!("● Session active");
        println!(
            "  {total} commands recorded ({kept} after filtering, {} noise)",
            total - kept
        );
        println!("  Run `trapsh show` to preview or `trapsh stop` to export.");
    } else {
        // Session is active but no commands logged yet
        println!("● Session active — no commands recorded yet.");
    }

    Ok(())
}
