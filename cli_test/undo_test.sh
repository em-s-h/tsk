set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'undo' command:\n"
reset_file

echo "Test: Unmark one"
## {{{
out=$(tsk_d undo 1)
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\..*[\s].*" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark one subtask"
## {{{
tsk_d add "test" &> /dev/null
tsk_d add -sub 1 "sub-test" &> /dev/null
tsk_d do 1.1 &> /dev/null
out=$(tsk_d undo 1.1)

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\.1\..*[\s].*" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unark one sub-subtask"
## {{{
tsk_d add "test" &> /dev/null
tsk_d add -sub 1 "sub-test" &> /dev/null
tsk_d add -sub 1.1 "sub-sub-test" &> /dev/null
tsk_d do 1.1.1 &> /dev/null
out=$(tsk_d undo 1.1.1)

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\.1\.1\..*[\s].*" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark list"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'one' ; tsk_d add 'one')
tsk_d do '1,2,3' &> /dev/null
out=$(tsk_d undo "1,2,3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

undone_count=$(grep -ce "\s[1-9]\..*[\s].*" <<< "$out")
[[ $undone_count -eq 4 ]] # +1 from the initial task added after a reset
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark list of subtasks"
## {{{
n=$(tsk_d add 'one' ; tsk_d add -sub 1 'one' ; tsk_d add -sub 1 'one' ; tsk_d add -sub 1 'one')
tsk_d do '1.1,2,3' &> /dev/null
out=$(tsk_d undo "1.1,2,3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

undone_count=$(grep -ce "\s[1-9]\..*[\s].*" <<< "$out")
[[ $undone_count -eq 5 ]]
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark list of sub-subtasks"
## {{{
n=$(tsk_d add 'one' ; tsk_d add -sub 1 'one' ; tsk_d add -sub 1.1 'one' ; tsk_d add -sub 1.1 'one' ; tsk_d add -sub 1.1 'one')
tsk_d do "1.1.1,2,3" &> /dev/null
out=$(tsk_d undo "1.1.1,2,3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

undone_count=$(grep -ce "\s[1-9]\..*[\s].*" <<< "$out")
[[ $undone_count -eq 6 ]]
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark range"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'one' ; tsk_d add 'one')
tsk_d do "1..3" &> /dev/null
out=$(tsk_d undo "1..3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

undone_count=$(grep -ce "\s[1-9]\..*[\s].*" <<< "$out")
[[ $undone_count -eq 4 ]]
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark range of subtasks"
## {{{
n=$(tsk_d add 'one' ; tsk_d add -sub 1 'one' ; tsk_d add -sub 1 'one' ; tsk_d add -sub 1 'one')
tsk_d do "1.1..3"
out=$(tsk_d undo "1.1..3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

undone_count=$(grep -ce "\s[1-9]\..*[\s].*" <<< "$out")
[[ $undone_count -eq 5 ]]
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark range of sub-subtasks"
## {{{
n=$(tsk_d add 'one' ; tsk_d add -sub 1 'one' ; tsk_d add -sub 1.1 'one' ; tsk_d add -sub 1.1 'one' ; tsk_d add -sub 1.1 'one')
tsk_d do "1.1.1..3"
out=$(tsk_d undo "1.1.1..3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

undone_count=$(grep -ce "\s[1-9]\..*[\s].*" <<< "$out")
[[ $undone_count -eq 6 ]]
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark all"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'two' ; tsk_d add 'three' ; tsk_d add -sub 1 'a' ; tsk_d add -sub 1.1 'b')
tsk_d do -all
out=$(tsk_d undo -all)
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

undone_count=$(grep -ce "\s[1-9]\..*[\s].*" <<< "$out")
[[ $undone_count -eq 6 ]]
echo "${GRE}OK${NC}"
## }}}

echo "Test: Fail when passing wrong id or option"
## {{{
set +e
out=$(tsk_d undo 50 2>&1) ; ex=$?
out2=$(tsk_d undo 1.50 2>&1) ; ex2=$?
out3=$(tsk_d undo 1.1.50 2>&1) ; ex3=$?
out4=$(tsk_d undo -not 2>&1) ; ex4=$?
set -e
if [[ $SHOW_OUT ]]; then
    echo -e "$out \n $out2 \n $out3 \n $out4"
fi

[[ $(($ex + $ex2 + $ex3 + $ex4)) -eq 4 ]]
echo "${GRE}OK${NC}"
## }}}
