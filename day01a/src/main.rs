fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|e| e.lines().filter_map(|c| c.parse::<i32>().ok()).sum::<i32>())
            .max()
            .unwrap()
        );
}
