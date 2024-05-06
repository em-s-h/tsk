#!/bin/bash

_tsk_comp() {
    # These commands don't require additional args.
    local leave_comp="print clear"
    if [[ $leave_comp =~ "${COMP_WORDS[1]}" && ${#COMP_WORDS[@]} -ne 2 ]]; then
        return
    fi

    #local req_id="do undo move swap append edit delete"
    #local req_sec_id="move swap"
    local all_opt="do undo"

    # Complete options
    if [[ ${COMP_WORDS[1]} =~ "-" && ${#COMP_WORDS[@]} -le 2 ]]; then
        COMPREPLY=( $(compgen -W "--help --version --no-color" -- "${COMP_WORDS[COMP_CWORD]}") )

    # Complete do and undo sub-option
    elif [[ $all_opt =~ "${COMP_WORDS[1]}" && ${#COMP_WORDS[@]} -eq 3 ]]; then
        COMPREPLY=( $(compgen -W "-all" -- "${COMP_WORDS[COMP_CWORD]}") )

    # Complete "add" sub-options
    elif [[ ${COMP_WORDS[1]} == "add" && ${#COMP_WORDS[@]} -eq 3 ]]; then
        COMPREPLY=( $(compgen -W "-top -bot" -- "${COMP_WORDS[COMP_CWORD]}") )

    # Normal complete
    elif [[ ${#COMP_WORDS[@]} -le 2 ]]; then
        COMPREPLY=( $(compgen -W "print add do undo move swap append edit delete clear" -- "${COMP_WORDS[COMP_CWORD]}") )
    fi
}

complete -F _tsk_comp tsk
