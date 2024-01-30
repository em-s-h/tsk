#!/bin/bash 

[[ $1 == "--show-output" ]] && show_out=true

if [[ $show_out ]]; then
    cargo t -j 1 -- --test-threads 1 --nocapture
else
    cargo t -j 1 -- --test-threads 1
fi
[[ $? -ne 0 ]] && exit 1
cargo b
echo

tsk_d() {
    # {{{
    ./target/debug/tsk "$@"
}
# }}}

reset_file() {
    # {{{
    printf "" > $task_f
    for i in {0..4}; do
        if [[ $1 == "done" ]]; then
            echo "[X] Test line $i" >> $task_f
        else
            echo "[ ] Test line $i" >> $task_f
        fi
    done
}
# }}}

get_first_ln() {
    # {{{
    echo "$(head -n 1 $task_f)"
}
# }}}

get_last_ln() {
    # {{{
    echo "$(tail -n 1 $task_f)"
}
# }}}

check() {
    # {{{
    local out=1
    if [[ "$3" == "ne" ]]; then
        [[ "$1" != *"$2"* ]]
        out=$?
    else
        [[ "$1" == *"$2"* ]]
        out=$?
    fi

    if [[ $out -eq 0 ]]; then
        echo "${GRE}Success${NC}"
    else
        echo "${RED}Fail${NC}"
        echo "Left: $1"
        echo "Right: $2"
        exit 1
    fi
}
# }}}

task_f=~/.local/share/tsk/tasks
RED=$(tput setaf 1) # Red
GRE=$(tput setaf 2) # Green
NC=$(tput sgr0)     # No color & format

if [[ -f $task_f ]]; then
    # {{{
    if [[ ! -f ~/.local/share/tsk/tasks.bak ]]; then
        echo "Backing up existing 'tasks' file"
        cp $task_f ~/.local/share/tsk/tasks.bak
    fi
    reset_file
fi
# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'add' command:\n"

# {{{

echo "Test: Add to top"
## {{{

echo "- No flag"
### {{{

task="Test add task"
out=$(tsk_d add "$task")
task="[ ] $task"

if [[ $show_out ]]; then
    echo "$out"
fi
first_ln=$(get_first_ln)

check "$task" "$first_ln"
echo
### }}}

echo "- With flag"
### {{{
task="Test add task with flag"
out=$(tsk_d add -top "$task")
task="[ ] $task"

if [[ $show_out ]]; then 
    echo "$out"
fi
first_ln=$(get_first_ln)

check "$task" "$first_ln"
echo
### }}}

## }}}

reset_file 

echo "Test: Add to bottom"
## {{{
task="Test add task"
out=$(tsk_d add -bot "$task")
task="[ ] $task"

if [[ $show_out ]]; then
    echo "$out"
fi

last_ln=$(get_last_ln)

check "$task" "$last_ln"
echo
## }}}

reset_file 

echo "Test: Remove '[ ]' & '[X]' from input"
## {{{
task="[ ] Test add task"
out=$(tsk_d add -bot "[X] $task")
out2=$(tsk_d add "$task")

if [[ $show_out ]]; then
    echo -e "$out \n $out2"
fi

first_ln=$(get_first_ln)
last_ln=$(get_last_ln)

check "$task" "$first_ln"
check "$task" "$last_ln"
echo
## }}}

echo "Test: Fail when passing empty string or wrong option"
## {{{
out=$(tsk_d add 2>&1) ; ex=$?
out2=$(tsk_d add -not 2>&1) ; ex2=$?

if [[ $show_out ]]; then
    echo -e "$out \n $out2"
fi

check "$ex" 1 ; check "$ex2" 1
echo
## }}}

# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'do' command:\n"
reset_file

# {{{

echo "Test: Mark one"
## {{{
out=$(tsk_d do 1)
if [[ $show_out ]]; then
    echo "$out"
fi

first_ln=$(get_first_ln)
check "$first_ln" "[X]"
echo
## }}}

reset_file

echo "Test: Mark list"
## {{{
out=$(tsk_d do "1,2,3")
if [[ $show_out ]]; then
    echo "$out"
fi

lines=$(head -n 3 $task_f)
old_ifs=$IFS
IFS=$'\n'

for l in $lines; do
    check "$l" "[X]"
done

IFS=$old_ifs
echo
## }}}

reset_file

echo "Test: Mark range"
## {{{
out=$(tsk_d do "1..3")
if [[ $show_out ]]; then
    echo "$out"
fi

