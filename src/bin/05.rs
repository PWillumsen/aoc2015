use std::collections::HashMap;

fn count_vowels(s: &str) -> u32 {
    s.chars().filter(|c| ("aeiou").contains(*c)).count() as u32
}

fn check_substring_part1(s: &str) -> bool {
    let mut double = false;
    let mut illigal = false;
    s.as_bytes().windows(2).for_each(|w| match w {
        [b'a', b'b'] => illigal = true,
        [b'c', b'd'] => illigal = true,
        [b'p', b'q'] => illigal = true,
        [b'x', b'y'] => illigal = true,
        [a, b] if a == b => double = true,
        _ => (),
    });
    double && !illigal
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter(|l| count_vowels(l) > 2 && check_substring_part1(l))
        .count() as u32;
    Some(res)
}

fn check_one_beween_same(s: &str) -> bool {
    let mut double = false;
    s.as_bytes().windows(3).for_each(|w| match w {
        [a, _, b] if a == b => double = true,
        _ => (),
    });
    double
}

fn has_two_pairs(s: &str) -> bool {
    let mut pairs: HashMap<(String, String), u32> = HashMap::new();
    let mut last = ["".to_string(), "".to_string()];

    s.as_bytes().windows(2).for_each(|w| match w {
        [a, b] if [a.to_string(), b.to_string()] != last => {
            *pairs.entry((a.to_string(), b.to_string())).or_insert(0) += 1;
            last = [a.to_string(), b.to_string()];
        }
        [a, b] => last = [a.to_string(), b.to_string()],
        _ => (),
    });

    pairs.values().any(|v| v > &1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter(|l| has_two_pairs(l) && check_one_beween_same(l))
        .count();

    Some(res as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
