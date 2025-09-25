#!/bin/bash

RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
RESET="\033[0m"


tasks=( 1 2 3 4 5 6 7 8 9 10)
points=(1 2 2 2 2 2 2 2 2  2)
is_passed_task=(1 1 1 1 1 1 1 1 1 1)



num_points=0

for (( i = 0; i < 10; ++i))
do
    idx_task=${tasks[$i]}
    test_points=${points[$i]}


    echo "................................TASK $idx_task................................."

    tests=($(ls -1 ../tests/input/task$idx_task | sed -E 's/\.in$//' | sort -n))

    for idx_test in "${tests[@]}"
    do
        echo -n "Test $idx_test......................................................"

        # Run specific test case with `cargo`:
        cargo test test_task$idx_task::case_$((idx_test+1)) -- --exact > /dev/null 2>&1

        if [[ $? -ne 0 ]]
        then
            echo -e "${RED}FAILED${RESET} 0/${test_points}p"
            is_passed_task[$i]=0
        else
            num_points=$((num_points+test_points))
            echo -e "${GREEN}passed${RESET} ${test_points}/${test_points}p"
        fi
    done
done



echo "................MORE points for solving tasks 7,8,9,10................."
if [[ is_passed_task[6] -eq 1 && is_passed_task[7] -eq 1 && is_passed_task[8] -eq 1 && is_passed_task[9] -eq 1 ]]
then
    num_points=$((num_points+10))
    echo -e "${GREEN}passed${RESET} 10/10p"
else
    echo -e "${RED}FAIL${RESET} 0/10p"
fi


echo "..............................TASK README.............................."
echo -n "Test README..............................................."

if [[ -f README || -f README.md ]]
then
    num_points=$((num_points+10))
    echo -e "${GREEN}passed${RESET} 10/10p"
else
    echo -e "${RED}FAILED${RESET} 0/10p"
    echo -e "\t[Suggestion] write a ${YELLOW}README.md${RESET} to describe your solution"
fi

echo "..............................CODING STYLE............................."
cargo fmt -- --check 2>&1 | tee coding_style.txt
echo -n "Test Coding Style..........................................."


if [[ $(cat coding_style.txt | wc -l ) -gt 0 ]]
then
    num_points=$((num_points-20))
    echo -e "${RED}FAILED${RESET} -20p"
    echo -e "\t[Suggestion] run: ${YELLOW}cargo fmt${RESET}"
else
    echo -e "${GREEN}passed${RESET}"
fi


if [[ $num_points -lt 0 ]]
then
    num_points=0
fi

echo "Total score: $num_points/100p"
