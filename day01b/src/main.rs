fn main() {

    let mut cals = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().filter_map(|c| c.parse::<i32>().ok()).sum::<i32>())
        .collect::<Vec<i32>>();

    cals.sort_unstable_by(|a, b| b.cmp(a));
    println!(
        "{}",
        cals.iter()
            .take(3)
            .sum::<i32>()
    )

}
