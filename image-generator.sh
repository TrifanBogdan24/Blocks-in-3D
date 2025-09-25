#!/bin/bash

METHOD=""   # "C"/"rs"

MIN_TASK_IDX=1
MAX_TASK_IDX=8

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

        ./view3d.sh tests-out/$METHOD/task$task_idx/$idx.out -o images/task$task_idx/$idx-out-$METHOD.png
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
    for (( idx=$MIN_TASK_IDX; idx<=$MAX_TASK_IDX; idx++ )); do
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
            if [[ "$3" -lt $MIN_TASK_IDX || "$3" -gt $MAX_TASK_IDX ]] ; then
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
