set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'add' command:\n"

echo "Test: Add to top"
## {{{

echo "- No flag"
### {{{

task="Test add task"
out=$(tsk_d add "$task")

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
### }}}

echo "- With flag"
### {{{
task="Test add task with flag"
out=$(tsk_d add -top "$task")

if [[ $SHOW_OUT ]]; then 
    echo "$out"
fi

grep -qe "\s1\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
### }}}

## }}}

reset_file 

echo "Test: Add to bottom"
## {{{
task="Test add task"
out=$(tsk_d add -bot "$task")

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

tail -n 1 <<< "$out" | grep -qE ".*$task"
echo "${GRE}OK${NC}"
## }}}

reset_file 

echo "Test: Fail when passing empty string or wrong option"
## {{{
set +e
out=$(tsk_d add 2>&1) ; ex=$?
out2=$(tsk_d add -not 2>&1) ; ex2=$?
set -e

if [[ $SHOW_OUT ]]; then
    echo -e "$out \n $out2"
fi

[[ $(($ex + $ex2)) -eq 2 ]]
echo "${GRE}OK${NC}"
## }}}

