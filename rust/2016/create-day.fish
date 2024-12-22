#!/bin/fish

set day_idx (math (count day??) + 1)
set day_name day(string pad --char='0' -w 2 $day_idx)

if not test -e $day_name
    cargo new $day_name --name aoc16-$day_name
    cd $day_name
    cargo add anyhow
end

echo https://adventofcode.com/2016/day/$day_idx
echo https://adventofcode.com/2016/day/$day_idx/input
