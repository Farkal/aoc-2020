#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use itertools::{iproduct, Itertools};
use std::{fs::File, io::Read};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Seat {
    Occupied,
    Empty,
    Floor,
}

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-11.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Vec<Vec<Seat>> {
    input
        .lines()
        .map(|l| {
            l.iter()
                .map(|c| match c {
                    b'#' => Seat::Occupied,
                    b'L' => Seat::Empty,
                    b'.' => Seat::Floor,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec()
}

fn part_1(data: &[Vec<Seat>]) -> usize {
    let mut changed = true;

    let height = data.len();
    let width = data[0].len();

    let mut last_generation;
    let mut current_generation = data.iter().cloned().collect_vec();

    while changed {
        last_generation = current_generation.iter().cloned().collect_vec();

        changed = false;
        for (x, y) in iproduct!(0..height, 0..width) {
            let min_x = x.saturating_sub(1);
            let max_x = if x + 1 == height { x } else { x + 1 };
            let min_y = y.saturating_sub(1);
            let max_y = if y + 1 == width { y } else { y + 1 };

            let mut area =
                iproduct!(min_x..=max_x, min_y..=max_y).filter(|(a, b)| (a, b) != (&x, &y));

            match current_generation[x][y] {
                Seat::Empty => {
                    let should_change = area.all(|(a, b)| last_generation[a][b] != Seat::Occupied);
                    changed = changed || should_change;
                    if should_change {
                        current_generation[x][y] = Seat::Occupied;
                    }
                }

                Seat::Occupied => {
                    let should_change = area
                        .filter(|(a, b)| last_generation[*a][*b] == Seat::Occupied)
                        .count()
                        >= 4;
                    changed = changed || should_change;
                    if should_change {
                        current_generation[x][y] = Seat::Empty;
                    }
                }
                _ => {}
            }
        }
    }

    current_generation
        .iter()
        .map(|l| l.iter().filter(|i| i == &&Seat::Occupied).count())
        .sum()
}

fn part_2(data: &[Vec<Seat>]) -> usize {
    let mut changed = true;

    let height = data.len();
    let width = data[0].len();

    let mut last_generation;
    let mut current_generation = data.iter().cloned().collect_vec();

    while changed {
        last_generation = current_generation.iter().cloned().collect_vec();

        changed = false;
        for (x, y) in iproduct!(0..height, 0..width) {
            let mut area = iproduct!(-1i64..=1i64, -1i64..=1i64)
                .filter(|(a, b)| (a, b) != (&0, &0))
                .map(|(a, b)| {
                    (1..)
                        .map(|n| {
                            last_generation
                                .get(if a < 0 {
                                    x.checked_sub((n * -a) as usize)?
                                } else {
                                    x + (n * a) as usize
                                })?
                                .get(if b < 0 {
                                    y.checked_sub((n * -b) as usize)?
                                } else {
                                    y + (n * b) as usize
                                })
                        })
                        .find_map(|seat| match seat {
                            None => Some(Seat::Floor),
                            Some(Seat::Floor) => None,
                            Some(s) => Some(s.clone()),
                        })
                        .unwrap()
                });

            match current_generation[x][y] {
                Seat::Empty => {
                    let should_change = area.all(|s| s != Seat::Occupied);
                    changed = changed || should_change;
                    if should_change {
                        current_generation[x][y] = Seat::Occupied;
                    }
                }

                Seat::Occupied => {
                    let should_change = area.filter(|s| s == &Seat::Occupied).count() >= 5;
                    changed = changed || should_change;
                    if should_change {
                        current_generation[x][y] = Seat::Empty;
                    }
                }
                _ => {}
            }
        }
    }

    current_generation
        .iter()
        .map(|l| l.iter().filter(|i| i == &&Seat::Occupied).count())
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

    const TEST_INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 37);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 26);
    }
}
