#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn is_above(&self, other: &Self) -> bool {
        self.x < other.x
    }

    fn is_below(&self, other: &Self) -> bool {
        self.x > other.x
    }

    fn is_left_of(&self, other: &Self) -> bool {
        self.y < other.y
    }

    fn is_right_of(&self, other: &Self) -> bool {
        self.y > other.y
    }
}

fn is_row_empty(x: usize, universe: &Vec<&[u8]>) -> bool {
    universe[x].iter().all({ |element| element == &b'.' })
}

fn is_column_empty(y: usize, universe: &Vec<&[u8]>) -> bool {
    for x in 0..universe.len() {
        if universe[x][y] != b'.' {
            return false;
        }
    }
    true
}

fn get_distance(p1: &Position, p2: &Position, universe: &Vec<&[u8]>, scale: u64) -> u64 {
    let mut steps = 0;
    let mut previous = Position { x: p1.x, y: p1.y };
    let mut current = Position { x: p1.x, y: p1.y };
    while current.x != p2.x || current.y != p2.y {
        if current.is_above(p2) {
            current.x += 1;
        } else if current.is_below(p2) {
            current.x -= 1;
        } else if current.is_left_of(p2) {
            current.y += 1;
        } else if current.is_right_of(p2) {
            current.y -= 1;
        }
        if previous.x != current.x && is_row_empty(previous.x, universe) {
            steps += scale;
        } else if previous.y != current.y && is_column_empty(previous.y, universe) {
            steps += scale;
        } else {
            steps += 1;
        }
        previous.x = current.x;
        previous.y = current.y;
    }
    steps
}

fn main() {
    let lines: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect();
    let mut galaxies: Vec<Position> = vec![];
    for (x, line) in lines.iter().enumerate() {
        for (y, el) in line.iter().enumerate() {
            if el == &b'#' {
                galaxies.push(Position { x, y });
            }
        }
    }
    let total_galaxies = galaxies.len();

    // sum of shortest paths
    let mut part1 = 0;
    for i in 0..total_galaxies {
        for j in i + 1..total_galaxies {
            part1 += get_distance(&galaxies[i], &galaxies[j], &lines, 2)
        }
    }
    dbg!(part1);

    // sum of shortest paths
    let mut part2 = 0;
    for i in 0..total_galaxies {
        for j in i + 1..total_galaxies {
            part2 += get_distance(&galaxies[i], &galaxies[j], &lines, 1000000)
        }
    }
    dbg!(part2);
}
