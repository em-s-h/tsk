set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'move' command:\n"
reset_file

echo "Test: Move first to last"
## {{{
n=$(tsk_d add 'two'; tsk_d add 'one'; tsk_d add 'three')
out=$(tsk_d move 1 3)
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\..*one" <<< "$out"
grep -qe "\s2\..*two" <<< "$out"
grep -qe "\s3\..*three" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Move last to first"
## {{{
n=$(tsk_d add 'one'; tsk_d add 'three'; tsk_d add 'two')
out=$(tsk_d move 3 1)
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\..*one" <<< "$out"
grep -qe "\s2\..*two" <<< "$out"
grep -qe "\s3\..*three" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

echo "Test: Fail when passing equal ids"
## {{{
set +e
out=$(tsk_d move 5 5 2>&1) ; ex=$?
set -e
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

[[ $ex -eq 1 ]]
echo "${GRE}OK${NC}"
## }}}


