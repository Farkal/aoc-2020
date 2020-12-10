#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{fs::File, io::Read, iter};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-10.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Vec<usize> {
    let mut out = input
        .lines()
        .map(|i| i.parse().unwrap())
        .chain(iter::once(0))
        .collect_vec();
    out.sort_unstable();
    out
}

fn part_1(data: &[usize]) -> usize {
    let (diff_1, diff_3) =
        data.iter()
            .zip(data.iter().skip(1))
            .fold((0, 1), |(diff_1, diff_3), (a, b)| match b - a {
                3 => (diff_1, diff_3 + 1),
                1 => (diff_1 + 1, diff_3),
                _ => (diff_1, diff_3),
            });

    diff_1 * diff_3
}

fn part_2(data: &[usize]) -> usize {
    data.iter()
        .zip(data.iter().skip(1))
        .map(|(a, b)| b - a)
        .fold((1, 0, 0), |(diff_1, diff_2, diff_3), diff| match diff {
            1 => (diff_1 + diff_2 + diff_3, diff_1, diff_2),
            2 => (diff_1 + diff_2, 0, diff_1),
            3 => (diff_1, 0, 0),
            _ => unreachable!(),
        })
        .0
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

    const TEST_INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 220);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 19208);
    }
}
