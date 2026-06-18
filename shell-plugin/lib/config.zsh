#!/usr/bin/env zsh

# Configuration variables for lucard plugin
# Using typeset to keep variables local to plugin scope and prevent public exposure

typeset -h _LUCARD_BIN="${LUCARD_BIN:-lucard}"
typeset -h _LUCARD_CONVERSATION_PATTERN=":"
typeset -h _LUCARD_MAX_COMMIT_DIFF="${LUCARD_MAX_COMMIT_DIFF:-100000}"
typeset -h _LUCARD_DELIMITER='\s\s+'
typeset -h _LUCARD_PREVIEW_WINDOW="--preview-window=top:75%:wrap:border-sharp"

# Detect fd command - Ubuntu/Debian use 'fdfind', others use 'fd'
typeset -h _LUCARD_FD_CMD="$(command -v fdfind 2>/dev/null || command -v fd 2>/dev/null || echo 'fd')"

# Detect bat command - use bat if available, otherwise fall back to cat
if command -v bat &>/dev/null; then
    typeset -h _LUCARD_CAT_CMD="bat --color=always --style=numbers,changes --line-range=:500"
else
    typeset -h _LUCARD_CAT_CMD="cat"
fi

# Commands cache - loaded lazily on first use
typeset -h _LUCARD_COMMANDS=""

# Store active agent ID in a local variable (session-scoped)
# Default to "lucard" agent
export _LUCARD_ACTIVE_AGENT=lucard

# Store conversation ID in a temporary variable (local to plugin)
export _LUCARD_CONVERSATION_ID=""
