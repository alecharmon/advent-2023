fn part1(input: Vec<Vec<u32>>) -> u32 {
    input.iter().map(|f| f.iter().sum()).max().unwrap()
}
fn part2(input: Vec<Vec<u32>>) -> u32 {
    let mut sums: Vec<u32> = input.iter().map(|f| f.iter().sum()).collect::<Vec<u32>>();

    sums.sort();
    sums.iter().rev().take(3).sum()
}

fn parse() -> Vec<Vec<u32>> {
    include_str!("../input.txt")
        .split("\n\n")
        .map(|f| f.split('\n').flat_map(|f| f.parse::<u32>()).collect())
        .collect()
}

fn main() {
    let r1 = part1(parse());
    let r2 = part2(parse());
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
