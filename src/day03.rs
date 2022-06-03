use itertools::Itertools;
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

fn solve_part1(input: &[Vec<u32>]) -> usize {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    let mz = Multizip(input.iter().map(|x| x.iter()).collect());

    for v in mz {
        let counts = v.into_iter().counts();
        if counts.get(&0) > counts.get(&1) {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}

fn recur_part2(input: &[Vec<u32>], pos: usize, oxygen: bool) -> usize {
    if input.len() == 1 {
        return usize::from_str_radix(
            &input
                .first()
                .unwrap()
                .iter()
                .map(|x| x.to_string())
                .join(""),
            2,
        )
        .unwrap();
    }

    let mut col = vec![];
    for row in input {
        col.push(row[pos]);
    }
    let counts = col.into_iter().counts();
    let filter = if counts.get(&0) > counts.get(&1) {
        if oxygen {
            1
        } else {
            0
        }
    } else if oxygen {
        0
    } else {
        1
    };

    let rem = input
        .iter()
        .filter(|v| *v.get(pos).unwrap() == filter)
        .map(|x| x.to_owned())
        .collect::<Vec<Vec<u32>>>();

    recur_part2(&rem, pos + 1, oxygen)
}

fn solve_part2(input: &[Vec<u32>]) -> usize {
    recur_part2(input, 0, true) * recur_part2(input, 0, false)
}

pub fn run() {
    let sample_input = &fs::read_to_string("src/day03_sample.txt").unwrap();
    let parsed_sample_input = parse_input(sample_input);
    let input = &fs::read_to_string("src/day03.txt").unwrap();
    let parsed_input = parse_input(input);

    assert_eq!(solve_part1(&parsed_sample_input), 198);
    assert_eq!(solve_part1(&parsed_input), 3242606);

    assert_eq!(solve_part2(&parsed_sample_input), 230);
    assert_eq!(solve_part2(&parsed_input), 4856080);

    println!("Day 03 ✓");
}
