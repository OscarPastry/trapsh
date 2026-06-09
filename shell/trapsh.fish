# trapsh.fish — drop this in ~/.config/fish/conf.d/
#
# Hooks into fish_postexec to log every command while a trapsh session is active.
# The Rust binary handles the "is a session active?" check, so this hook is safe
# to leave installed permanently — it's a no-op when no session is running.

function __trapsh_log --on-event fish_postexec
    # $argv[1] is the command string
    # $status is the exit code of the last command
    trapsh log $argv[1] $status 2>/dev/null
end
