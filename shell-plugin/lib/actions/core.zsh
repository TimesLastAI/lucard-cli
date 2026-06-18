#!/usr/bin/env zsh

# Core action handlers for basic lucard operations

# Action handler: Start a new conversation
function _lucard_action_new() {
    _LUCARD_CONVERSATION_ID=""
    _LUCARD_ACTIVE_AGENT="lucard"
    
    echo
    _lucard_exec banner
    _lucard_reset
}

# Action handler: Show session info
function _lucard_action_info() {
    echo
    if [[ -n "$_LUCARD_CONVERSATION_ID" ]]; then
        _lucard_exec info --cid "$_LUCARD_CONVERSATION_ID"
    else
        _lucard_exec info
    fi
    _lucard_reset
}

# Action handler: Show environment info
function _lucard_action_env() {
    echo
    _lucard_exec env
    _lucard_reset
}

# Action handler: Dump conversation
function _lucard_action_dump() {
    local input_text="$1"
    if [[ "$input_text" == "html" ]]; then
        _lucard_handle_conversation_command "dump" "--html"
    else
        _lucard_handle_conversation_command "dump"
    fi
}

# Action handler: Compact conversation
function _lucard_action_compact() {
    _lucard_handle_conversation_command "compact"
}

# Action handler: Retry last message
function _lucard_action_retry() {
    _lucard_handle_conversation_command "retry"
}

# Helper function to handle conversation commands that require an active conversation
function _lucard_handle_conversation_command() {
    local subcommand="$1"
    shift  # Remove first argument, remaining args become extra parameters
    
    echo
    
    # Check if LUCARD_CONVERSATION_ID is set
    if [[ -z "$_LUCARD_CONVERSATION_ID" ]]; then
        _lucard_log error "No active conversation. Start a conversation first or use :list to see existing ones"
        _lucard_reset
        return 0
    fi
    
    # Execute the conversation command with conversation ID and any extra arguments
    _lucard_exec conversation "$subcommand" "$_LUCARD_CONVERSATION_ID" "$@"
    
    _lucard_reset
    return 0
}
