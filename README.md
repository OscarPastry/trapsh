# trapsh

> **Record your shell session and replay it as a clean, shareable bash script.**

`trapsh` acts as a transparent background recorder for your terminal. It hooks into your `fish` (or `bash`/`zsh` or `powershell`) session, records the commands you run, filters out the noise (like failed commands, typos, and syntax errors), and outputs a clean, ready-to-use bash script. 

Perfect for creating tutorials, sharing reproduction steps, or just automating your workflow without having to write the script from scratch.

---

## Demo

<img width="648" height="875" alt="trapsh demo" src="https://github.com/user-attachments/assets/264f6b4c-82f0-4d4c-95cf-94a7ca94d6ca" />

## Features

- **Noise Filtering**: Automatically removes failed commands, duplicate entries, and typos from the final output.
- **Cross-Shell Support**: Hook installation is supported for `fish`, `bash`, `zsh`, and `powershell`.
- **Clean Output**: Generates a standardized bash script (`trapsh_out.sh`) that you can immediately run or share.
- **Live Preview**: See what your final script will look like before you stop recording.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then run:

```bash
# Build and install from source
cargo install --path .

# Install the shell hooks (supports fish, bash, zsh, powershell)
trapsh install 
```

## Usage

Using `trapsh` is simple. Start a session, do your work, and stop it when you're done.

```bash
# 1. Begin recording your session
trapsh start

# 2. ... Run your commands normally ...
$ mkdir my_project
$ cd my_project
$ npm init -y

# 3. Check the status of your current session
trapsh status

# 4. Preview the generated script so far
trapsh show           # Filtered, clean preview
trapsh show --raw     # Show everything, including noise and failed commands

# 5. Stop recording and save the output
trapsh stop                   # Saves to ./trapsh_out.sh
trapsh stop -o setup.sh       # Saves to a custom file
trapsh stop --raw             # Save the unfiltered session
```

## How it Works

When you run `trapsh start`, it initializes a tracker that logs your shell history in real-time. Upon running `trapsh stop`, it analyzes the return codes of your executed commands, strips out anything that failed (unless `--raw` is passed), and writes the successful sequence to a file.

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open an issue or pull request.
