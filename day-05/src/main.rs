use bstr::io::BufReadExt;
use itertools::Itertools;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);

    let out: Vec<_> = reader
        .byte_lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.iter()
                .fold(0, |acc, c| acc << 1 | (c == &b'B' || c == &b'R') as u16)
        })
        .collect();

    let max_seat = out.iter().max().unwrap_or(&0);
    println!("Part 1: {}", max_seat);

    let my_seat = out
        .iter()
        .sorted()
        .tuple_windows()
        .find_map(|(a, b)| if a + 1 != *b { Some(a + 1) } else { None })
        .unwrap();

    println!("Part 2: {}", my_seat);
}
