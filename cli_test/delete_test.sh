set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'delete' command:\n"
reset_file

echo "Test: delete a task"
## {{{
out=$(tsk_d delete 1)
if [[ $SHOW_OUT ]]; then
    echo -e "$out"
fi

[[ $(tail -n 1 <<< "$out") == "No tasks to print" ]]
echo "${GRE}OK${NC}"
## }}}
