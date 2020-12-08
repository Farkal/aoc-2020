use bstr::ByteSlice;
use bstr_parse::*;
use memchr::memchr_iter;
use regex::bytes::Regex;
use std::{fs::File, io::Read};

struct Password {
    pub min: usize,
    pub max: usize,
    pub letter: u8,
    pub password: Vec<u8>,
}

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-02.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Vec<Password> {
    let re = Regex::new(
        r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>[[:alpha:]]):\s(?P<password>[[:alpha:]]+)",
    )
    .unwrap();
    input
        .lines()
        .map(|x| re.captures(x).unwrap())
        .map(|x| Password {
            min: x["min"].parse().unwrap(),
            max: x["max"].parse().unwrap(),
            letter: x["letter"][0],
            password: x["password"].into(),
        })
        .collect()
}

fn part_1(data: &[Password]) -> usize {
    data.iter()
        .filter(|pw| {
            let count = memchr_iter(pw.letter, &pw.password).count();

            (pw.min..=pw.max).contains(&count)
        })
        .count()
}

fn part_2(data: &[Password]) -> usize {
    data.iter()
        .filter(|pw| {
            let a = pw.password.get(pw.min);
            let b = pw.password.get(pw.max);

            (a == Some(&pw.letter)) ^ (b == Some(&pw.letter))
        })
        .count()
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
    fn test_part_1() {
        let input = concat!("1-3 a: abcde\n", "1-3 b: cdefg\n", "2-9 c: ccccccccc");
        let parsed = parse_input(input.as_bytes());
        assert_eq!(part_1(&parsed), 2);
    }

    #[test]
    fn test_part_2() {
        let input = concat!("1-3 a: abcde\n", "1-3 b: cdefg\n", "2-9 c: ccccccccc");
        let parsed = parse_input(input.as_bytes());
        assert_eq!(part_2(&parsed), 1);
    }
}
