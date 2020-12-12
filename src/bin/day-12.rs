#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{fs::File, io::Read, mem};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-12.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

#[derive(Clone, Debug)]
enum Move {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn parse_input(input: &[u8]) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let k = l[0];
            let v = l[1..].parse().unwrap();

            match k {
                b'N' => Move::North(v),
                b'S' => Move::South(v),
                b'E' => Move::East(v),
                b'W' => Move::West(v),
                b'R' => Move::Right(v),
                b'L' => Move::Left(v),
                b'F' => Move::Forward(v),
                _ => unreachable!(),
            }
        })
        .collect_vec()
}

fn part_1(data: &[Move]) -> i32 {
    let (ew_pos, ns_pos, _heading) =
        data.iter()
            .fold((0, 0, 90), |(mut ew_pos, mut ns_pos, mut heading), m| {
                match m {
                    Move::North(n) => ns_pos += n,
                    Move::South(n) => ns_pos -= n,
                    Move::East(n) => ew_pos += n,
                    Move::West(n) => ew_pos -= n,
                    Move::Left(n) => heading = (heading - n).rem_euclid(360),
                    Move::Right(n) => heading = (heading + n).rem_euclid(360),
                    Move::Forward(n) => match heading {
                        0 => ns_pos += n,
                        90 => ew_pos += n,
                        180 => ns_pos -= n,
                        270 => ew_pos -= n,
                        _ => unreachable!(),
                    },
                };

                (ew_pos, ns_pos, heading)
            });

    ew_pos.abs() + ns_pos.abs()
}

fn part_2(data: &[Move]) -> i32 {
    let ((ew_pos, ns_pos), _wp) = data.iter().fold(
        ((0, 0), (10, 1)),
        |((mut ew_pos, mut ns_pos), (mut ew_wp, mut ns_wp)), m| {
            match m {
                Move::North(n) => ns_wp += n,
                Move::South(n) => ns_wp -= n,
                Move::East(n) => ew_wp += n,
                Move::West(n) => ew_wp -= n,
                Move::Right(_) | Move::Left(_) => {
                    let n = match m {
                        Move::Right(n) => *n,
                        Move::Left(n) => 360 - n,
                        _ => unreachable!(),
                    }
                    .rem_euclid(360);
                    match n {
                        0 => {}
                        90 => {
                            mem::swap(&mut ew_wp, &mut ns_wp);
                            ns_wp = -ns_wp;
                        }
                        180 => {
                            ew_wp = -ew_wp;
                            ns_wp = -ns_wp;
                        }
                        270 => {
                            mem::swap(&mut ew_wp, &mut ns_wp);
                            ew_wp = -ew_wp;
                        }
                        _ => unreachable!(),
                    }
                }
                Move::Forward(n) => {
                    ew_pos += n * ew_wp;
                    ns_pos += n * ns_wp;
                }
            };

            ((ew_pos, ns_pos), (ew_wp, ns_wp))
        },
    );

    ew_pos.abs() + ns_pos.abs()
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

    const TEST_INPUT: &str = "F10
N3
F7
R90
F11
";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 25);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 286);
    }
}
