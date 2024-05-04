set -e

echo -e "|-------------------------------------------------|\n"
echo -e "Testing 'edit' command:\n"
reset_file "done"

echo "Test: Undo done task when editing"
## {{{
out=$(tsk_d edit 1 "edited")
if [[ $SHOW_OUT ]]; then
    echo -e "$out"
fi

grep -qe "\s1.*[\s]*edited" <<< "$out"
echo "${GRE}OK${NC}"
## }}}
