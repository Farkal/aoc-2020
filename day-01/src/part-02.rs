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

    data.sort();

    for i in &data {
        for j in &data {
            let x = 2020_u16.checked_sub(*i).and_then(|t| t.checked_sub(*j));

            if let Some(x) = x {
                if data.binary_search(&x).is_ok() {
                    println!("{} * {} * {} = {}", x, i, j, j * x * i);
                    return;
                }
            }
        }
    }
}
