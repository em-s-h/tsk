#!/bin/bash

_tsk_comp() {
    # These commands don't require additional args.
    local leave_comp="print clear"
    if [[ $leave_comp =~ "${COMP_WORDS[1]}" && ${#COMP_WORDS[@]} -ne 2 ]]; then
        return
    fi

    local req_id="do undo move swap append edit delete"
    local req_sec_id="move swap"
    local all_opt="do undo"

    # Complete options
    if [[ ${COMP_WORDS[1]} =~ "-" && ${#COMP_WORDS[@]} -le 2 ]]; then
        COMPREPLY=( $(compgen -W "--help --version --no-color" -- "${COMP_WORDS[COMP_CWORD]}") )

    # Complete id
    elif [[ $req_id =~ "${COMP_WORDS[1]}" && ${#COMP_WORDS[@]} -eq 3 ]]; then
        local ids=$(_get_ids)
        if [[ -z $ids ]]; then
            return
        fi
        if [[ $all_opt =~ "${COMP_WORDS[1]}" ]]; then
            ids="$ids -all"
        fi

        COMPREPLY=( $(compgen -W "$ids" -- "${COMP_WORDS[COMP_CWORD]}") )

    # Complete secondary id
    elif [[ $req_sec_id =~ "${COMP_WORDS[1]}" && ${#COMP_WORDS[@]} -eq 4 ]]; then
        local ids=$(_get_ids)
        if [[ -z $ids ]]; then
            return
        fi
        local ids=$(echo "${ids} " | sed "s/${COMP_WORDS[2]}\s//")
        COMPREPLY=( $(compgen -W "$ids" -- "${COMP_WORDS[COMP_CWORD]}") )

    # Complete "add" sub-options
    elif [[ ${COMP_WORDS[1]} == "add" && ${#COMP_WORDS[@]} -eq 3 ]]; then
        COMPREPLY=( $(compgen -W "-top -bot" -- "${COMP_WORDS[COMP_CWORD]}") )

    # Complete task to be edited
    elif [[ ${COMP_WORDS[1]} == "edit" && ${#COMP_WORDS[@]} -eq 4 ]]; then
        # {{{
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

        COMPREPLY=( "'$(compgen -W "$item" -- "${COMP_WORDS[COMP_CWORD]}")'" )
        IFS="$old_ifs"
        # }}}

    # Normal complete
    elif [[ ${#COMP_WORDS[@]} -le 2 ]]; then
        COMPREPLY=( $(compgen -W "print add $req_id clear" -- "${COMP_WORDS[COMP_CWORD]}") )
    fi
}

_get_ids() {
    # {{{
    local ln_count=$(( $(wc -l ~/.local/share/tsk/tasks | sed "s/\/.*//g") ))
    local ids=""

    for ((i=1; i <= ln_count; i++)) do
        ids="$ids $i"
    done
    echo -n "$ids"
}
# }}}

complete -F _tsk_comp tsk
