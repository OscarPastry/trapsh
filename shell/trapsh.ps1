# trapsh PowerShell hook

$global:TrapshHookInstalled = $true

# Rename the existing prompt function to preserve it
if (Get-Command prompt -ErrorAction SilentlyContinue) {
    Rename-Item -Path "Function:\prompt" -NewName "trapsh_original_prompt" -ErrorAction SilentlyContinue
}

function prompt {
    # Capture the success status of the last command immediately
    $is_success = $?
    $last_exit = $LASTEXITCODE

    $last_cmd = Get-History -Count 1
    if ($null -ne $last_cmd) {
        $exit_code = if ($is_success) { 0 } else { if ($last_exit -ne 0) { $last_exit } else { 1 } }
        # Log it to trapsh
        try {
            trapsh log "$($last_cmd.CommandLine)" "$exit_code" *>$null
        } catch {}
    }

    # Call the original prompt if it exists, otherwise use a fallback
    if (Get-Command trapsh_original_prompt -ErrorAction SilentlyContinue) {
        trapsh_original_prompt
    } else {
        "PS $($executionContext.SessionState.Path.CurrentLocation)$('>' * ($nestedPromptLevel + 1)) "
    }
}
