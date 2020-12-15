#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{collections::HashMap, fs::File, io::Read};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-15.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Vec<usize> {
    input
        .split_str(",")
        .map(|l| l.parse().unwrap())
        .collect_vec()
}

fn part_1(data: &[usize]) -> usize {
    solution(data, 2020)
}

fn part_2(data: &[usize]) -> usize {
    solution(data, 30000000)
}

fn solution(data: &[usize], turns: usize) -> usize {
    let mut m: HashMap<_, _> = data.iter().enumerate().map(|(a, b)| (*b, a)).collect();

    (data.len() - 1..turns - 1).fold(*data.last().unwrap(), |last, turn| {
        m.insert(last, turn)
            .map(|last_occurred| turn - last_occurred)
            .unwrap_or(0)
    })
}

fn main() {
    let input = read_input();
    let parsed = parse_input(&input);

    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,3,6";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 436);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 175594);
    }
}
