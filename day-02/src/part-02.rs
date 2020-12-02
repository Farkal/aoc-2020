use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<char>[[:alpha:]]):\s(?P<pw>[[:alpha:]]+)")
        .unwrap();

    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);

    let out = reader
        .lines()
        .filter_map(|i| i.ok())
        .filter(|line| {
            let caps = re.captures(&line).unwrap();

            let min: usize = caps["min"].parse().unwrap();
            let max: usize = caps["max"].parse().unwrap();

            let c = caps["char"].as_bytes()[0];
            let a = caps["pw"].as_bytes().get(min - 1);
            let b = caps["pw"].as_bytes().get(max - 1);

            (a == Some(&c)) ^ (b == Some(&c))
        })
        .count();

    println!("Number of compliant passwords: {}", out);
}
