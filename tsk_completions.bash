#!/bin/bash

#
# Put this file in a place that is sourced by `~/.bashrc` or copy the following
# code to the bottom of your `.bashrc`.
#

_tsk_comp() {
    local current=${COMP_WORDS[COMP_CWORD]}
    local comp=$(tsk --generate-shell-completions "${COMP_WORDS[*]} $current,$COMP_CWORD")
    local suc=$?

    if [[ $suc -eq 0 ]]; then
        if [[ $current == "=" ]]; then local current=""; fi

        if [[ $comp =~ "'" ]]; then
            COMPREPLY=( "\"$(compgen -W "$comp" -- "$current")\"" )
        else
            COMPREPLY=( $(compgen -W "$comp" -- "$current") )
        fi
    fi
}

complete -F _tsk_comp tsk
