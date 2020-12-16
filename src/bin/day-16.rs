#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::{izip, Itertools};
use regex::bytes::Regex;
use std::{collections::HashSet, fs::File, io::Read, ops::RangeInclusive};

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-16.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_input(
    input: &[u8],
) -> (
    Vec<Vec<usize>>,
    Vec<(RangeInclusive<usize>, RangeInclusive<usize>, bool)>,
) {
    let (rules, my_ticket, nearby_tickets) = input.split_str("\n\n").collect_tuple().unwrap();

    let re = Regex::new(r".+: (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)").unwrap();

    let rules = rules
        .lines()
        .map(|r| {
            let m = re.captures(r).unwrap();

            (
                m["min1"].parse().unwrap()..=m["max1"].parse().unwrap(),
                m["min2"].parse().unwrap()..=m["max2"].parse().unwrap(),
                r.starts_with_str("departure"),
            )
        })
        .collect_vec();

    let tickets = my_ticket
        .lines()
        .skip(1)
        .chain(nearby_tickets.lines().skip(1))
        .map(|l| l.split_str(",").map(|i| i.parse().unwrap()).collect_vec())
        .collect_vec();

    (tickets, rules)
}

fn part_1(
    tickets: &[Vec<usize>],
    rules: &[(RangeInclusive<usize>, RangeInclusive<usize>, bool)],
) -> usize {
    tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|field| {
                    rules
                        .iter()
                        .all(|(r0, r1, _)| !r0.contains(field) && !r1.contains(field))
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_2(
    tickets: &[Vec<usize>],
    rules: &[(RangeInclusive<usize>, RangeInclusive<usize>, bool)],
) -> usize {
    let mut possibilities: Vec<HashSet<usize>> = (0..rules.len())
        .map(|_| (0..rules.len()).collect())
        .collect();

    let valid_tickets = tickets.iter().filter(|ticket| {
        ticket.iter().all(|field| {
            rules
                .iter()
                .any(|(r0, r1, _)| r0.contains(field) || r1.contains(field))
        })
    });

    valid_tickets.for_each(|ticket| {
        ticket.iter().enumerate().for_each(|(i, field)| {
            izip!(rules.iter(), possibilities.iter_mut()).for_each(|((r0, r1, _), possible_set)| {
                if possible_set.contains(&i) && !r0.contains(field) && !r1.contains(field) {
                    possible_set.remove(&i);
                }
            })
        });
    });

    let mut changed = true;
    while changed {
        changed = false;
        let remove_list = possibilities
            .iter()
            .filter_map(|i| i.iter().exactly_one().ok())
            .cloned()
            .collect_vec();

        possibilities
            .iter_mut()
            .filter(|p| p.len() > 1)
            .for_each(|p| {
                remove_list
                    .iter()
                    .for_each(|r| changed = changed || p.remove(r))
            });
    }

    possibilities
        .iter()
        .zip(rules.iter())
        .filter(|(_, (_, _, pred))| *pred)
        .map(|s| s.0.iter().exactly_one().unwrap())
        .map(|i| tickets[0][*i])
        .product()
}

fn main() {
    let input = read_input();
    let (tickets, rules) = parse_input(&input);

    println!("Part 1: {}", part_1(&tickets, &rules));
    println!("Part 2: {}", part_2(&tickets, &rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part_1() {
        let (tickets, rules) = parse_input(TEST_INPUT.as_bytes());
        assert_eq!(part_1(&tickets, &rules), 71);
    }
}
