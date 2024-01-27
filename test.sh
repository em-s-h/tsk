#!/bin/bash 

cargo t -j 1 -- --test-threads 1
cargo b

tsk_d() {
    ./target/debug/tsk $@
}

test_exit() {
    # {{{
    if [[ $? -eq $1 ]]; then
        echo "Test:$GRE Success"
    else
        echo "Test:$RED Fail"
    fi
}
# }}}

if [[ -f ~/.local/share/tsk/tasks ]]; then
    echo "Backing up existing 'tasks' file"
    task_f=~/.local/share/tsk/tasks

    cp $task_f ~/.local/share/tsk/tasks.bak
    printf "" > $task_f

    for i in {0..4}; do
        echo "[ ] Test line $i" >> $task_f
    done
fi
task_f=~/.local/share/tsk/tasks

echo "Testing 'add' command:"
# {{{

echo "Test: Add to top"
# {{{

task="Test add task"
tsk add "$task"
test_exit 0
# }}}

# }}}

