#!/bin/bash 
set -e

if [[ $1 == "--show-output" ]]; then
    export SHOW_OUT=true
fi

if [[ $SHOW_OUT ]]; then
    cargo t -j 1 -- --test-threads 1 --nocapture
else
    cargo t -j 1 -- --test-threads 1
fi
cargo b
echo

tsk_d() { # {{{
    ../target/debug/tsk "$@"
}
# }}}

reset_file() { # {{{
    printf "" > $TASK_F
}
# }}}

export -f tsk_d
export -f reset_file

export TASK_F=~/.local/share/tsk/tasks.json
export RED=$(tput setaf 1) # Red
export GRE=$(tput setaf 2) # Green
export NC=$(tput sgr0)     # No color & format

if [[ -f $TASK_F ]]; then
    # {{{
    if [[ ! -f ~/.local/share/tsk/tasks.json.bak ]]; then
        echo "Backing up existing 'tasks' file"
        mv $TASK_F ~/.local/share/tsk/tasks.json.bak
    fi
    printf "" > "$TASK_F"
fi
# }}}

bash ./add_test.sh
echo

bash ./do_test.sh
echo

bash ./undo_test.sh
echo

bash ./move_test.sh
echo

bash ./swap_test.sh
echo

bash ./append_test.sh
echo

bash ./edit_test.sh
echo

bash ./delete_test.sh
echo

bash ./clear_test.sh
echo

if [[ -f ~/.local/share/tsk/tasks.json.bak ]]; then
    echo "Recovering backup"
    mv ~/.local/share/tsk/tasks.json.bak $TASK_F
fi
