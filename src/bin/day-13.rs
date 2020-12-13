#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{fs::File, io::Read};

enum Departure {
    Bus(i64),
    X,
}

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-13.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> (i64, Vec<Departure>) {
    input
        .lines()
        .tuples()
        .next()
        .map(|(ts, deps)| {
            (
                ts.parse().unwrap(),
                deps.split_str(",")
                    .map(|n| n.parse().map(Departure::Bus).unwrap_or(Departure::X))
                    .collect_vec(),
            )
        })
        .unwrap()
}

fn part_1(timestamp: i64, departures: &[Departure]) -> i64 {
    departures
        .iter()
        .filter_map(|i| match i {
            Departure::Bus(n) => Some(*n),
            _ => None,
        })
        .min_by_key(|key| *key - timestamp % *key)
        .map(|min_id| min_id * (min_id - timestamp % min_id))
        .unwrap()
}

// Chinese remainder implementation from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
// End chinese remainder

fn part_2(data: &[Departure]) -> i64 {
    let it = data.iter().enumerate().filter_map(|(n, d)| match d {
        Departure::Bus(x) => Some((n as i64, *x)),
        _ => None,
    });

    let residues = it.clone().map(|(n, id)| id - n).collect_vec();
    let modulii = it.map(|(_n, id)| id).collect_vec();

    chinese_remainder(&residues[..], &modulii[..]).unwrap()
}

fn main() {
    let input = read_input();
    let (ts, deps) = parse_input(&input);

    println!("Part 1: {}", part_1(ts, &deps));
    println!("Part 2: {}", part_2(&deps));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = concat!("939\n", "7,13,x,x,59,x,31,19\n");

    #[test]
    fn test_part_1() {
        let (ts, deps) = parse_input(TEST_INPUT_P1.as_bytes());
        assert_eq!(part_1(ts, &deps), 295);
    }

    const TEST_INPUTS_P2: &[(&str, i64)] = &[
        ("17,x,13,19", 3417),
        ("67,7,59,61", 754018),
        ("67,x,7,59,61", 779210),
        ("67,7,x,59,61", 1261476),
        ("1789,37,47,1889", 1202161486),
    ];

    #[test]
    fn test_part_2() {
        for (inp, expected) in TEST_INPUTS_P2 {
            let inp = format!("0\n{}\n", inp);

            let (_, deps) = parse_input(inp.as_bytes());
            assert_eq!(part_2(&deps), *expected);
        }
    }
}
