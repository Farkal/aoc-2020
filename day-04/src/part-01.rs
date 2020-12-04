use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Default, Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport {
    fn is_complete(&self) -> bool {
        self.ecl && self.pid && self.eyr && self.hcl && self.byr && self.iyr && self.hgt
    }
}

fn main() {
    let re = Regex::new(r"(\S+):").unwrap();

    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);

    let mut out = 0;

    let last = reader
        .lines()
        .map(|i| i.unwrap())
        .fold(Passport::default(), |acc, line| {
            if line.is_empty() {
                if acc.is_complete() {
                    out += 1;
                }
                return Passport::default();
            }
            re.captures_iter(&line).fold(acc, |mut acc, x| {
                match x.get(1).unwrap().as_str() {
                    "ecl" => acc.ecl = true,
                    "pid" => acc.pid = true,
                    "eyr" => acc.eyr = true,
                    "hcl" => acc.hcl = true,
                    "byr" => acc.byr = true,
                    "iyr" => acc.iyr = true,
                    "cid" => acc.cid = true,
                    "hgt" => acc.hgt = true,
                    v => panic!("uknown {}", v),
                };
                acc
            })
        });

    if last.is_complete() {
        out += 1;
    }

    println!("Part 1: {}", out);

}
