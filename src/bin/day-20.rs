#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::{iproduct, Itertools};
use rayon::prelude::*;
use std::{convert::TryInto, fs::File, io::Read};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-20.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sea {
    Wave,
    Clear,
}

#[derive(Clone, Debug)]
struct Tile {
    id: u64,
    border_up: [Sea; 10],
    border_down: [Sea; 10],
    border_left: [Sea; 10],
    border_right: [Sea; 10],
    inner: [[Sea; 8]; 8],
}

impl Tile {
    fn is_match(&self, other: &Tile) -> bool {
        self.id != other.id
            && iproduct!(
                &[
                    &self.border_up,
                    &self.border_down,
                    &self.border_left,
                    &self.border_right
                ],
                &[
                    &other.border_up,
                    &other.border_down,
                    &other.border_left,
                    &other.border_right
                ]
            )
            .any(|(a, b)| a.iter().eq(b.iter()) || a.iter().rev().eq(b.iter()))
    }
}

fn parse_input(input: &[u8]) -> Vec<Tile> {
    input
        .trim()
        .split_str("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines.next().unwrap();
            let id = id[5..id.len() - 1].parse().unwrap();

            let grid = lines
                .map(|l| {
                    l.iter()
                        .map(|c| match c {
                            b'#' => Sea::Wave,
                            b'.' => Sea::Clear,
                            _ => unreachable!(),
                        })
                        .collect_vec()
                })
                .collect_vec();

            let border_up = grid.first().unwrap().clone().try_into().unwrap();
            let border_down = grid.last().unwrap().clone().try_into().unwrap();
            let border_left = grid
                .iter()
                .map(|l| l.first().unwrap())
                .copied()
                .collect_vec()
                .try_into()
                .unwrap();
            let border_right = grid
                .iter()
                .map(|l| l.last().unwrap())
                .copied()
                .collect_vec()
                .try_into()
                .unwrap();

            let inner = grid[1..9]
                .iter()
                .map(|l| l[1..9].to_vec().try_into().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();

            Tile {
                id,
                border_up,
                border_down,
                border_left,
                border_right,
                inner,
            }
        })
        .collect_vec()
}

fn part_1(data: &[Tile]) -> u64 {
    data.par_iter()
        .filter(|tile| data.iter().filter(|other| tile.is_match(other)).count() == 2)
        .map(|tile| tile.id)
        .product()
}

fn part_2(_data: &[Tile]) -> usize {
    0
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

    const TEST_INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 20899048083289);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 273);
    }
}
