#! /bin/bash
# adapted from https://github.com/wmoussa-cds/aoc2022/blob/main/data/get_input.sh

# Usage: ./get_input.sh <day>

if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    exit 1
fi

day=$1


day_url=$(echo ${day} | sed 's/^0*//')
curl -b $(cat aoc_cookie) "https://adventofcode.com/2022/day/${day_url}/input" > "data/day"${day}".txt"

cp "src/day-template.rs" "src/day${day}.rs"
cp "tests/day-template.rs" "tests/day${day}.rs"
echo "pub mod day${day};" >> src/lib.rs

