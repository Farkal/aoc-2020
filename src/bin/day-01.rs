#![feature(bool_to_option)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::Read};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-01.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> HashSet<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part_1(data: &HashSet<u32>) -> u32 {
    data.iter()
        .find_map(|x| data.contains(&(2020 - x)).then_some(x * (2020 - *x)))
        .unwrap()
}

fn part_2(data: &HashSet<u32>) -> u32 {
    data.iter()
        .tuple_combinations()
        .find_map(|(x, y)| {
            2020u32
                .checked_sub(x + y)
                .filter(|z| data.contains(&z))
                .map(|z| x * y * z)
        })
        .unwrap()
}

fn main() {
    let input = read_input();
    let parsed = parse_input(&input);

    println!("Part 1: {:#?}", part_1(&parsed));
    println!("Part 2: {:#?}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456]
            .iter()
            .map(|i| *i)
            .collect();
        assert_eq!(part_1(&input), 514579);
    }

    #[test]
    fn test_part_2() {
        let input = [1721, 979, 366, 299, 675, 1456]
            .iter()
            .map(|i| *i)
            .collect();
        assert_eq!(part_2(&input), 241861950);
    }
}
