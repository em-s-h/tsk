set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'swap' command:\n"
reset_file

echo "Test: Swap first with last"
## {{{
n=$(tsk_d add 'one'; tsk_d add 'two'; tsk_d add 'three')
out=$(tsk_d swap 1 3)
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\..*one" <<< "$out"
grep -qe "\s2\..*two" <<< "$out"
grep -qe "\s3\..*three" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

