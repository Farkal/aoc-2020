use bstr::ByteSlice;
use std::{fs::File, io::Read};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-03.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn slope(data: &[Vec<bool>], right: usize, down: usize) -> usize {
    let row_len = data[0].len();

    (0..data.len())
        .step_by(down)
        .enumerate()
        .map(|(col, row)| (row, (col * right) % row_len))
        .filter(|(row, col)| data[*row][*col])
        .count()
}

fn parse_input(input: &[u8]) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| {
            l.iter()
                .map(|c| match c {
                    b'.' => false,
                    b'#' => true,
                    _ => panic!(),
                })
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn part_1(data: &[Vec<bool>]) -> usize {
    slope(data, 3, 1)
}

fn part_2(data: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| slope(data, *right, *down))
        .product()
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

    static EXAMPLE_INPUT: &str = concat!(
        "..##.......\n",
        "#...#...#..\n",
        ".#....#..#.\n",
        "..#.#...#.#\n",
        ".#...##..#.\n",
        "..#.##.....\n",
        ".#.#.#....#\n",
        ".#........#\n",
        "#.##...#...\n",
        "#...##....#\n",
        ".#..#...#.#\n",
    );

    #[test]
    fn test_part_1() {
        let parsed = parse_input(&EXAMPLE_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 7);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(&EXAMPLE_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 336);
    }
}
