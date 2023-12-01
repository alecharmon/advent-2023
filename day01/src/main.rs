use lazy_static::lazy_static;
use std::collections::HashMap;

type InputType = Vec<&'static str>;

fn build(calibration_values: Vec<char>) -> u32 {
    let first = calibration_values.first().unwrap();
    let last = calibration_values.last().unwrap();
    format!("{first}{last}").parse::<u32>().unwrap()
}
fn part1(input: InputType) -> u32 {
    input
        .iter()
        .map(|f| f.chars().filter(|c| c.is_numeric()).collect())
        .map(build)
        .sum()
}

lazy_static! {
    static ref LOOKUP: HashMap<&'static str, char> = {
        let mut lookup = HashMap::new();
        lookup.insert("one", '1');
        lookup.insert("two", '2');
        lookup.insert("three", '3');
        lookup.insert("four", '4');
        lookup.insert("five", '5');
        lookup.insert("six", '6');
        lookup.insert("seven", '7');
        lookup.insert("eight", '8');
        lookup.insert("nine", '9');

        lookup
    };
}

fn scan(input: InputType) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|line| {
            let line_length = line.len();

            let mut res: Vec<char> = Vec::new();
            let mut i = 0;
            while i < line_length {
                let c = line.as_bytes()[i] as char;
                if c.is_numeric() {
                    res.push(c);
                    i += 1;
                    continue;
                }
                for (word, c) in LOOKUP.iter() {
                    if word.len() + i <= line_length
                        && line.get(i..i + word.len()).unwrap_or("") == *word
                    {
                        res.push(c.to_owned());
                        i += 1;
                        continue;
                    }
                }
                i += 1;
            }
            res
        })
        .collect()
}

fn part2(input: InputType) -> u32 {
    scan(input)
        .to_owned()
        .iter()
        .map(|v| v.to_owned())
        .map(build)
        .sum()
}

fn parse() -> InputType {
    include_str!("../input.txt").lines().collect()
}

fn main() {
    let r1 = part1(parse());
    let r2 = part2(parse());
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
