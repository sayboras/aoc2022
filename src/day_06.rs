use crate::file_reader;
use std::collections::HashSet;

pub fn run() {
    let content = file_reader::read_file_in_cwd("src/input/day_06.txt");
    part1(&content);
    part2(&content);
}

fn part1(input: &str) {
    println!("Part 1 result: {}", solve(input, 4));
}

fn part2(input: &str) {
    println!("Part 2 result: {}", solve(input, 14));
}

fn solve(input: &str, num_of_distinct_values: usize) -> usize {
    let mut res = 0;
    let mut last = Vec::new();

    for (i, c) in input.chars().enumerate() {
        last.push(c);
        if last.len() > num_of_distinct_values {
            last.remove(0);
        }

        if last.iter().collect::<HashSet<_>>().len() == num_of_distinct_values {
            res = i + 1;
            break
        }
    }
    res
}