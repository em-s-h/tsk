set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'do' command:\n"
reset_file

echo "Test: Mark one"
## {{{
tsk_d add "test" &> /dev/null
out=$(tsk_d do 1)

if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s1\..*[X]*" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Mark list"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'one' ; tsk_d add 'one')
out=$(tsk_d do "1,2,3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

lines=$(tail -n +4 <<< "$out" | head -n 3)
IFS=$'\n' ; for l in $lines; do
    grep -qe "\s[1-9]\..*[X]*" <<< "$l"
done

echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Mark range"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'one' ; tsk_d add 'one')
out=$(tsk_d do "1..3")
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

lines=$(tail -n +4 <<< "$out" | head -n 3)
IFS=$'\n' ; for l in $lines; do
    grep -qe "\s[1-9]\..*[X]*" <<< "$l"
done

echo "${GRE}OK${NC}"
## }}}

reset_file

echo "Test: Mark all"
## {{{
n=$(tsk_d add 'one' ; tsk_d add 'one' ; tsk_d add 'one')
out=$(tsk_d do -all)
if [[ $SHOW_OUT ]]; then
    echo "$out"
fi

grep -qe "\s[1-9]\..*[X]*" <<< "$out"
echo "${GRE}OK${NC}"
## }}}

echo "Test: Fail when passing wrong id or option"
## {{{
set +e
out=$(tsk_d do 50 2>&1) ; ex=$?
out2=$(tsk_d do -not 2>&1) ; ex2=$?
set -e
if [[ $SHOW_OUT ]]; then
    echo -e "$out \n $out2"
fi

[[ $(($ex + $ex2)) -eq 2 ]]
echo "${GRE}OK${NC}"
## }}}

