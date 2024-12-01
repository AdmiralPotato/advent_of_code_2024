use std::collections::{HashMap};
use std::fs::read_to_string;

fn main() {
    let text_file_string = read_to_string("./src/day_01/input.txt").unwrap();
    let part_1_result = part_1(&text_file_string);
    let part_2_result = part_2(&text_file_string);
    println!("Part 1: {part_1_result}");
    println!("Part 2: {part_2_result}");
}

fn part_1(input: &str) -> i32 {
    let mut l: Vec<i32> = vec![];
    let mut r: Vec<i32> = vec![];
    input.split('\n').for_each(|line| {
        let (a, b) = line.trim().split_once(' ').unwrap();
        l.push(a.trim().parse::<i32>().unwrap());
        r.push(b.trim().parse::<i32>().unwrap());
    });
    l.sort();
    r.sort();
    l
        .iter()
        .copied()
        .zip(r.iter().copied())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_2(input: &str) -> i32 {
    let mut l: Vec<i32> = vec![];
    let mut r: HashMap<i32, i32> = HashMap::new();
    input.split('\n').for_each(|line| {
        let (a, b) = line.trim().split_once(' ').unwrap();
        let b = b.trim().parse::<i32>().unwrap();
        l.push(a.trim().parse::<i32>().unwrap());
        *(r.entry(b).or_insert(0)) += 1;
    });
    l
        .iter()
        .copied()
        .map(|a| a * r.get(&a).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_1() {
        let result = super::part_1("\
            3   4\n\
            4   3\n\
            2   5\n\
            1   3\n\
            3   9\n\
            3   3\n\
        ".trim());
        assert_eq!(result, 11);
    }

    #[test]
    fn part_2() {
        let result = super::part_2("\
            3   4\n\
            4   3\n\
            2   5\n\
            1   3\n\
            3   9\n\
            3   3\n\
        ".trim());
        assert_eq!(result, 31);
    }
}
