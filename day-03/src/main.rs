use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn slope(data: &[Vec<bool>], right: usize, down: usize) -> usize {
    let row_len = data[0].len();

    (0..data.len())
        .step_by(down)
        .enumerate()
        .map(|(col, row)| (row, (col * right) % row_len))
        .filter(|(row, col)| data[*row][*col])
        .count()
}
fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);

    let data: Vec<_> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Bad data!"),
                })
                .collect::<Vec<bool>>()
        })
        .collect();

    let r3d1 = slope(&data, 3, 1);
    println!("Part 1: {}", r3d1);

    let r1d1 = slope(&data, 1, 1);
    let r5d1 = slope(&data, 5, 1);
    let r7d1 = slope(&data, 7, 1);
    let r1d2 = slope(&data, 1, 2);

    println!(
        "Part 2: {} * {} * {} * {} * {} = {}",
        r3d1,
        r1d1,
        r5d1,
        r7d1,
        r1d2,
        r3d1 * r1d1 * r5d1 * r7d1 * r1d2
    );
}
