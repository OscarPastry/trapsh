# trapsh bash hook — add this to your ~/.bashrc
#
# Uses PROMPT_COMMAND to log every command while a trapsh session is active.
# Safe to leave permanently — trapsh log is a no-op when no session is running.

__trapsh_log() {
    local exit_code=$?
    local last_cmd
    last_cmd=$(HISTTIMEFORMAT= history 1 | sed 's/^[ ]*[0-9]*[ ]*//')
    trapsh log "$last_cmd" "$exit_code" 2>/dev/null
}

# Flush history immediately so history 1 always returns the latest command
HISTCONTROL=
shopt -s histappend
PROMPT_COMMAND="history -a; __trapsh_log${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
