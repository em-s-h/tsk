set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'clear' command:\n"
reset_file

echo "Test: clear all tasks"
## {{{
out=$(tsk_d clear)
if [[ $SHOW_OUT ]]; then
    echo -e "$out"
fi

[[ $(tail -n 1 <<< "$out") == "No tasks to print" ]]
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: do nothing with undone tasks"
## {{{
n=$(tsk_d add 'one'; tsk_d add 'one'; tsk_d add 'other'; tsk_d do 1,2)
out=$(tsk_d clear)
if [[ $SHOW_OUT ]]; then
    echo -e "$out"
fi

grep -qe "\s1.*[\s]*" <<< "$out"
echo "${GRE}OK${NC}"
## }}}
