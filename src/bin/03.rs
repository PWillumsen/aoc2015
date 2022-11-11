use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut houses = HashSet::new();
    let mut current = (0, 0);
    houses.insert(current);

    for dir in input.chars() {
        let (x, y) = current;
        match dir {
            '<' => current = (x - 1, y),
            '>' => current = (x + 1, y),
            '^' => current = (x, y + 1),
            'v' => current = (x, y - 1),
            _ => unreachable!(),
        }
        houses.insert(current);
    }

    Some(houses.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut houses = HashSet::new();
    let mut santa = (0, 0);
    let mut robot = (0, 0);
    houses.insert(santa);

    for (i, dir) in input.chars().enumerate() {
        let turn = if i % 2 == 0 { &mut santa } else { &mut robot };
        let next_move = match dir {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, 1),
            'v' => (0, -1),
            _ => unreachable!(),
        };
        *turn = (turn.0 + next_move.0, turn.1 + next_move.1);
        houses.insert(*turn);
    }
    Some(houses.len() as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "^v^v^v^v^v";
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = "^v^v^v^v^v";
        assert_eq!(part_two(&input), Some(11));
    }
}
