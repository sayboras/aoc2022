use crate::file_reader;

pub fn run() {
    let content = file_reader::read_file_in_cwd("src/day_03_input.txt");
    let input = content.lines().collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<&str>) {
    let res: i32 = input
        .iter()
        .map(|item| {
            let mut res = 0;
            let (part1, part2) = item.split_at(item.len() / 2);
            for c in part1.chars() {
                if part2.contains(c) {
                    res = priority(c);
                    break;
                }
            }
            res
        })
        .sum();
    println!("Part 1 result: {}", res);
}

fn part2(input: &Vec<&str>) {
    let res: i32 = input
        .chunks(3)
        .map(|item| {
            let mut res = 0;
            for c in item[0].chars() {
                if item[1].contains(c) && item[2].contains(c) {
                    res = priority(c);
                    break
                }
            }
            res
        })
        .sum();
    println!("Part 2 result: {}", res);
}

fn priority(c: char) -> i32 {
    let ascii_code = c as i32;
    if ascii_code >= 65 && ascii_code <= 90 {
        return ascii_code - 65 + 27;
    }
    return ascii_code - 97 + 1;
}