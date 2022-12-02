#! /bin/bash
# adapted from https://github.com/wmoussa-cds/aoc2022/blob/main/data/get_input.sh

day_file=$(date +%d)
day_url=$(echo ${day_file} | sed 's/^0*//')

curl -b $(cat aoc_cookie) "https://adventofcode.com/2022/day/${day_url}/input" > "data/day"${day_file}".txt"