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
    let re = Regex::new(r"(\S+):(\S+)").unwrap();

    let height_re = Regex::new(r"(?P<height>\d+)(?P<unit>\D+)").unwrap();

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
                let key = x.get(1).unwrap().as_str();
                let value = x.get(2).unwrap().as_str();
                match key {
                    "ecl" => {
                        acc.ecl = match value {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                            _ => false,
                        }
                    }
                    "pid" => {
                        acc.pid =
                            value.as_bytes().len() == 9 && value.as_bytes().iter().all(|a| (b'0'..=b'9').contains(a))
                    }
                    "eyr" => acc.eyr = (2020..=2030).contains(&value.parse::<u16>().unwrap_or(0)),
                    "hcl" => {
                        acc.hcl = value.as_bytes()[0] == b'#'
                            && value.len() == 7
                            && value.as_bytes()[1..]
                                .iter()
                                .all(|a| (b'0'..=b'9').contains(a) || (b'a'..=b'f').contains(a))
                    }
                    "byr" => {
                        acc.byr = value.len() == 4
                            && (1920..=2002).contains(&value.parse::<u16>().unwrap_or(0))
                    }
                    "iyr" => {
                        acc.iyr = value.len() == 4
                            && (2010..=2020).contains(&value.parse::<u16>().unwrap_or(0))
                    }
                    "cid" => acc.cid = true,
                    "hgt" => {
                        acc.hgt = height_re
                            .captures(value)
                            .map(|caps| {
                                let height = caps["height"].parse::<u16>();
                                let unit = &caps["unit"];
                                match (height, unit) {
                                    (Ok(height), "cm") => (150..=193).contains(&height),
                                    (Ok(height), "in") => (59..=76).contains(&height),
                                    (_, _) => false,
                                }
                            })
                            .unwrap_or(false)
                    }
                    v => panic!("uknown {}", v),
                };
                acc
            })
        });

    if last.is_complete() {
        out += 1;
    }

    println!("Part 2: {}", out);
}
