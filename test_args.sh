#!/bin/bash 

cd ~/dev/clist/
show_out=$1

echo "Create list properly"
# {{{
output=$(cargo r -- create "list1")
if [[ $output = *"created"* && -f ./lists/list1 ]]; then
    echo "Success"
else
    echo "Fail"
fi
[[ $show_out = *"show"* ]] && echo -e "$output"
# }}}

echo
echo "Create list w/o passing list name"
# {{{
output=$(cargo r -- create)
if [[ $output = *"provide the name"* ]]; then
    echo "Success"
else
    echo "Fail"
fi
[[ $show_out = *"show"* ]] && echo -e "$output"
# }}}

echo
echo "Remove list properly"
# {{{
output=$(cargo r -- remove "list1" y)
if [[ $output = *"Removed"* && ! -f ./lists/list1 ]]; then
    echo "Success"
else
    echo "Fail"
fi
[[ $show_out = *"show"* ]] && echo -e "$output"
# }}}

echo
echo "Remove nonexisting list"
# {{{
output=$(cargo r -- remove "list2" y)
if [[ $output = *"list doesn't exist"* ]]; then
    echo "Success"
else
    echo "Fail"
fi
[[ $show_out = *"show"* ]] && echo -e "$output"
# }}}

echo
echo "Remove list w/o passing list name"
# {{{
output=$(cargo r -- remove )
if [[ $output = *"provide the name"* ]]; then
    echo "Success"
else
    echo "Fail"
fi
[[ $show_out = *"show"* ]] && echo -e "$output"
# }}}

echo
echo "Add to list properly"
# {{{
output=$(cargo r -- remove )
if [[ $output = *"provide the name"* ]]; then
    echo "Success"
else
    echo "Fail"
fi
[[ $show_out = *"show"* ]] && echo -e "$output"
# }}}
