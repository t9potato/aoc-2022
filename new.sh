#!/usr/bin/bash
cargo new day$1 --vcs none
rm day$1/src/main.rs
echo 'fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}' >> day$1/src/main.rs
touch day$1/src/input
cd day$1
zellij --layout aoc
