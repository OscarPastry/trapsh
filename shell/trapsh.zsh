# trapsh zsh hook — add this to your ~/.zshrc

# Capture the command right before it executes
__trapsh_preexec() {
    __trapsh_last_cmd="$1"
}

# Capture the exit code immediately after it finishes
__trapsh_precmd() {
    local exit_code=$?
    
    # Only log if we actually recorded a command
    if [[ -n "$__trapsh_last_cmd" ]]; then
        trapsh log "$__trapsh_last_cmd" "$exit_code" 2>/dev/null
        # Clear it so we don't log it again if the user just hits Enter
        __trapsh_last_cmd=""
    fi
}

# Register the hooks
autoload -Uz add-zsh-hook
add-zsh-hook preexec __trapsh_preexec
add-zsh-hook precmd __trapsh_precmd
