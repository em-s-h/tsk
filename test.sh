#!/bin/bash 

cargo t -j 1 -- --test-threads 1
cargo b

RED=$(tput setaf 1) # Red
GRE=$(tput setaf 2) # Green
NC=$(tput sgr0)     # No color & format

task_f=~/.local/share/tsk/tasks

[[ $1 == "--show-output" ]] && show_out=true

tsk_d() {
    # {{{
    ./target/debug/tsk "$@"
}
# }}}

reset_file() {
    # {{{
    printf "" > $1
    for i in {0..4}; do
        echo "[ ] Test line $i" >> $1
    done
}
# }}}

get_first_ln() {
    # {{{
    head -n 1 $1
}
# }}}

get_last_ln() {
    # {{{
    tail -n 1 $1
}
# }}}

echo -e "\nNow running cli tests\n"

if [[ -f $task_f ]]; then
    # {{{
    echo "Backing up existing 'tasks' file"
    task_f=~/.local/share/tsk/tasks

    cp $task_f ~/.local/share/tsk/tasks.bak
    printf "" > $task_f

    for i in {0..4}; do
        echo "[ ] Test line $i" >> $task_f
    done
fi
# }}}

echo -e "\nTesting 'add' command:\n"
# {{{

echo "Test: Add to top"
## {{{

echo "- No flag"
### {{{
task="Test add task"

if [[ $show_out ]]; then
    tsk_d add "$task"
else
    tsk_d add "$task" &> /dev/null
fi
task="[ ] $task"

first_ln=$(get_first_ln $task_f)

if [[ $first_ln == *"$task"* ]]; then
    echo "${GRE}Success${NC}"
else
    echo "${RED}Fail${NC}"
    echo "Expected: $task"
    echo "File: $first_ln"
    exit 1
fi
echo
### }}}

echo "- With flag"
### {{{
task="Test add task with flag"

if [[ $show_out ]]; then 
    tsk_d add -top "$task"
else 
    tsk_d add -top "$task" &> /dev/null
fi
task="[ ] $task"

first_ln=$(get_first_ln $task_f)

if [[ $first_ln == *"$task"* ]]; then
    echo "${GRE}Success${NC}"
else
    echo "${RED}Fail${NC}"
    echo "Expected: $task"
    echo "File: $first_ln"
    exit 1
fi
echo
### }}}

## }}}

reset_file $task_f

echo "Test: Add to bottom"
## {{{
task="Test add task"

if [[ $show_out ]]; then
    tsk_d add -bot "$task"
else
    tsk_d add -bot "$task" &> /dev/null
fi
task="[ ] $task"

last_ln=$(get_last_ln $task_f)

if [[ $last_ln == *"$task"* ]]; then
    echo "${GRE}Success${NC}"
else
    echo "${RED}Fail${NC}"
    echo "Expected: $task"
    echo "File: $last_ln"
    exit 1
fi
echo
## }}}

reset_file $task_f

echo "Test: Remove '[ ]' & '[X]' from input"
## {{{
task="[ ] Test add task"

if [[ $show_out ]]; then
    tsk_d add -bot "[X] $task"
    tsk_d add "$task"
else
    tsk_d add -bot "[X] $task" &> /dev/null
    tsk_d add "$task" &> /dev/null
fi

first_ln=$(get_first_ln $task_f)
last_ln=$(get_last_ln $task_f)

if [[ $last_ln == *"$task"* && $first_ln == *"$task"* ]]; then
    echo "${GRE}Success${NC}"
else
    echo "${RED}Fail${NC}"
    echo "Expected: $task"
    echo "File: $first_ln & $last_ln"
    exit 1
fi
echo
## }}}

reset_file $task_f

echo "Test: Fail when passing empty string"
## {{{

if [[ $show_out ]]; then
    tsk_d add 
    out=$?
else
    out=$(tsk_d add &> /dev/null; echo $?)
fi

if [[ $out -eq 1 ]]; then
    echo "${GRE}Success${NC}"
else
    echo "${RED}Fail${NC}"
    echo "Exit code: $out"
    exit 1
fi
echo
## }}}

echo "Test: Fail when passing wrong option"
## {{{

if [[ $show_out ]]; then
    tsk_d add -not
    out=$?
else
    out=$(tsk_d add -not &> /dev/null; echo $?)
fi

if [[ $out -eq 1 ]]; then
    echo "${GRE}Success${NC}"
else
    echo "${RED}Fail${NC}"
    echo "Exit code: $out"
    exit 1
fi
echo
## }}}

# }}}

echo -e "Testing 'do' command:\n"
# {{{
# }}}
