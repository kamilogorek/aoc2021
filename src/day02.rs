use std::fs;

fn parse_input(input: &str) -> Vec<(&str, usize)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(&str, usize)>>()
}

fn solve_part1(input: &[(&str, usize)]) -> usize {
    let mut pos = 0;
    let mut depth = 0;

    for (command, val) in input {
        match *command {
            "forward" => pos += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => unreachable!(),
        }
    }

    pos * depth
}

fn solve_part2(input: &[(&str, usize)]) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (command, val) in input {
        match *command {
            "forward" => {
                pos += val;
                depth += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => unreachable!(),
        }
    }

    pos * depth
}

pub fn run() {
    let sample_input = &fs::read_to_string("src/day02_sample.txt").unwrap();
    let parsed_sample_input = parse_input(sample_input);
    let input = &fs::read_to_string("src/day02.txt").unwrap();
    let parsed_input = parse_input(input);

    assert_eq!(solve_part1(&parsed_sample_input), 150);
    assert_eq!(solve_part1(&parsed_input), 1561344);

    assert_eq!(solve_part2(&parsed_sample_input), 900);
    assert_eq!(solve_part2(&parsed_input), 1848454425);

    println!("Day 02 ✓");
}
