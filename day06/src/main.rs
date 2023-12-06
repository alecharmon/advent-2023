use regex::Regex;

type InputType = Vec<(u64, u64)>;

fn part1(input: InputType) -> u64 {
    let res: Vec<u64> = input
        .iter()
        .map(|(time, distance)| {
            let mut count: u64 = 0;
            for amount_held in 0..time.to_owned() {
                if (amount_held * (time.to_owned() - amount_held)) > *distance {
                    count += 1;
                }
            }
            count
        })
        .collect();
    res.iter().map(|f| f.to_owned()).product()
}

fn part2(input: InputType) -> u64 {
    // truly i ended up just deleting the white space but figured that was dishonest
    let mut time_str = String::new();
    let mut distance_str = String::new();
    input.iter().for_each(|(time, distance)| {
        time_str += time.to_string().as_str();
        distance_str += distance.to_string().as_str();
    });

    part1(vec![(
        time_str.parse().unwrap(),
        distance_str.parse().unwrap(),
    )])
}

fn parse() -> InputType {
    let numbers_pattern = Regex::new(r"(\d+)").unwrap();
    let lines: Vec<Vec<u64>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
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
    let r2 = part2(parse());
    println!("Part 1: {r1}");
    println!("Part 2: {r2}");
}
