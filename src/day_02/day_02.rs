use std::fs::read_to_string;

fn main() {
    let text_file_string = read_to_string("./src/day_02/input.txt").unwrap();
    let part_1_result = part_1(&text_file_string);
    // expect 479
    println!("Part 1: {part_1_result}");
    let part_2_result = part_2(&text_file_string);
    // expect 531
    println!("Part 2: {part_2_result}");
}

fn process_rows(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}
fn make_diff_for_row(row: Vec<i32>) -> Vec<i32> {
    row.windows(2).map(|a| a[0] - a[1]).collect()
}
fn judge_row(row: &&Vec<i32>) -> bool {
    let diff = make_diff_for_row(row.to_vec());
    for i in 1..diff.len() {
        let a = diff[i - 1];
        let b = diff[i];
        let sign_a = a.signum();
        let sign_b = b.signum();
        let abs_a = a.abs();
        let abs_b = b.abs();
        if abs_a >= 4 || abs_b >= 4 || abs_a == 0 || abs_b == 0 || sign_a != sign_b {
            return false;
        }
    }
    true
}
fn part_1(input: &str) -> u32 {
    let rows = process_rows(input);
    println!(
        "Part 1 first line: {}",
        rows.first()
            .unwrap()
            .iter()
            .map(i32::to_string)
            .collect::<Vec<String>>()
            .join(",")
    );
    rows.iter().filter(judge_row).count() as u32
}

fn part_2(input: &str) -> u32 {
    let rows = process_rows(input);
    rows.iter()
        .filter(|row| match judge_row(row) {
            true => true,
            false => {
                let mut pass = false;
                for i in 0..row.len() {
                    let mut attempt = (*row).clone();
                    attempt.remove(i);
                    if judge_row(&&attempt) {
                        pass = true;
                    }
                }
                pass
            }
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
7 6 4 2 1\n\
1 2 7 8 9\n\
9 7 6 2 1\n\
1 3 2 4 5\n\
8 6 4 4 1\n\
1 3 6 7 9\n\
";
    #[test]
    fn part_1() {
        let result = super::part_1(SAMPLE_INPUT.trim());
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2() {
        let result = super::part_2(SAMPLE_INPUT.trim());
        assert_eq!(result, 4);
    }
}