lines=$(head -n 3 $task_f)
old_ifs=$IFS
IFS=$'\n'

for l in $lines; do
    check "$l" "[X]"
done

IFS=$old_ifs
echo
## }}}

reset_file

echo "Test: Mark all"
## {{{
out=$(tsk_d do -all)
if [[ $show_out ]]; then
    echo "$out"
fi
lines=$(< $task_f)

check "$l" "[ ]" "ne"
echo
## }}}

echo "Test: Fail when passing wrong id or option"
## {{{
out=$(tsk_d do 50 2>&1) ; ex=$?
out2=$(tsk_d do -not 2>&1) ; ex2=$?
if [[ $show_out ]]; then
    echo -e "$out \n $out2"
fi

check "$ex" 1 ; check "$ex2" 1
echo
## }}}

# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'undo' command:\n"
reset_file "done"

# {{{

echo "Test: Unmark one"
## {{{
out=$(tsk_d undo 1)
if [[ $show_out ]]; then
    echo "$out"
fi

first_ln=$(get_first_ln)
check "$first_ln" "[ ]"
echo
## }}}

reset_file "done"

echo "Test: Unmark list"
## {{{
out=$(tsk_d undo "1,2,3")
if [[ $show_out ]]; then
    echo "$out"
fi

lines=$(head -n 3 $task_f)
old_ifs=$IFS
IFS=$'\n'

for l in $lines; do
    check "$l" "[ ]"
done

IFS=$old_ifs
echo
## }}}

reset_file "done"

echo "Test: Unmark range"
## {{{
out=$(tsk_d undo "1..3")
if [[ $show_out ]]; then
    echo "$out"
fi

lines=$(head -n 3 $task_f)
old_ifs=$IFS
IFS=$'\n'

for l in $lines; do
    check "$l" "[ ]"
done

IFS=$old_ifs
echo
## }}}

reset_file "done"

echo "Test: Unmark all"
## {{{
out=$(tsk_d undo -all)
if [[ $show_out ]]; then
    echo "$out"
fi
lines=$(< $task_f)

check "$l" "[X]" "ne"
echo
## }}}

# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'move' command:\n"
reset_file

# {{{

echo "Test: Move first to last"
## {{{
first_ln=$(get_first_ln)
out=$(tsk_d move 1 5)
if [[ $show_out ]]; then
    echo "$out"
fi

last_ln=$(get_last_ln)
check "$first_ln" "$last_ln"
echo
## }}}

reset_file

echo "Test: Move last to first"
## {{{
last_ln=$(get_last_ln)
out=$(tsk_d move 5 1)
if [[ $show_out ]]; then
    echo "$out"
fi

first_ln=$(get_first_ln)
check "$last_ln" "$first_ln"
echo
## }}}

# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'append' command:\n"
reset_file "done"

# {{{

echo "Test: Undo done task when appending"
## {{{
out=$(tsk_d append 1 "Append")
if [[ $show_out ]]; then
    echo -e "$out"
fi
first_ln=$(get_first_ln)

check "$first_ln" "[ ] Test line 0 Append"
echo
## }}}

# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'edit' command:\n"
reset_file "done"

# {{{

echo "Test: Undo done task when editing"
## {{{
out=$(tsk_d edit 1 "Edited")
if [[ $show_out ]]; then
    echo -e "$out"
fi
first_ln=$(get_first_ln)

check "$first_ln" "[ ] Edited"
echo
## }}}

# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'delete' command:\n"
reset_file

# {{{

echo "Test: delete a task"
## {{{
out=$(tsk_d delete 1)
if [[ $show_out ]]; then
    echo -e "$out"
fi
ln_count=$(wc -l $task_f)

check "$ln_count" 4
echo
## }}}

# }}}

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'clear' command:\n"
reset_file "done"

# {{{

echo "Test: clear all tasks"
## {{{
out=$(tsk_d clear)
if [[ $show_out ]]; then
    echo -e "$out"
fi
ln_count=$(wc -l $task_f)

check "$ln_count" 0
echo
## }}}

reset_file

echo "Test: do nothing with undone tasks"
## {{{
out=$(tsk do 1,2)
out2=$(tsk_d clear)
if [[ $show_out ]]; then
    echo -e "$out \n $out2"
fi
ln_count=$(wc -l $task_f)

check "$ln_count" 3
echo
## }}}

# }}}

if [[ -f ~/.local/share/tsk/tasks.bak ]]; then
    echo "Recovering backup"
    mv ~/.local/share/tsk/tasks.bak $task_f
fi
