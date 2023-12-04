use regex::Regex;

struct Cube {
    color: String,
    count: u32,
}

struct Set {
    cubes: Vec<Cube>,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let game_id_regex = Regex::new(r"Game (\d+)").unwrap();
        let cube_regex = Regex::new(r"(\d+) (\w+)").unwrap();

        let sets: Vec<_> = line.split(": ").collect();
        let mut game = if let Some(captures) = game_id_regex.captures(sets[0]) {
            Game {
                id: captures[1].parse().unwrap(),
                sets: vec![],
            }
        } else {
            Game {
                id: 0,
                sets: vec![],
            }
        };
        for set in sets[1..].iter() {
            let mut set_vec = vec![];
            for sub_set in set.split("; ") {
                let mut s_set = Set { cubes: vec![] };
                s_set.cubes = sub_set
                    .split(", ")
                    .map(|cube| {
                        if let Some(captures) = cube_regex.captures(cube) {
                            Cube {
                                color: captures[2].parse().unwrap(),
                                count: captures[1].parse().unwrap(),
                            }
                        } else {
                            Cube {
                                color: "".to_string(),
                                count: 0,
                            }
                        }
                    })
                    .collect();
                set_vec.push(s_set);
            }
            game.sets = set_vec;
        }
        game
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|set| {
            set.cubes.iter().all(|cube| match cube.color.as_str() {
                "red" => cube.count <= 12,
                "green" => cube.count <= 13,
                "blue" => cube.count <= 14,
                _ => false,
            })
        })
    }

    fn max_by(&self, color: &str) -> u32 {
        self.sets
            .iter()
            .map(|set| {
                set.cubes
                    .iter()
                    .filter(|cube| cube.color == color)
                    .map(|cube| cube.count)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .max()
            .unwrap()
    }
}

fn main() {
    let part1 = include_str!("input.txt")
        .lines()
        .map(|line| Game::from(line))
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum::<u32>();
    dbg!(part1);

    let part2: u32 = include_str!("input.txt")
        .lines()
        .map(|line| Game::from(line))
        .map(|game| game.max_by("red") * game.max_by("blue") * game.max_by("green"))
        .sum();
    dbg!(part2);
}
