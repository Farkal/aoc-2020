#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::Read};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-09.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Vec<u64> {
    input.lines().map(|i| i.parse().unwrap()).collect_vec()
}

fn part_1(data: &[u64], preamble: usize) -> (u64, usize) {
    for i in 0..=data.len() - preamble {
        let current = data[i + preamble];
        let s = data[i..i + preamble].iter().collect::<HashSet<_>>();

        if !data[i..i + preamble]
            .iter()
            .filter(|j| *j * 2 != current)
            .filter_map(|j| current.checked_sub(*j))
            .any(|j| s.contains(&j))
        {
            return (current, i + preamble);
        }
    }
    unreachable!()
}

fn part_2(data: &[u64], tosum: u64) -> u64 {
    let mut running_sum = 0;
    let mut j = 0;
    for i in 0..(data.len() - 1) {
        while running_sum < tosum {
            running_sum += data[j];
            j += 1;
        }

        if running_sum == tosum {
            let (min, max) = data[i..j - 1].iter().minmax().into_option().unwrap();
            return min + max;
        }

        // remove first item
        running_sum -= data[i];

        // revert last additions if necessary
        while running_sum > tosum {
            j -= 1;
            running_sum -= data[j];
        }
    }

    unreachable!()
}
fn main() {
    let input = read_input();
    let parsed = parse_input(&input);

    let (p1_r, p1_index) = part_1(&parsed, 25);

    println!("Part 1: {}", p1_r);
    println!("Part 2: {}", part_2(&parsed[..p1_index], p1_r));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
57";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed, 5).0, 127);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed, 127), 62);
    }
}
