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

grep -qe "\s1\..*[\s]" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark list"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'one' ; tsk_d add 'one')
n=$(tsk_d do "1..3")

out=$(tsk_d undo "1,2,3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

lines=$(tail -n +4 <<< "$out" | head -n 3)
IFS=$'\n' ; for l in $lines; do
    grep -qe "\s[1-9]\..*[\s]*" <<< "$l"
done

echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark range"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'one' ; tsk_d add 'one')
n=$(tsk_d do "1..3")

out=$(tsk_d undo "1..3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

lines=$(tail -n +4 <<< "$out" | head -n 3)
IFS=$'\n' ; for l in $lines; do
    grep -qe "\s[1-9]\..*[\s]*" <<< "$l"
done

echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Unmark all"
## {{{
out=$(tsk_d undo -all)
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s[1-9]\..*[\s]*" <<< "$out"
echo "${GRE}OK${NC}"
## }}}
