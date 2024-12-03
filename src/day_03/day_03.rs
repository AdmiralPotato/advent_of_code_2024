use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let text_file_string = read_to_string("./src/day_03/input.txt").unwrap();
    let part_1_result = part_1(&text_file_string);
    // expect 189527826
    println!("Part 1: {part_1_result}");
    let part_2_result = part_2(&text_file_string);
    // expect 63013756
    println!("Part 2: {part_2_result}");
}

fn part_1(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)").unwrap();
    let mut result = 0;
    for captures in regex.captures_iter(input) {
        let a: u32 = captures[1].parse().unwrap();
        let b: u32 = captures[2].parse().unwrap();
        // dbg!(a, b);
        result += a * b;
    }
    result
}

fn part_2(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for captures in regex.captures_iter(input) {
        let op: &str = captures[0].trim();
        if op == "do()" {
            enabled = true;
            continue;
        }
        if op == "don't()" {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        let a: u32 = captures[1].parse().unwrap();
        let b: u32 = captures[2].parse().unwrap();
        // dbg!(a, b);
        result += a * b;
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let result = super::part_1(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(result, 161);
    }

    #[test]
    fn part_2() {
        let result = super::part_2(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, 48);
    }
}
