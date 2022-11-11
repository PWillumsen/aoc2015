pub fn part_one(input: &str) -> Option<u32> {
    let mut ft2 = 0;
    for line in input.lines() {
        let dims = line
            .split('x')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if let [h, w, l] = &dims[..] {
            ft2 += 2 * h * w + 2 * h * l + 2 * w * l + [h * w, h * l, w * l].iter().min().unwrap();
        }
    }
    Some(ft2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ft = 0;
    for line in input.lines() {
        let mut dims = line
            .split('x')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        dims.sort();

        ft += 2 * dims[0] + 2 * dims[1] + dims[0] * dims[1] * dims[2];
    }
    Some(ft)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
