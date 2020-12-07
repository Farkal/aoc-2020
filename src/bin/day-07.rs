use bstr::ByteSlice;
use bstr_parse::*;
use regex::bytes::Regex;
use std::{collections::HashMap, fs::File, io::Read};

type Out = usize;

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-07.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input<'a>(input: &'a [u8]) -> HashMap<&'a [u8], HashMap<&'a [u8], usize>> {
    let re = Regex::new(r"(?P<count>\d+) (?P<color>.+?) bags?").unwrap();

    input.lines().fold(
        HashMap::new(),
        |mut m: HashMap<&'a [u8], HashMap<&'a [u8], usize>>, line: &'a [u8]| {
            let mut split = line.split_str(" bags contain ");
            let key = split.next().unwrap();
            let value =
                re.captures_iter(split.next().unwrap())
                    .fold(HashMap::new(), |mut acc, x| {
                        let key = Box::new(x["color"].as_bytes().to_owned());
                        let key: &'a [u8] = Box::leak(key);
                        let value = x["count"].as_bytes().parse().unwrap();

                        acc.insert(key, value);
                        acc
                    });
            m.insert(key, value);
            m
        },
    )
}

fn count_shiny_gold_bags<'a>(
    data: &'a HashMap<&'a [u8], HashMap<&'a [u8], usize>>,
    key: &'a [u8],
    seen: &mut HashMap<&'a [u8], usize>,
) -> usize {
    let bag_content = data.get(key).unwrap();
    let shiny_gold = b"shiny gold";

    if let Some(amount) = bag_content.get(&shiny_gold[..]) {
        seen.insert(key, *amount);
        return *amount;
    }

    let amount = bag_content
        .iter()
        .map(|(key, value)| {
            if let Some(amount) = seen.get(key) {
                value * amount
            } else {
                value * count_shiny_gold_bags(data, key, seen)
            }
        })
        .max()
        .unwrap_or(0);

    seen.insert(key, amount);
    return amount;
}

fn count_bags_in<'a>(
    data: &'a HashMap<&'a [u8], HashMap<&'a [u8], usize>>,
    key: &'a [u8],
    seen: &mut HashMap<&'a [u8], usize>,
) -> usize {
    let bag_content = data.get(key).unwrap();

    let amount = bag_content
        .iter()
        .map(|(key, value)| {
            if let Some(amount) = seen.get(key) {
                value * amount
            } else {
                value * count_bags_in(data, key, seen)
            }
        })
        .sum::<usize>()
        + 1;

    seen.insert(key, amount);
    return amount;
}

fn part_1<'a>(data: &'a HashMap<&'a [u8], HashMap<&'a [u8], usize>>) -> Out {
    let mut shiny = HashMap::new();
    data.iter()
        .filter(|(key, _map)| count_shiny_gold_bags(data, key, &mut shiny) > 0)
        .count()
}

fn part_2<'a>(data: &'a HashMap<&'a [u8], HashMap<&'a [u8], usize>>) -> Out {
    let mut shiny = HashMap::new();
    count_bags_in(data, b"shiny gold", &mut shiny) - 1
}
fn main() {
    let input = read_input();
    let parsed = parse_input(&input);

    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_INPUT_P2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_part_1() {
        let parsed = parse_input(TEST_INPUT_P1.as_bytes());
        assert_eq!(part_1(&parsed), 4);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse_input(TEST_INPUT_P2.as_bytes());
        assert_eq!(part_2(&parsed), 126);
    }
}
