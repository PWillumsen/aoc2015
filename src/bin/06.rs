use itertools::Itertools;

#[derive(Debug)]
enum Event {
    On,
    Off,
    Toggle,
}

fn parse_instruction(s: &str) -> Option<(Event, (u32, u32), (u32, u32))> {
    match s.split(' ').collect::<Vec<&str>>()[..] {
        ["turn", "on", from, _, to] => return Some((Event::On, parse_pair(from), parse_pair(to))),
        ["turn", "off", from, _, to] => {
            return Some((Event::Off, parse_pair(from), parse_pair(to)))
        }
        ["toggle", from, _, to] => return Some((Event::Toggle, parse_pair(from), parse_pair(to))),
        _ => return None,
    }
}

fn parse_pair(to: &str) -> (u32, u32) {
    to.split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut state = [[false; 1000]; 1000];
    input.lines().for_each(|l| {
        let task = parse_instruction(l).unwrap();
        for i in (task.1 .0)..(task.2 .0) + 1 {
            for j in (task.1 .1)..(task.2 .1) + 1 {
                match task.0 {
                    Event::Off => state[i as usize][j as usize] = false,
                    Event::On => state[i as usize][j as usize] = true,
                    Event::Toggle => state[i as usize][j as usize] = !state[i as usize][j as usize],
                }
            }
        }
    });
    Some(state.iter().flatten().filter(|n| **n).count() as u64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut state = [[0; 1000]; 1000];
    input.lines().for_each(|l| {
        let task = parse_instruction(l).unwrap();
        for i in (task.1 .0)..(task.2 .0) + 1 {
            for j in (task.1 .1)..(task.2 .1) + 1 {
                match task.0 {
                    Event::Off => {
                        state[i as usize][j as usize] = (0).max(state[i as usize][j as usize] - 1)
                    }
                    Event::On => state[i as usize][j as usize] = state[i as usize][j as usize] + 1,
                    Event::Toggle => {
                        state[i as usize][j as usize] = state[i as usize][j as usize] + 2
                    }
                }
            }
        }
    });
    Some(state.iter().flatten().sum::<i64>())
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
