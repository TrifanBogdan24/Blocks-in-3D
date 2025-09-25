#!/bin/bash

METHOD=""   # "C"/"rs"

MIN_idx_task=1
MAX_idx_task=8

GREEN="\033[1;32m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
RESET="\033[0m"
BLINK="\033[5m"

function generate_task_images() {
    idx_task=$1

    if [[ ! -d images/task$idx_task/ ]] ; then
        mkdir -p images/task$idx_task/
    fi

    rm -f images/task$idx_task/*-out.png
    num_task_tests=$(ls "tests/input/task$idx_task/" | wc -l)

    # Loop over each test index
    for (( idx=0; idx<num_task_tests; idx++ )); do
        # Using \r (carriage return) moves the cursor to the start of the line, so we can overwrite it
        echo -ne "Generating images for task$idx_task-test_$idx: ${YELLOW}pending${RESET}\r"

        # Don't generate the in/ref images if they already exist
        if [[ ! -f images/task$idx_task/$idx-in.png ]] ; then
            ./view3d.sh tests/input/task$idx_task/$idx.in -o images/task$idx_task/$idx-in.png
        fi
        if [[ ! -f images/task$idx_task/$idx-ref.png ]] ; then
            ./view3d.sh tests/ref_output/task$idx_task/$idx.ref -o images/task$idx_task/$idx-ref.png
        fi

        ./view3d.sh tests-out/$METHOD/task$idx_task/$idx.out -o images/task$idx_task/$idx-out-$METHOD.png

        # Overwrite with "DONE" message
        echo -e "Generating images for task$idx_task-test_$idx: ${BLUE}DONE   ${RESET}"
    done
}


function generate_all_images() {
    rm -rf images/*
    if [[ ! -d images ]] ; then
        mkdir images/
    fi

    num_tasks=$(ls "tests/input/" | wc -l)

    # Loop over each task index
    local idx
    for (( idx=$MIN_idx_task; idx<=$MAX_idx_task; idx++ )); do
        generate_task_images $idx
    done
}


if [[ $# -eq 0 ]]; then
    echo "[ERROR] No arguments provided." >&2
    exit 255
fi


if [[ $# -ne 2 && $# -ne 3 ]] ; then
    echo "[ERROR] Script was called with invalid number of arguments!" >&2
    echo "[INFO]  Template to call script"
    echo "        $0 <-c/-rs> <-a/--all>"
    echo "        $0 <-c/-rs> <-t/-task> 1"
fi


case "$1" in
    -rs|--rust)
        METHOD="method-rs"
        ;;
    -c)
        METHOD="method-C"
        ;;
    *)
        echo "[ERROR] Invalid method name" >&2
        exit 255
        ;;
esac




case "$2" in
    -a|--all)
        generate_all_images
        ;;
    -t|--task)
        if [[ -n "$3" && "$3" =~ ^[0-9]+$ ]] ; then
            if [[ "$3" -lt $MIN_idx_task || "$3" -gt $MAX_idx_task ]] ; then
                echo "[ERROR] Invalid task index $3 to generate images for." >&2
                exit 255
            fi
            generate_task_images "$3"
        else
            echo "[ERROR] '-t'/'--task' requires a numeric argument." >&2
            exit 255
        fi
        ;;
    *)
        echo "[ERROR] Invalid arguments." >&2
        exit 255
        ;;
esac
