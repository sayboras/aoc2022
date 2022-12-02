use crate::file_reader;

pub fn run() {
    let input = read_input("src/day_01_input.txt");
    part1(&input);
    part2(&input);
}

fn read_input(file: &str) -> Vec<Vec<i32>> {
    file_reader::read_file_in_cwd(file)
        .split("\n\n")
        .map(|x| x.trim())
        .map(|val| val.split("\n").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect()
}

fn part1(input: &Vec<Vec<i32>>) {
    let res = input.iter().map(|x| x.iter().sum::<i32>()).max().unwrap();
    println!("Part 1 result: {}", res);
}

fn part2(input: &Vec<Vec<i32>>) {
    let mut total: Vec<i32> = input.iter().map(|x| x.iter().sum()).collect();
    total.sort_by(|a, b| b.cmp(a));
    println!("Part 2 result: {}", total[..3].iter().sum::<i32>());
}
