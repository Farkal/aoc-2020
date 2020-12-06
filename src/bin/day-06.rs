use bstr::ByteSlice;
use itertools::Itertools;
use rayon::prelude::*;
use std::{fs::File, io::Read};

type Parsed = Vec<Vec<u32>>;
type Out = u32;

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-06.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Parsed {
    input
        .split_str("\n\n")
        .map(|i| {
            i.lines()
                .map(|l| l.iter().map(|i| 1u32 << (i - b'a')).sum())
                .collect_vec()
        })
        .collect_vec()
}

fn part_1(data: &Parsed) -> Out {
    data.par_iter()
        .map(|passenger_group| {
            passenger_group
                .iter()
                .copied()
                .fold1(|acc, passenger| acc | passenger)
                .map(|i| i.count_ones())
                .unwrap()
        })
        .sum()
}

fn part_2(data: &Parsed) -> Out {
    data.par_iter()
        .map(|passenger_group| {
            passenger_group
                .iter()
                .copied()
                .fold1(|acc, passenger| acc & passenger)
                .map(|i| i.count_ones())
                .unwrap()
        })
        .sum()
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

    const TEST_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 11);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 6);
    }
}
