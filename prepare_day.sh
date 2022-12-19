#! /bin/bash
# inspired by https://github.com/wmoussa-cds/aoc2022/blob/main/data/get_input.sh

# Usage: ./prepare_day.sh 07

if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    exit 1
fi

day=$1
day_url=$(echo ${day} | sed 's/^0*//')

curl -b $(cat aoc_cookie) "https://adventofcode.com/2022/day/${day_url}/input" > "data/day"${day}".txt"

cat templates/main.rs | sed s/day01/day${day}/g > src/main.rs
cat templates/test.rs | sed s/day01/day${day}/g > tests/day${day}.rs
cp templates/day.rs src/day${day}.rs
echo "pub mod day${day};" >> src/lib.rs
cargo fmt
