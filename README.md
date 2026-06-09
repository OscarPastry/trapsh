
# trapsh
 
Record your fish shell session and replay it as a clean bash script.
 
## Install
 
```bash
cargo install --path .
 
# Drop the fish hook into your config
cp shell/trapsh.fish ~/.config/fish/conf.d/trapsh.fish
```
 
## Usage
 
```bash
trapsh start          # begin recording
 
# ... do your thing ...
 
trapsh show           # preview the script (filtered)
trapsh show --raw     # preview everything including failed/noisy commands
 
trapsh stop           # write script to ./trapsh_out.sh and end session
trapsh stop -o setup.sh   # write to a custom file
trapsh stop --raw     # skip noise filtering
```
