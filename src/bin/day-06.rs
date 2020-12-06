use bstr::ByteSlice;
use itertools::Itertools;
use std::io::Read;
use std::{collections::HashSet, fs::File};

type Parsed = Vec<Vec<Vec<u8>>>;
type Out = usize;

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
                .map(|l| l.iter().copied().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

fn part_1(data: &Parsed) -> Out {
    data
        .iter()
        .map(|passenger_group| {
            passenger_group.iter()
                .map(|passenger| passenger.iter().collect::<HashSet<_>>())
                .fold1(|acc, passenger| acc.union(&passenger).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}

fn part_2(data: &Parsed) -> Out {
    data
        .iter()
        .map(|passenger_group| {
            passenger_group.iter()
                .map(|passenger| passenger.iter().collect::<HashSet<_>>())
                .fold1(|acc, passenger| acc.intersection(&passenger).copied().collect())
                .unwrap()
                .len()
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
