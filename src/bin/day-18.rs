use itertools::Itertools;
use lalrpop_util::lalrpop_mod;
use std::{fs::File, io::Read};

lalrpop_mod!(pub day_18_p1);
lalrpop_mod!(pub day_18_p2);

fn read_input() -> String {
    let mut f = File::open("input/day-18.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect_vec()
}

fn part_1(data: &[&str]) -> u64 {
    let parser = day_18_p1::ExprParser::new();

    data.iter().map(|term| parser.parse(term).unwrap()).sum()
}

fn part_2(data: &[&str]) -> u64 {
    let parser = day_18_p2::ExprParser::new();

    data.iter().map(|term| parser.parse(term).unwrap()).sum()
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

    const TEST_INPUTS_P1: &[(&str, u64)] = &[
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ];

    const TEST_INPUTS_P2: &[(&str, u64)] = &[
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    ];

    #[test]
    fn test_part_1() {
        for (input, expected) in TEST_INPUTS_P1 {
            let parsed = parse_input(input);
            assert_eq!(part_1(&parsed), *expected);
        }
    }

    #[test]
    fn test_part_2() {
        for (input, expected) in TEST_INPUTS_P2 {
            let parsed = parse_input(input);
            assert_eq!(part_2(&parsed), *expected);
        }
    }
}
