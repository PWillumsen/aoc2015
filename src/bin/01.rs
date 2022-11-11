pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().map(|c| if c == '(' { 1 } else { -1 }).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut count = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }
        if count == -1 {
            return Some(i as i32 + 1);
        }
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
