use itertools::Itertools;
use std::fs;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn solve_part1(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows()
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
}

fn solve_part2(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
}

pub fn run() {
    let sample_input = parse_input(&fs::read_to_string("src/day01_sample.txt").unwrap());
    let input = parse_input(&fs::read_to_string("src/day01.txt").unwrap());

    assert_eq!(solve_part1(&sample_input), 7);
    assert_eq!(solve_part1(&input), 1167);

    assert_eq!(solve_part2(&sample_input), 5);
    assert_eq!(solve_part2(&input), 1130);

    println!("Day 01 ✓");
}
