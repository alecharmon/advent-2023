struct Graph {
    graph: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
}

impl Graph {
    fn new(graph: Vec<Vec<char>>) -> Self {
        let visited = graph
            .iter()
            .map(|line| line.iter().map(|_| false).collect())
            .collect();
        Self { graph, visited }
    }

    fn get(&mut self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 || y >= self.graph.len() as isize || x >= self.graph[0].len() as isize {
            None
        } else if !self.visited[y as usize][x as usize] {
            self.visited[y as usize][x as usize] = true;
            Some(self.graph[y as usize][x as usize])
        } else {
            None
        }
    }

    fn get_num(&mut self, x: isize, y: isize) -> Option<char> {
        self.get(x, y).filter(|c| c.is_numeric())
    }

    fn build_number(&mut self, x: isize, y: isize) -> Option<u32> {
        let num_str = self.get_num(x, y)?;
        let mut num_str = num_str.to_string();

        let mut left = x - 1;
        while let Some(c) = self.get_num(left, y) {
            num_str = c.to_string() + &num_str;
            left -= 1;
        }

        let mut right = x + 1;
        while let Some(c) = self.get_num(right, y) {
            num_str += &c.to_string();
            right += 1;
        }

        Some(num_str.parse().unwrap_or(0))
    }
}

fn part1(mut input: Graph) -> u32 {
    let mut sum = 0;
    for (y, line) in input.graph.to_vec().iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c != &'.' && !c.is_numeric() {
                for adj_x in x as isize - 1..=x as isize + 1 {
                    for adj_y in y as isize - 1..=y as isize + 1 {
                        if let Some(num) = input.build_number(adj_x, adj_y) {
                            sum += num;
                        }
                    }
                }
            }
        }
    }
    sum
}

fn part2(mut input: Graph) -> u32 {
    let mut sum = 0;
    for (y, line) in input.graph.to_vec().iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'*' {
                let nums: Vec<_> = (x as isize - 1..=x as isize + 1)
                    .flat_map(|adj_x| {
                        (y as isize - 1..=y as isize + 1).map(move |adj_y| (adj_x, adj_y))
                    })
                    .filter_map(|(adj_x, adj_y)| input.build_number(adj_x, adj_y))
                    .collect();

                if nums.len() == 2 {
                    sum += nums.iter().product::<u32>();
                }
            }
        }
    }
    sum
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let r1 = part1(Graph::new(input.clone()));
    let r2 = part2(Graph::new(input));
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
