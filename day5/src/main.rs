// TODO: part-2

#[derive(Debug)]
struct Map {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

impl Map {
    fn get_value_for(&self, n: u64) -> Option<u64> {
        let src_end = self.src_start + self.length - 1;
        if n >= self.src_start && n <= src_end {
            let dist = n - self.src_start;
            Some(self.dest_start + dist)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Maps<'a> {
    map_type: &'a str,
    maps: Vec<Map>,
}

impl<'a> Maps<'a> {
    fn push(&mut self, map: Map) {
        self.maps.push(map)
    }

    fn get_value_for(&self, n: u64) -> u64 {
        let mut r = None;
        for i in 0..self.maps.len() {
            if let Some(value) = self.maps[i].get_value_for(n) {
                r = Some(value);
                break;
            }
        }
        if r.is_some() {
            r.unwrap()
        } else {
            n
        }
    }
}

#[derive(Debug)]
struct Almanac<'a> {
    list_of_maps: Vec<Maps<'a>>,
}

impl<'a> Almanac<'a> {
    fn push(&mut self, maps: Maps<'a>) {
        self.list_of_maps.push(maps)
    }
}

fn extract_maps<'a>(map_type: &'a str, lines: &'a Vec<&'a str>) -> Maps<'a> {
    let index = lines
        .iter()
        .position(|line| line.starts_with(map_type))
        .expect(format!("{} not found", map_type).as_str())
        + 1;
    let mut maps = Maps {
        map_type: map_type,
        maps: vec![],
    };

    for i in index..lines.len() {
        if lines[i].len() == 0 {
            break;
        }
        let mut iter = lines[i].split(" ");
        let map = Map {
            dest_start: iter
                .next()
                .expect(format!("{} destination start not found", map_type).as_str())
                .parse::<u64>()
                .expect(format!("{} destination start not an integer", map_type).as_str()),
            src_start: iter
                .next()
                .expect(format!("{} source start not found", map_type).as_str())
                .parse::<u64>()
                .expect(format!("{} source start not an integer", map_type).as_str()),
            length: iter
                .next()
                .expect(format!("{} length not found", map_type).as_str())
                .parse::<u64>()
                .expect(format!("{} length not an integer", map_type).as_str()),
        };
        maps.push(map);
    }
    maps
}

fn extract_seeds(lines: &Vec<&str>) -> Vec<u64> {
    if let Some(seed_line) = lines[0].strip_prefix("seeds: ") {
        seed_line
            .split(" ")
            .map(|n| n.parse::<u64>().expect("seed number not found"))
            .collect()
    } else {
        panic!("seeds not found");
    }
}

fn main() {
    let lines: Vec<_> = include_str!("input1.txt").lines().collect();
    let mut almanac: Almanac = Almanac{ list_of_maps: vec![]};
    let seeds = extract_seeds(&lines);
    almanac.push(extract_maps("seed-to-soil map", &lines));
    almanac.push(extract_maps("soil-to-fertilizer map", &lines));
    almanac.push(extract_maps("fertilizer-to-water map", &lines));
    almanac.push(extract_maps("water-to-light map", &lines));
    almanac.push(extract_maps("light-to-temperature map", &lines));
    almanac.push(extract_maps("temperature-to-humidity map", &lines));
    almanac.push(extract_maps("humidity-to-location map", &lines));

    let part1 = seeds
        .iter()
        .map(|&seed| {
            almanac.list_of_maps.iter().fold(seed, |value, maps| {
                maps.get_value_for(value)
            })
        })
        .min();

    dbg!(part1);
}
