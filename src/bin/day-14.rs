#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{collections::HashMap, fs::File, io::Read};

enum Bit {
    Zero,
    One,
    Floating,
}

enum Instruction {
    Set((u64, u64)),
    Mask(Vec<Bit>),
}

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-14.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (key, value) = l.split_str(" = ").collect_tuple().unwrap();
            match key {
                b"mask" => Instruction::Mask(
                    value
                        .iter()
                        .rev()
                        .map(|c| match c {
                            b'0' => Bit::Zero,
                            b'1' => Bit::One,
                            b'X' => Bit::Floating,
                            _ => unreachable!(),
                        })
                        .collect_vec(),
                ),
                _ => Instruction::Set((
                    key[4..key.len() - 1].parse().unwrap(),
                    value.parse().unwrap(),
                )),
            }
        })
        .collect_vec()
}

fn part_1(data: &[Instruction]) -> u64 {
    data.iter()
        .fold(
            (HashMap::new(), [].as_ref()),
            |(mut memory, mut mask), inst| {
                match inst {
                    Instruction::Mask(m) => mask = m,
                    Instruction::Set((addr, value)) => {
                        memory.insert(
                            *addr,
                            mask.iter()
                                .enumerate()
                                .fold(*value, |acc, (i, bit)| match bit {
                                    Bit::One => acc | (1 << i),
                                    Bit::Zero => acc & !(1 << i),
                                    Bit::Floating => acc,
                                }),
                        );
                    }
                };
                (memory, mask)
            },
        )
        .0
        .iter()
        .map(|(_addr, val)| val)
        .sum()
}
fn part_2(data: &[Instruction]) -> u64 {
    data.iter()
        .fold(
            (HashMap::new(), [].as_ref()),
            |(mut memory, mut mask), inst| {
                match inst {
                    Instruction::Mask(m) => mask = m,
                    Instruction::Set((addr, value)) => mask
                        .iter()
                        .enumerate()
                        .fold(vec![*addr], |mut addrs, (i, addr)| {
                            match addr {
                                Bit::One => addrs.iter_mut().for_each(|a| *a |= 1 << i),
                                Bit::Zero => {}
                                Bit::Floating => {
                                    addrs.extend(addrs.clone().iter().map(|a| match (a >> i) & 1 {
                                        0 => a | (1 << i),
                                        1 => a & !(1 << i),
                                        _ => unreachable!(),
                                    }))
                                }
                            };
                            addrs
                        })
                        .iter()
                        .for_each(|addr| {
                            memory.insert(*addr as u64, *value as u64);
                        }),
                };
                (memory, mask)
            },
        )
        .0
        .iter()
        .map(|(_addr, val)| val)
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

    const TEST_INPUT_P1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT_P1.as_bytes());
        assert_eq!(part_1(&parsed), 165);
    }

    const TEST_INPUT_P2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT_P2.as_bytes());
        assert_eq!(part_2(&parsed), 208);
    }
}
