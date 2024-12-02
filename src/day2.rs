pub fn solve() {
    let input: Vec<Vec<i64>> = include_str!("../input/day2")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[Vec<i64>]) -> usize {
    input.iter().filter(|report| is_safe(report)).count()
}

fn part2(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|report| is_safe(report) || is_safe_with_removal(report))
        .count()
}

fn is_safe(report: &[i64]) -> bool {
    let diffs: Vec<i64> = report.windows(2).map(|w| w[1] - w[0]).collect();
    let is_safe_increasing = diffs.iter().all(|&n| n >= 1 && n <= 3);
    let is_safe_decreasing = diffs.iter().all(|&n| n >= -3 && n <= -1);
    is_safe_increasing || is_safe_decreasing
}

fn is_safe_with_removal(report: &[i64]) -> bool {
    (0..report.len()).any(|idx| {
        let mut new_report = report.to_vec();
        new_report.remove(idx);
        is_safe(&new_report)
    })
}
