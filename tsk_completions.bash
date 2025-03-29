#!/bin/bash

_tsk_comp() {
    local len=${#COMP_WORDS[@]}

    # These commands don't require additional args.
    local leave_comp="print clear"
    if [[ $leave_comp =~ "${COMP_WORDS[1]}" && $len -ne 2 ]]; then
        return
    fi
    local current=${COMP_WORDS[COMP_CWORD]}
    local prev=${COMP_WORDS[$len - 2]}

    # Complete `--add-to` values.
    # This way looks better, and is easier to change in the future.
    [[ ${COMP_WORDS[*]} =~ "-t" || ${COMP_WORDS[*]} =~ "--add-to" ]]; local has_opt=$?
    [[ $current == "=" || $prev == "=" ]]; local near_eq=$?

    # Empty string shows all possible completion values.
    [[ $current == "=" ]] && local current=""

    if [[ $has_opt -eq 0 && $near_eq -eq 0 ]]; then
        COMPREPLY=( $(compgen -W "top bottom" -- "$current") )
        return
    fi

    # Complete `--subtask` values.
    [[ ${COMP_WORDS[*]} =~ "-s" || ${COMP_WORDS[*]} =~ "--subtask" ]]; local has_opt=$?

    if [[ $has_opt -eq 0 && $near_eq -eq 0 ]]; then
        local t_count=$(grep -c 'subtasks' ~/.local/share/tsk/tasks.json)

        # {x..y} only accepts literals, this is a workaround.
        local t_ids=$(for i in $(eval echo "{1..$t_count}"); do echo "$i"; done)
        COMPREPLY=( $(compgen -W "$t_ids" -- "$current") )
        return
    fi

    # Complete options.
    if [[ $current =~ "--" ]]; then
        COMPREPLY=( $(compgen -W "--help --version --no-color --all --add-to --subtask" -- "$current") )

    elif [[ $current =~ "-" ]]; then
        COMPREPLY=( $(compgen -W "-h -v -c -a -t -s" -- "$current") )

    # Normal complete.
    elif [[ $len -le 2 ]]; then
        COMPREPLY=( $(compgen -W "print add do undo move swap append edit delete clear" -- "$current") )
    fi
}

complete -F _tsk_comp tsk
