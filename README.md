
# trapsh
 
Record your fish shell session and replay it as a clean bash script.

## Demo
<img width="648" height="875" alt="readme1" src="https://github.com/user-attachments/assets/264f6b4c-82f0-4d4c-95cf-94a7ca94d6ca" />

## Install
 
```bash
cargo install --path .
 
# Auto shell hook installation for fish, bash and zsh
trapsh install 
```
 
## Usage
 
```bash
trapsh start          # begin recording
 
# ... do your thing ...


trapsh show           # preview the script (filtered)
trapsh show --raw     # preview everything including failed/noisy commands

trapsh status         # Show the state of the session, if active or not
 
trapsh stop           # write script to ./trapsh_out.sh and end session
trapsh stop -o setup.sh   # write to a custom file
trapsh stop --raw     # skip noise filtering
```
