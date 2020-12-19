#![feature(destructuring_assignment)]

use bstr::ByteSlice;
use bstr_parse::*;
use itertools::Itertools;
use regex::bytes::Regex;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Clone)]
enum Rule {
    Seq(Vec<u8>),
    Or(Vec<u8>, Vec<u8>),
    Lit(char),
}

fn read_input() -> Vec<u8> {
    let mut out = Vec::new();
    let mut f = File::open("input/day-19.txt").unwrap();

    f.read_to_end(&mut out).unwrap();
    out
}

fn parse_list(input: &[u8]) -> Vec<u8> {
    input
        .trim()
        .split_str(" ")
        .map(|i| i.parse().unwrap())
        .collect_vec()
}

fn parse_input(input: &[u8]) -> (HashMap<u8, Rule>, Vec<Vec<u8>>) {
    let (rules, messages) = input.splitn_str(2, "\n\n").collect_tuple().unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (num, rule) = line.splitn_str(2, ":").collect_tuple().unwrap();

            let num = num.parse().unwrap();

            let rule = if let Some(lit) = rule.split_str("\"").nth(1) {
                assert_eq!(lit.len(), 1);
                Rule::Lit(match &lit[0] {
                    b'a' => 'a',
                    b'b' => 'b',
                    _ => unreachable!(),
                })
            } else if let Some((a, b)) = rule.split_str("|").collect_tuple() {
                let rule_a = parse_list(a);
                let rule_b = parse_list(b);

                Rule::Or(rule_a, rule_b)
            } else {
                Rule::Seq(parse_list(rule))
            };

            (num, rule)
        })
        .collect();

    let messages = messages.lines().map(|line| line.to_vec()).collect_vec();

    (rules, messages)
}

fn format_rules(
    rule: &Rule,
    rule_id: Option<u8>,
    rules: &HashMap<u8, Rule>,
    cache: &mut HashMap<u8, String>,
) -> String {
    if let Some(ref id) = rule_id {
        if let Some(cached) = cache.get(id) {
            return cached.clone();
        }
    }

    let out = match rule {
        Rule::Lit(lit) => lit.to_string(),
        Rule::Seq(seq) => format!(
            "(?:{})",
            seq.iter()
                .map(|r| format_rules(rules.get(r).unwrap(), Some(*r), rules, cache))
                .join("")
        ),
        Rule::Or(a, b) => format!(
            "(?:{}|{})",
            format_rules(&Rule::Seq(a.to_vec()), None, rules, cache),
            format_rules(&Rule::Seq(b.to_vec()), None, rules, cache)
        ),
    };

    if let Some(id) = rule_id {
        cache.insert(id, out.clone());
    }

    out
}

fn part_1(rules: &mut HashMap<u8, Rule>, messsages: &[Vec<u8>]) -> usize {
    let re = Regex::new(&format!(
        "^(:?{})$",
        format_rules(rules.get(&0).unwrap(), Some(0), rules, &mut HashMap::new())
    ))
    .unwrap();

    messsages
        .iter()
        .filter(|message| re.is_match(message))
        .count()
}

fn part_2(rules: &mut HashMap<u8, Rule>, messsages: &[Vec<u8>]) -> usize {
    let mut cache = HashMap::new();
    let rule42 = format_rules(rules.get(&42).unwrap(), Some(42), rules, &mut cache);

    cache.insert(8, format!("{}+", rule42));

    let rule31 = format_rules(rules.get(&31).unwrap(), Some(31), rules, &mut cache);
    let fake_loop_2 = (1..=10)
        .rev()
        .map(|i| format!("(?:(?:{}{{{}}})(?:{}{{{}}}))", rule42, i, rule31, i))
        .join("|");
    cache.insert(11, format!("(?:{})", fake_loop_2));

    let re = Regex::new(&format!(
        "^(:?{})$",
        format_rules(rules.get(&0).unwrap(), Some(0), rules, &mut cache)
    ))
    .unwrap();

    messsages
        .iter()
        .filter(|message| re.is_match(message))
        .count()
}

fn main() {
    let input = read_input();
    let (mut rules, messages) = parse_input(&input);

    println!("Part 1: {}", part_1(&mut rules, &messages));
    println!("Part 2: {}", part_2(&mut rules, &messages));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";

    const TEST_INPUT_P2: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";

    #[test]
    fn test_part_1() {
        let (mut rules, messages) = parse_input(TEST_INPUT_P1.as_bytes());
        assert_eq!(part_1(&mut rules, &messages), 2);
    }

    #[test]
    fn test_part_2() {
        let (mut rules, messages) = parse_input(TEST_INPUT_P2.as_bytes());
        assert_eq!(part_2(&mut rules, &messages), 12);
    }
}
