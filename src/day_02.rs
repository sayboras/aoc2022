use crate::file_reader;

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/input/day_02.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let res: u32 = input
        .lines()
        .into_iter()
        .fold(0u32, |score, line| match line {
            "A X" => score + 4,
            "A Y" => score + 8,
            "A Z" => score + 3,
            "B X" => score + 1,
            "B Y" => score + 5,
            "B Z" => score + 9,
            "C X" => score + 7,
            "C Y" => score + 2,
            "C Z" => score + 6,
            _ => unreachable!(),
        })
        .into();
    println!("Part 1 result: {}", res);
}

fn part2(input: &str) {
    let res: u32 = input
        .lines()
        .into_iter()
        .fold(0u32, |score, line| match line {
            "A X" => score + 3,
            "B X" => score + 1,
            "C X" => score + 2,
            "A Y" => score + 4,
            "B Y" => score + 5,
            "C Y" => score + 6,
            "A Z" => score + 8,
            "B Z" => score + 9,
            "C Z" => score + 7,
            _ => unreachable!(),
        })
        .into();
    println!("Part 2 result: {}", res);
}
