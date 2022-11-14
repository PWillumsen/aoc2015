use md5::Digest;

pub fn part_one(input: &str) -> Option<u32> {
    // let mut i = 1;
    let mut i = 254570;
    let mut s = String::new();

    s.push_str(input);
    loop {
        s.push_str(&i.to_string());
        let hash = md5::Md5::digest(&s);
        let hex = format!("{:x}", hash);
        if &hex[..5] == "00000" {
            return Some(i);
        }
        i += 1;
        s.truncate(input.len());
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut i = 1;
    let mut s = String::new();
    s.push_str(input);
    loop {
        s.push_str(&i.to_string());
        let hash = md5::Md5::digest(&s);
        let hex = format!("{:x}", hash);
        if &hex[..6] == "000000" {
            return Some(i);
        }
        i += 1;
        s.truncate(input.len());
    }
}

fn main() {
    let input = &"bgvyzdsv".to_string();
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "abcdef";
        assert_eq!(part_one(&input), Some(609043));
    }
}
