#!/bin/bash

_tsk_comp() {
    case "${COMP_WORDS[1]}" in
        "edit"|"do"|"undo"|"move"|"delete"|"append"|"add") 
        ;;
        *) [[ ${#COMP_WORDS[@]} -ne 2 ]] && return
        ;;
    esac

    local comps=( $(compgen -W "--help --version --no-color print do undo clear add append edit move delete clear" -- "${COMP_WORDS[1]}") )
    local req_id="edit do undo move delete append"
    local all_opt="do undo"

    if [[ $req_id =~ "${COMP_WORDS[1]}" && ${#COMP_WORDS[@]} -eq 3 ]]; then
        local ids=$(_get_ids)
        if [[ $all_opt =~ "${COMP_WORDS[1]}" ]]; then
            ids="$ids -all"
        fi
        COMPREPLY=( $(compgen -W "$ids" -- "${COMP_WORDS[2]}") )

    elif [[ ${COMP_WORDS[1]} == "add" && ${#COMP_WORDS[@]} -eq 3 ]]; then
        COMPREPLY=( $(compgen -W "-top -bot" -- "${COMP_WORDS[2]}") )

    elif [[ ${COMP_WORDS[1]} == "move" && ${#COMP_WORDS[@]} -eq 4 ]]; then
        local ids=$(_get_ids)
        COMPREPLY=( $(compgen -W "$ids" -- "${COMP_WORDS[3]}") )

    elif [[ ${COMP_WORDS[1]} == "edit" && ${#COMP_WORDS[@]} -eq 4 ]]; then
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

    elif [[ ${#COMP_WORDS[@]} -le 2 ]]; then
        COMPREPLY=("${comps[@]}")
    fi
}

_get_ids() {
    local ln_count=$(( $(wc -l ~/.local/share/tsk/tasks | sed "s/\/.*//g") ))
    local ids=1

    for ((i=2; i <= ln_count; i++)) do
        ids="$ids $i"
    done
    echo -n "$ids"
}

complete -F _tsk_comp tsk
