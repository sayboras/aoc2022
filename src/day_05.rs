use crate::file_reader;

pub fn run() {
    let content = file_reader::read_file_in_cwd("src/input/day_05.txt");
    let input = content.lines().collect();
    part1(&input);
    part2(&input);
}

fn part1(_input: &Vec<&str>) {

}

fn part2(_input: &Vec<&str>) {

}