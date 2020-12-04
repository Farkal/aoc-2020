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
    let re = Regex::new(r"(\S+):").unwrap();

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
                match x.get(1).unwrap().as_bytes() {
                    b"ecl" => acc.ecl = true,
                    b"pid" => acc.pid = true,
                    b"eyr" => acc.eyr = true,
                    b"hcl" => acc.hcl = true,
                    b"byr" => acc.byr = true,
                    b"iyr" => acc.iyr = true,
                    b"cid" => acc.cid = true,
                    b"hgt" => acc.hgt = true,
                    v => panic!("uknown {}", std::str::from_utf8(v).unwrap()),
                };
                acc
            });
            (count, pass)
        });

    println!("Part 1: {}", out.0);
}
