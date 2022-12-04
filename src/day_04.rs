use crate::file_reader;

pub fn run() {
    let input = read_input("src/input/day_04.txt");
    part1(&input);
    part2(&input);
}

fn read_input(file: &str) -> Vec<Vec<i32>> {
    file_reader::read_file_in_cwd(file)
        .split("\n")
        .map(|line| {
            let (part1, part2) = line.split_once(",").unwrap();
            let (part11, part12) = part1.split_once("-").unwrap();
            let (part21, part22) = part2.split_once("-").unwrap();
            let v = vec![
                part11.parse::<i32>().unwrap(),
                part12.parse::<i32>().unwrap(),
                part21.parse::<i32>().unwrap(),
                part22.parse::<i32>().unwrap(),
            ];
            v
        })
        .collect()
}

fn part1(input: &Vec<Vec<i32>>) {
    let res: i32 = input
        .iter()
        .map(|x| if (x[0] >= x[2] && x[1] <= x[3]) || (x[0] <= x[2] && x[1] >= x[3]) { 1 } else { 0 })
        .sum();
    println!("Part 1 result: {}", res);
}

fn part2(input: &Vec<Vec<i32>>) {
    let res: i32 = input
        .iter()
        .map(|x| if (x[0] >= x[2] && x[0] <= x[3]) || (x[2] >= x[0] && x[2] <= x[1]) { 1 } else { 0 })
        .sum();
    println!("Part 2 result: {}", res);
}