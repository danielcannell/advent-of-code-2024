#!/usr/bin/env bash

set -o errexit -o nounset

if [ ! -f "AOC_SESSION" ]
then
    echo "ERROR: The AOC session coookie should be stored in ./AOC_SESSION"
    exit 1
fi

if [ ! $# -eq 1 ]
then
    echo "ERROR: Please provide a day number."
    echo "Usage: $0 DAY"
    exit 1
fi

mkdir -p input

curl "https://adventofcode.com/2024/day/$1/input" --cookie "session=$(<AOC_SESSION)" -s | tee "input/day$1"
