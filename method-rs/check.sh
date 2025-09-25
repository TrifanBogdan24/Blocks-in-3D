#!/bin/bash

RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
RESET="\033[0m"


tasks=($(ls -1 ../tests/input | sed -E 's/task([0-9]+).*/\1/' | sort -n))



for task in "${tasks[@]}"
do
    echo "................................TASK $task............................"

    tests=($(ls -1 ../tests/input/task$task | sed -E 's/\.in$//' | sort -n))

    for test in "${tests[@]}"
    do
        echo -n "Test $test......................................................"

        # Run specific test case with `cargo`:
        cargo test test_task$task::case_$((test+1)) -- --exact > /dev/null 2>&1

        if [[ $? -ne 0 ]]
        then
            echo -e "${RED}FAILED${RESET}"
        else
            echo -e "${GREEN}passed${RESET}"
        fi
    done
done


echo "..............................CODING STYLE............................."

cargo fmt -- --check 2>&1 | tee coding_style.txt
echo -n "Test Coding Style..........................................."


if [[ $(cat coding_style.txt | wc -l ) -gt 0 ]]
then
    echo -n "-20p "
    echo -e "${RED}FAILED${RESET}"
    echo -e "\t[Suggestion] run: ${YELLOW}cargo fmt${RESET}"
else
    echo -e "${GREEN}passed${RESET}"
fi
