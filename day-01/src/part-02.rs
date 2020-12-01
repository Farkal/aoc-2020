use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);

    let mut data: Vec<_> = reader
        .lines()
        .map(|l| l.unwrap().parse::<u16>().unwrap())
        .collect();

    data.sort_unstable();

    for i in &data {
        for x in data.iter().filter_map(|j| 2020_u16.checked_sub(i + j)) {
            if data.binary_search(&x).is_ok() {
                let j = 2020 - x - i;
                println!("{} * {} * {} = {}", x, i, j, j * x * i);
                return;
            }
        }
    }
}
