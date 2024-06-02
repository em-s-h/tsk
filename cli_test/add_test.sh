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

echo "Test: Add subtask to task top"
## {{{

tsk_d add -top "Test" &> /dev/null

echo "- No flag"
### {{{
task="sub-test1"
out=$(tsk_d add -sub 1 "$task")

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\.1\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
### }}}

echo "- With flag"
### {{{
task="sub-test2"
out=$(tsk_d add -top -sub 1 "$task")

if [[ $SHOW_OUT ]]; then 
    echo "$out"
fi

grep -qe "\s1\.1\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
### }}}

## }}}

reset_file 

echo "Test: Add subtask to task bottom"
## {{{
tsk_d add -top "Test" &> /dev/null
tsk_d add -sub 1 "subtest1" &> /dev/null
tsk_d add -sub 1 "subtest2" &> /dev/null
task="subtest3"
out=$(tsk_d add -bot -sub 1 "$task")

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\.3\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

reset_file 

echo "Test: Add subtask to subtask top"
## {{{

tsk_d add -top "Test" &> /dev/null
tsk_d add -sub 1 "Subtest" &> /dev/null

echo "- No flag"
### {{{
task="sub-sub-test1"
out=$(tsk_d add -sub 1.1 "$task")

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\.1\.1\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
### }}}

echo "- With flag"
### {{{
task="sub-sub-test2"
out=$(tsk_d add -top -sub 1.1 "$task")

if [[ $SHOW_OUT ]]; then 
    echo "$out"
fi

grep -qe "\s1\.1\.1\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
### }}}

## }}}

reset_file 

echo "Test: Add subtask to subtask bottom"
## {{{
tsk_d add -top "Test" &> /dev/null
tsk_d add -sub 1 "Subtest" &> /dev/null
tsk_d add -sub 1.1 "sub-sub-test1" &> /dev/null
tsk_d add -sub 1.1 "sub-sub-test2" &> /dev/null
task="sub-sub-test3"
out=$(tsk_d add -bot -sub 1.1 "$task")

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\.1\.3\..*$task" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

echo "Test: Fail when passing empty string or wrong option"
## {{{
set +e
out=$(tsk_d add 2>&1) ; ex=$?
out2=$(tsk_d add -not "a" 2>&1) ; ex2=$?
out3=$(tsk_d add -sub 50 "a" 2>&1) ; ex3=$?
out4=$(tsk_d add -sub 50.50 "a" 2>&1) ; ex4=$?
set -e

if [[ $SHOW_OUT ]]; then
    echo -e "$out \n $out2 \n $out3 \n $out4"
fi

[[ $(($ex + $ex2 + $ex3 + $ex4)) -eq 4 ]]
echo "${GRE}OK${NC}"
## }}}

