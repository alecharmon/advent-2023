use regex::Regex;
use std::collections::{HashMap, HashSet};

type InputType = Vec<Vec<HashSet<u32>>>;

fn part1(input: InputType) -> u32 {
    let mut sum = 0;
    for card in input {
        let (winning, mine) = (&card[0], &card[1]);
        let winning_count = winning.intersection(mine).count();
        if winning_count > 0 {
            sum += (2_u32).pow(winning_count as u32 - 1)
        }
    }
    sum
}

fn part2(input: InputType) -> usize {
    let mut lookup: HashMap<usize, usize> = HashMap::new();

    for (idx, _card) in input.iter().enumerate() {
        lookup.insert(idx + 1, 1);
    }

    for (idx, card) in input.iter().enumerate() {
        let (winning, mine) = (&card[0], &card[1]);
        let winning_count = winning.intersection(mine).count();
        if winning_count > 0 {
            let card_id = idx + 1;
            for cards_won in idx + 2..(idx + winning.intersection(mine).count() + 2) {
                lookup.insert(
                    cards_won,
                    *lookup.get(&cards_won).unwrap() + lookup.get(&card_id).unwrap(),
                );
            }
        }
    }
    lookup.values().sum()
}

fn parse() -> InputType {
    let numbers_pattern = Regex::new(r"(\d+)").unwrap();
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let no_game: &str = line.split(':').collect::<Vec<&str>>()[1];
            no_game
                .split('|')
                .map(|set| {
                    let numbers_itr: Vec<u32> = numbers_pattern
                        .captures_iter(set)
                        .map(|m| m.get(1).unwrap().as_str().parse().unwrap())
                        .collect();
                    HashSet::from_iter(numbers_itr)
                })
                .collect()
        })
        .collect()
}

fn main() {
    let r1 = part1(parse());
    let r2 = part2(parse());
    println!("Part 1: {r1}");
    println!("Part 2: {r2}");
}
