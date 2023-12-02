use regex::Regex;
use std::collections::HashMap;

type InputType = Vec<Game>;

#[derive(Debug, Clone, Copy)]
enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl Cube {
    fn color(&self) -> &str {
        match self {
            Cube::Red(_) => "red",
            Cube::Blue(_) => "blue",
            Cube::Green(_) => "green",
        }
    }
    fn count(&self) -> u32 {
        match self {
            Cube::Red(x) => x.to_owned(),
            Cube::Blue(x) => x.to_owned(),
            Cube::Green(x) => x.to_owned(),
        }
    }

    fn follows_constraint(&self, constraint: Self) -> bool {
        if self.color() != constraint.color() {
            return true;
        }
        self.count() <= constraint.count()
    }

    fn from_color(color: &str, count: u32) -> Self {
        match color {
            "red" => Cube::Red(count),
            "blue" => Cube::Blue(count),
            "green" => Cube::Green(count),
            _ => panic!("invalid color, {color}"),
        }
    }
}
type CubeSet = Vec<Cube>;

#[derive(Debug, Clone)]
struct Game {
    sets: Vec<CubeSet>,
    pub id: u32,
}

impl Game {
    fn new(sets: Vec<CubeSet>, id: u32) -> Self {
        Self { sets, id }
    }

    fn is_valid(&self, constraints: CubeSet) -> bool {
        for set in self.sets.iter().cloned() {
            for cube in set {
                for constraint in constraints.iter().copied() {
                    if !cube.follows_constraint(constraint) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn max_enum(&self) -> CubeSet {
        let mut maxes: HashMap<_, u32> = [("green", 0), ("red", 0), ("blue", 0)]
            .iter()
            .cloned()
            .collect();

        for key in maxes.clone().keys() {
            for set in self.sets.iter().cloned() {
                for cube in set {
                    if cube.color() == *key && *maxes.get(cube.color()).unwrap() < cube.count() {
                        maxes.insert(key, cube.count());
                    }
                }
            }
        }

        let maxes_counts = maxes
            .iter()
            .map(|(key, value)| Cube::from_color(key, value.to_owned()))
            .collect();

        maxes_counts
    }

    fn power(&self) -> u32 {
        let mut product = 1;
        self.max_enum().iter().for_each(|cube| {
            if cube.count() > 0 {
                product *= cube.count();
            }
        });
        product
    }
}

fn part1(input: InputType, constraints: CubeSet) -> u32 {
    let mut sum = 0;
    input.iter().for_each(|game| {
        if game.is_valid(constraints.to_vec()) {
            sum += game.id
        }
    });
    sum
}

fn part2(input: InputType) -> u32 {
    let mut sum = 0;
    input.iter().for_each(|game| {
        sum += game.power();
    });
    sum
}

fn parse() -> InputType {
    let set_pattern = Regex::new(r"(\d+)\s*(blue|red|green)").unwrap();
    let id_pattern = Regex::new(r"Game (\d+)").unwrap();
    include_str!("../input.txt")
        .lines()
        .map(|game| {
            let id = id_pattern
                .captures(game)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let sets = game
                .split(';')
                .map(|set_str| {
                    let mut results = CubeSet::new();
                    set_pattern
                        .captures_iter(set_str)
                        .map(|capture| match capture[2].as_ref() {
                            "red" => Cube::Red(capture[1].parse::<u32>().unwrap()),
                            "blue" => Cube::Blue(capture[1].parse::<u32>().unwrap()),
                            "green" => Cube::Green(capture[1].parse::<u32>().unwrap()),
                            _ => panic!("unexpected color"),
                        })
                        .for_each(|cube| results.push(cube));
                    results
                })
                .collect::<Vec<Vec<Cube>>>();
            Game::new(sets, id)
        })
        .collect()
}

fn main() {
    let r1 = part1(
        parse(),
        vec![Cube::Red(12), Cube::Blue(14), Cube::Green(13)],
    );
    let r2 = part2(parse());
    println!("Part 1: {}", r1);
    println!("Part 2: {r2:?}");
}
