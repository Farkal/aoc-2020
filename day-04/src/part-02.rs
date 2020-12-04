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

fn bytes_as_u16(value: &[u8]) -> u16 {
    std::str::from_utf8(value)
        .ok()
        .and_then(|x| x.parse::<u16>().ok())
        .unwrap_or(0)
}

fn main() {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();

    let height_re = Regex::new(r"(?P<height>\d+)(?P<unit>\D+)").unwrap();

    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);

    let out = reader
        .byte_lines()
        .map(|i| i.unwrap())
        .chain(std::iter::once(Vec::new()))
        .fold((0, Passport::default()), |(count, pass), line| {
            if line.is_empty() {
                return (count + pass.is_complete() as usize, Passport::default());
            }
            let pass = re.captures_iter(&line).fold(pass, |mut acc, x| {
                let key = x.get(1).unwrap().as_bytes();
                let value = x.get(2).unwrap().as_bytes();
                match key {
                    b"ecl" => {
                        acc.ecl = matches!(
                            value,
                            b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth"
                        )
                    }
                    b"pid" => {
                        acc.pid =
                            value.len() == 9 && value.iter().all(|a| (b'0'..=b'9').contains(a))
                    }
                    b"eyr" => acc.eyr = (2020..=2030).contains(&bytes_as_u16(value)),
                    b"hcl" => {
                        acc.hcl = value[0] == b'#'
                            && value.len() == 7
                            && value
                                .iter()
                                .skip(1)
                                .all(|a| (b'0'..=b'9').contains(a) || (b'a'..=b'f').contains(a))
                    }
                    b"byr" => {
                        acc.byr = value.len() == 4 && (1920..=2002).contains(&bytes_as_u16(value))
                    }
                    b"iyr" => {
                        acc.iyr = value.len() == 4 && (2010..=2020).contains(&bytes_as_u16(value))
                    }
                    b"cid" => acc.cid = true,
                    b"hgt" => {
                        acc.hgt = height_re.captures(value).map_or(false, |caps| {
                            let height = bytes_as_u16(&caps["height"]);
                            matches!(
                                (height, &caps["unit"]),
                                (150..=193, b"cm") | (59..=76, b"in")
                            )
                        })
                    }
                    v => panic!("unknown {}", std::str::from_utf8(v).unwrap()),
                };
                acc
            });
            (count, pass)
        });

    println!("Part 2: {}", out.0);
}
