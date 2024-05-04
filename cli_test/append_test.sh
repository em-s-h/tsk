set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'append' command:\n"
reset_file

echo "Test: Undo done task when appending"
## {{{
out=$(tsk_d append 1 "append")
if [[ $SHOW_OUT ]]; then
    echo -e "$out"
fi

grep -qe "\s1\..*[\s].*append" <<< "$out"
echo "${GRE}OK${NC}"
## }}}
