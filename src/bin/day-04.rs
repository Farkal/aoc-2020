use bstr::ByteSlice;
use bstr_parse::*;
use regex::bytes::Regex;
use std::{fs::File, io::Read};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-04.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

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

fn part_1(data: &[u8]) -> usize {
    let re = Regex::new(r"(\S+):").unwrap();

    let out = data
        .lines()
        .fold((0, Passport::default()), |(count, pass), line| {
            if line.is_empty() {
                return (count + pass.is_complete() as usize, Passport::default());
            }
            let pass = re.captures_iter(&line).fold(pass, |mut acc, x| {
                match x.get(1).unwrap().as_bytes() {
                    b"ecl" => acc.ecl = true,
                    b"pid" => acc.pid = true,
                    b"eyr" => acc.eyr = true,
                    b"hcl" => acc.hcl = true,
                    b"byr" => acc.byr = true,
                    b"iyr" => acc.iyr = true,
                    b"cid" => acc.cid = true,
                    b"hgt" => acc.hgt = true,
                    v => panic!("unknown {}", std::str::from_utf8(v).unwrap()),
                };
                acc
            });
            (count, pass)
        });

    out.0 + out.1.is_complete() as usize
}

fn part_2(data: &[u8]) -> usize {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let height_re = Regex::new(r"(?P<height>\d+)(?P<unit>\D+)").unwrap();

    let out = data
        .lines()
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
                    b"eyr" => acc.eyr = (2020..=2030).contains(&value.parse().unwrap_or(0)),
                    b"hcl" => {
                        acc.hcl = value[0] == b'#'
                            && value.len() == 7
                            && value
                                .iter()
                                .skip(1)
                                .all(|a| (b'0'..=b'9').contains(a) || (b'a'..=b'f').contains(a))
                    }
                    b"byr" => {
                        acc.byr =
                            value.len() == 4 && (1920..=2002).contains(&value.parse().unwrap_or(0))
                    }
                    b"iyr" => {
                        acc.iyr =
                            value.len() == 4 && (2010..=2020).contains(&value.parse().unwrap_or(0))
                    }
                    b"cid" => acc.cid = true,
                    b"hgt" => {
                        acc.hgt = height_re.captures(value).map_or(false, |caps| {
                            let height = &caps["height"].parse().unwrap_or(0);
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

    out.0 + out.1.is_complete() as usize
}

fn main() {
    let input = read_input();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT.as_bytes()), 2);
    }

    const P2_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const P2_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_part_2() {
        assert_eq!(part_1(&P2_VALID.as_bytes()), part_2(&P2_VALID.as_bytes()));
        assert_eq!(part_2(&P2_INVALID.as_bytes()), 0);
    }
}
