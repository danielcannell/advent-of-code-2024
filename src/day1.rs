pub fn solve() {
    let input: Vec<(i64, i64)> = include_str!("../input/day1")
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(" ").unwrap();
            (
                first.trim().parse().unwrap(),
                second.trim().parse().unwrap(),
            )
        })
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[(i64, i64)]) -> i64 {
    let mut list1: Vec<i64> = input.iter().map(|&(a, _)| a).collect();
    let mut list2: Vec<i64> = input.iter().map(|&(_, b)| b).collect();

    list1.sort();
    list2.sort();

    let mut result = 0;

    for (n, m) in list1.into_iter().zip(list2) {
        result += (n - m).abs();
    }

    result
}

fn part2(input: &[(i64, i64)]) -> i64 {
    let list1: Vec<i64> = input.iter().map(|&(a, _)| a).collect();
    let list2: Vec<i64> = input.iter().map(|&(_, b)| b).collect();

    let mut result = 0;

    for n in list1 {
        let count = list2.iter().filter(|&&m| n == m).count() as i64;
        result += n * count;
    }

    result
}
