#!/usr/bin/env zsh

# Authentication action handlers

# Action handler: Login to provider
function _lucard_action_login() {
    echo
    local selected
    selected=$(_lucard_select_provider)
    if [[ -n "$selected" ]]; then
        # Extract the second field (provider ID)
        local provider=$(echo "$selected" | awk '{print $2}')
        _lucard_exec provider login "$provider"
    fi
    _lucard_reset
}

# Action handler: Logout from provider
function _lucard_action_logout() {
    echo
    local selected
    selected=$(_lucard_select_provider "\[yes\]")
    if [[ -n "$selected" ]]; then
        # Extract the second field (provider ID)
        local provider=$(echo "$selected" | awk '{print $2}')
        _lucard_exec provider logout "$provider"
    fi
    _lucard_reset
}
