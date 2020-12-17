use bstr::ByteSlice;
use itertools::{iproduct, Itertools};
use std::{collections::HashSet, fs::File, io::Read};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-17.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(input: &[u8]) -> Vec<(i16, i16)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, l)| {
            l.iter()
                .enumerate()
                .filter(|(_y, value)| value == &&b'#')
                .map(move |(y, _value)| (x as i16, y as i16))
        })
        .collect()
}

fn part_1(data: &[(i16, i16)]) -> usize {
    let mut out: HashSet<(i16, i16, i16)> = data.iter().map(|(x, y)| (*x, *y, 0)).collect();
    for _ in 0..6 {
        let data = out.clone();
        let dim_x = data.iter().map(|p| p.0).minmax().into_option().unwrap();
        let dim_y = data.iter().map(|p| p.1).minmax().into_option().unwrap();
        let dim_z = data.iter().map(|p| p.2).minmax().into_option().unwrap();

        for (x, y, z) in iproduct!(
            (dim_x.0 - 1)..=(dim_x.1 + 1),
            (dim_y.0 - 1)..=(dim_y.1 + 1),
            (dim_z.0 - 1)..=(dim_z.1 + 1)
        ) {
            let neighbors = iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1), (z - 1)..=(z + 1))
                .filter(|pos| pos != &(x, y, z));

            let active_count = neighbors.filter(|pos| data.contains(&pos)).count();

            match (data.contains(&(x, y, z)), active_count) {
                (true, 2..=3) => {}
                (true, _) => {
                    out.remove(&(x, y, z));
                }
                (false, 3) => {
                    out.insert((x, y, z));
                }
                (_, _) => {}
            };
        }
    }
    out.len()
}

fn part_2(data: &[(i16, i16)]) -> usize {
    let mut out: HashSet<(i16, i16, i16, i16)> = data.iter().map(|(x, y)| (*x, *y, 0, 0)).collect();
    for _ in 0..6 {
        let data = out.clone();
        let dim_x = data.iter().map(|p| p.0).minmax().into_option().unwrap();
        let dim_y = data.iter().map(|p| p.1).minmax().into_option().unwrap();
        let dim_z = data.iter().map(|p| p.2).minmax().into_option().unwrap();
        let dim_w = data.iter().map(|p| p.3).minmax().into_option().unwrap();

        for (x, y, z, w) in iproduct!(
            (dim_x.0 - 1)..=(dim_x.1 + 1),
            (dim_y.0 - 1)..=(dim_y.1 + 1),
            (dim_z.0 - 1)..=(dim_z.1 + 1),
            (dim_w.0 - 1)..=(dim_w.1 + 1)
        ) {
            let neighbors = iproduct!(
                (x - 1)..=(x + 1),
                (y - 1)..=(y + 1),
                (z - 1)..=(z + 1),
                (w - 1)..=(w + 1)
            )
            .filter(|pos| pos != &(x, y, z, w));

            let active_count = neighbors.filter(|pos| data.contains(&pos)).count();

            match (data.contains(&(x, y, z, w)), active_count) {
                (true, 2..=3) => {}
                (true, _) => {
                    out.remove(&(x, y, z, w));
                }
                (false, 3) => {
                    out.insert((x, y, z, w));
                }
                (_, _) => {}
            };
        }
    }
    out.len()
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

    const TEST_INPUT: &str = ".#.
..#
###
";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&parsed), 112);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_2(&parsed), 848);
    }
}
