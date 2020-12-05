use bstr::ByteSlice;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-05.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(data: &[u8]) -> Vec<u16> {
    data
        .lines()
        .map(|l| {
            l.iter()
                .fold(0, |acc, c| acc << 1 | (c == &b'B' || c == &b'R') as u16)
        })
        .collect()
}

fn part_1(input: &[u16]) -> u16 {
    input.iter().max().copied().unwrap_or(0)
}

fn part_2(data: &[u16]) -> u16 {
    data
        .iter()
        .sorted()
        .tuple_windows()
        .find_map(|(a, b)| if a + 1 != *b { Some(a + 1) } else { None })
        .unwrap()
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

    #[test]
    fn test_parse() {
        let tests = [
            (b"FBFBBFFRLR", 357),
            (b"BFFFBBFRRR", 567),
            (b"FFFBBBFRRR", 119),
            (b"BBFFBBFRLL", 820),
        ];
        tests
            .iter()
            .for_each(|(s, id)| assert_eq!(parse_input(*s)[0], *id))
    }
}
