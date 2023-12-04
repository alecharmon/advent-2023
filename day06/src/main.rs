use regex::Regex;

type InputType = Vec<(u32, u32)>;

fn part1(input: InputType) -> u32 {
    let mut sum = 0;

    sum
}

fn part2(input: InputType) -> usize {
    0
}

fn parse() -> InputType {
    let numbers_pattern = Regex::new(r"(\d+)").unwrap();
    let lines: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let rhs: &str = line.split(':').collect::<Vec<&str>>()[1];
            numbers_pattern
                .captures_iter(line)
                .map(|m| m.get(1).unwrap().as_str().parse().unwrap())
                .collect()
        })
        .collect();
    lines[0]
        .iter()
        .map(|x| x.to_owned())
        .zip(lines[1].iter().map(|x| x.to_owned()))
        .collect()
}

fn main() {
    let r1 = part1(parse());
    /// The line `// let r2 = part2(parse());` is commented out, which means it is not being executed. It is a placeholder
    /// for calling the `part2` function with the parsed input and storing the result in the variable `r2`. Currently, it is
    // let r2 = part2(parse());
    // println!("Part 1: {r1}");
    // println!("Part 2: {r2}");
}
