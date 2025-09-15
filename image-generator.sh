#!/bin/bash

function generate_task_images() {
    task_idx=$1

    if [[ ! -d images/task$task_idx/ ]] ; then
        mkdir -p images/task$task_idx/
    fi

    rm -f images/task$task_idx/*-out.png
    num_task_tests=$(ls "tests/input/task$task_idx/" | wc -l)

    # Loop over each test index
    local idx
    for (( idx=0; idx<num_task_tests; idx++ )); do
        # Don't generate the in/ref images if they already exist
        if [[ ! -f images/task$task_idx/$idx-in.png ]] ; then
            ./view3d.sh tests/input/task$task_idx/$idx.in -o images/task$task_idx/$idx-in.png
        fi
        if [[ ! -f images/task$task_idx/$idx-ref.png ]] ; then
            ./view3d.sh tests/ref_output/task$task_idx/$idx.ref -o images/task$task_idx/$idx-ref.png
        fi

        ./view3d.sh tests-out/task$task_idx/$idx.out -o images/task$task_idx/$idx-out.png
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
    for (( idx=1; idx<=num_tasks; idx++ )); do
        generate_task_images $idx
    done
}

if [[ $# -eq 0 ]]; then
    echo "Error: No arguments provided." >&2
    exit 255
fi

case "$1" in
    -a|--all)
        generate_all_images
        ;;
    -t|--task)
        if [[ -n "$2" && "$2" =~ ^[0-9]+$ ]]; then
            generate_task_images "$2"
        else
            echo "Error: '-t'/'--task' requires a numeric argument." >&2
            exit 255
        fi
        ;;
    *)
        echo "Error: Invalid arguments." >&2
        exit 255
        ;;
esac


