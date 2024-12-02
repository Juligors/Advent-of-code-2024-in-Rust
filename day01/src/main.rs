use std::collections::HashMap;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let (mut left_col, mut right_col): (Vec<i32>, Vec<i32>) = content
        .trim()
        .split('\n')
        .map(|line| {
            let mut numbers = line.trim().split_ascii_whitespace();
            let first: i32 = numbers.next().unwrap().parse().unwrap();
            let second: i32 = numbers.next().unwrap().parse().unwrap();

            (first, second)
        })
        .unzip();

    left_col.sort();
    right_col.sort();

    part_1(&left_col, &right_col);
    part_2(&left_col, &right_col);
}

fn part_1(left_col: &[i32], right_col: &[i32]) {
    let part_1_result: i32 = left_col
        .iter()
        .zip(right_col.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Part 1 result: {}", part_1_result);
}

fn part_2(left_col: &[i32], right_col: &[i32]) {
    let mut value_counts_right: HashMap<i32, i32> = HashMap::new();
    for num in right_col.iter() {
        *value_counts_right.entry(*num).or_default() += 1;
    }

    let part_2_result: i32 = left_col
        .iter()
        .map(|num| match value_counts_right.get(num) {
            Some(value) => num * value,
            None => 0,
        })
        .sum();

    println!("Part 2 result: {}", part_2_result);
}
