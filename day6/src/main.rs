#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_beat(&self) -> u64 {
        let mut hold_time = 0;
        let mut result = 0;
        while hold_time <= self.time {
            let run_time = self.time - hold_time;
            let speed = hold_time;
            if speed * run_time > self.distance {
                result += 1;
            }
            hold_time += 1;
        }
        result
    }
}

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl Races {
    fn ways_to_beat_product(&self) -> u64 {
        self.races.iter().fold(1, |product, race| {
            product * race.ways_to_beat()
        })
    }

    fn collate_races(&self) -> Races {
        let mut race = Race {time: 0, distance: 0};
        let mut new_race = self.races.iter().fold(race, |mut result, race| {
            let mut collated_time = result.time.to_string();
            collated_time.push_str(race.time.to_string().as_str());
            result.time = collated_time.parse::<u64>().expect("Collate races: invalid time");

            let mut collated_distance = result.distance.to_string();
            collated_distance.push_str(race.distance.to_string().as_str());
            result.distance = collated_distance.parse::<u64>().expect("Collate races: invalid distance");
            result
        });
        Races {races: vec![new_race]}
    }
}

impl From<&Vec<&str>> for Races {
    fn from(lines: &Vec<&str>) -> Self {
        let times = lines[0].strip_prefix("Time: ").expect("Times not found").split_whitespace();
        let distances = lines[1].strip_prefix("Distance: ").expect("Distances not found").split_whitespace();
        let races = times.zip(distances).map(|(time, distance)| {
            Race {time: time.parse::<u64>().expect("Time expected u64"), distance: distance.parse::<u64>().expect("Distance expected u64")}
        }).collect();
        Races {races: races}
    }
}

fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let races = Races::from(&lines);
    let part1 = races.ways_to_beat_product();
    dbg!(part1);

    let part2 = races.collate_races().ways_to_beat_product()
    dbg!(part2);
}
