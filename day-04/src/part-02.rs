use bstr::io::BufReadExt;
use regex::bytes::Regex;
use std::fs::File;
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
        .byte_lines()
        .map(|i| i.unwrap())
        .fold(Passport::default(), |acc, line| {
            if line.is_empty() {
                if acc.is_complete() {
                    out += 1;
                }
                return Passport::default();
            }
            re.captures_iter(&line).fold(acc, |mut acc, x| {
                let key = x.get(1).unwrap().as_bytes();
                let value = x.get(2).unwrap().as_bytes();
                match key {
                    b"ecl" => {
                        acc.ecl = matches!(value, b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth")
                    }
                    b"pid" => {
                        acc.pid =
                            value.len() == 9 && value.iter().all(|a| (b'0'..=b'9').contains(a))
                    }
                    b"eyr" => {
                        acc.eyr = (2020..=2030).contains(
                            &std::str::from_utf8(value)
                                .ok()
                                .and_then(|x| x.parse::<u16>().ok())
                                .unwrap_or(0),
                        )
                    }
                    b"hcl" => {
                        acc.hcl = value[0] == b'#'
                            && value.len() == 7
                            && value[1..]
                                .iter()
                                .all(|a| (b'0'..=b'9').contains(a) || (b'a'..=b'f').contains(a))
                    }
                    b"byr" => {
                        acc.byr = value.len() == 4
                            && (1920..=2002).contains(
                                &std::str::from_utf8(value)
                                    .ok()
                                    .and_then(|x| x.parse::<u16>().ok())
                                    .unwrap_or(0),
                            )
                    }
                    b"iyr" => {
                        acc.iyr = value.len() == 4
                            && (2010..=2020).contains(
                                &std::str::from_utf8(value)
                                    .ok()
                                    .and_then(|x| x.parse::<u16>().ok())
                                    .unwrap_or(0),
                            )
                    }
                    b"cid" => acc.cid = true,
                    b"hgt" => {
                        acc.hgt = height_re
                            .captures(value)
                            .map(|caps| {
                                let height = std::str::from_utf8(&caps["height"])
                                    .ok()
                                    .and_then(|x| x.parse::<u16>().ok());
                                let unit = &caps["unit"];
                                match (height, unit) {
                                    (Some(height), b"cm") => (150..=193).contains(&height),
                                    (Some(height), b"in") => (59..=76).contains(&height),
                                    (_, _) => false,
                                }
                            })
                            .unwrap_or(false)
                    }
                    v => panic!("uknown {}", std::str::from_utf8(v).unwrap()),
                };
                acc
            })
        });

    if last.is_complete() {
        out += 1;
    }

    println!("Part 2: {}", out);
}
