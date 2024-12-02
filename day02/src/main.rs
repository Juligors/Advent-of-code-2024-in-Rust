fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let reports: Vec<Vec<i32>> = content
        .trim()
        .split('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    part_1(&reports);
    part_2(&reports);
}

fn part_1(reports: &[Vec<i32>]) {
    let mut safe_reports: Vec<bool> = Vec::with_capacity(reports.len());

    for report in reports.iter() {
        let diff = report[0] - report[1];
        let should_be_increasing = diff > 0;

        let levels_are_safe = report.windows(2).all(|window| {
            let diff = window[0] - window[1];

            let is_monotonic = (diff > 0) == should_be_increasing;
            let is_diff_in_range = (-3..=-1).contains(&diff) || (1..=3).contains(&diff);

            is_monotonic && is_diff_in_range
        });

        safe_reports.push(levels_are_safe);
    }

    let part_1_result: usize = safe_reports.iter().filter(|b| **b).count();
    println!("Part 1 result: {}", part_1_result);
}

fn part_2(reports: &[Vec<i32>]) {
    let mut safe_reports: Vec<bool> = Vec::with_capacity(reports.len());

    'outer: for report in reports.iter() {
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);

            let diff = report[0] - report[1];
            let should_be_increasing = diff > 0;

            let levels_are_safe = report.windows(2).all(|window| {
                let diff = window[0] - window[1];

                let is_monotonic = (diff > 0) == should_be_increasing;
                let is_diff_in_range = (-3..=-1).contains(&diff) || (1..=3).contains(&diff);

                is_monotonic && is_diff_in_range
            });

            if levels_are_safe {
                safe_reports.push(true);
                continue 'outer;
            }
        }

        safe_reports.push(false);
    }

    let part_2_result: usize = safe_reports.iter().filter(|b| **b).count();
    println!("Part 2 result: {}", part_2_result);
}
