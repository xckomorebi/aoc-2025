#! /bin/bash

if [ $# -lt 2 ]; then
    echo 'not enough args'
    echo "usage: $0 {DAY} {PART}"
    exit 1
fi

DAY=$1
PART=$2

function ensure_input() {
    day=$1
    if [ -e data/$day.txt ]; then
        return
    fi

    if [ -z $AOC_SESSION ]; then
        echo '$AOC_SESSION enviroment variable must be set!'
        exit 1
    fi

    mkdir -p data

    curl "https://adventofcode.com/2025/day/$DAY/input" -o data/$DAY.txt \
        -b "session=$AOC_SESSION" -s
}

ensure_input $DAY

exe_name=day${DAY}-${PART}


cat data/$DAY.txt | cargo run --bin $exe_name
