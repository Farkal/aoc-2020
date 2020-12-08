use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use std::{collections::VecDeque, fs::File, io::Read, mem::swap};

type Parsed = Vec<Instruction>;
type Out = i32;

#[derive(Clone, Debug)]
enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
    End,
}

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-08.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Parsed {
    input
        .lines()
        .map(|i| {
            let mut iter = i.split_str(" ");
            let cmd = iter.next().unwrap();
            let value = iter.next().unwrap().parse().unwrap();
            match cmd {
                b"jmp" => Instruction::Jmp(value),
                b"acc" => Instruction::Acc(value),
                b"nop" => Instruction::Nop(value),
                _ => panic!(),
            }
        })
        .collect_vec()
}

fn part_1(mut data: Parsed) -> Out {
    let mut acc = 0;
    let mut pc: i32 = 0;

    loop {
        let mut inst = Instruction::End;
        swap(&mut inst, &mut data[pc as usize]);

        pc += 1;

        match inst {
            Instruction::Acc(n) => acc += n,
            Instruction::Jmp(n) => pc = pc - 1 + n,
            Instruction::Nop(_) => {}
            Instruction::End => return acc,
        };
    }
}

fn part_2(mut data: Parsed) -> Out {
    let mut acc = 0;
    let mut pc: i32 = 0;
    let mut q = VecDeque::new();

    let mut did_travel_back_in_time = false;

    while pc < data.len() as i32 {
        let mut inst = Instruction::End;
        swap(&mut inst, &mut data[pc as usize]);

        match inst {
            Instruction::Acc(n) => acc += n,
            Instruction::Jmp(n) => {
                if !did_travel_back_in_time {
                    q.push_back((pc + 1, acc));
                }
                pc = pc - 1 + n;
            }
            Instruction::Nop(0) => {}
            Instruction::Nop(n) => {
                if !did_travel_back_in_time {
                    q.push_back((pc + n, acc));
                }
            }
            Instruction::End => {
                let x = q.pop_back().unwrap();
                pc = x.0;
                acc = x.1;

                did_travel_back_in_time = true;

                continue;
            }
        };

        pc += 1;
    }
    acc
}
fn main() {
    let input = read_input();
    let parsed = parse_input(&input);

    println!("Part 1: {}", part_1(parsed.clone()));
    println!("Part 2: {}", part_2(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(parsed), 5);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(parsed), 8);
    }
}
