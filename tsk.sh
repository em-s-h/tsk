#!/bin/bash

_tsk_comp() {
    if [[ ${#COMP_WORDS[@]} != 2 && ${COMP_WORDS[1]} != "edit" ]] ;then
        return
    fi
    local comps=( $(compgen -W "--help --version --no-color print do undo clear add append edit move delete clear" -- "${COMP_WORDS[1]}") )

    if [[ ${COMP_WORDS[1]} == "edit" && ${#COMP_WORDS[@]} == 4 ]]; then
        local old_ifs="$IFS"
        IFS=$'\n'

        local items=( $(< ~/.local/share/tsk/tasks)  )
        local id=$(( ${COMP_WORDS[2]} - 1 ))

        if [[ $id -lt 0 ]]; then
            IFS="$old_ifs"
            return
        fi

        local item="${items[$id]}"
        local item=$( echo "${item/\[*\]/}" | xargs )

        COMPREPLY=( "'$(compgen -W "$item" -- "${COMP_WORDS[3]}")'" )
        IFS="$old_ifs"
    elif [[ ${COMP_WORDS[1]} != "edit" ]]; then
        COMPREPLY=("${comps[@]}")
    fi
}

complete -F _tsk_comp tsk
