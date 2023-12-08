// TODO: Memoize computation of scratch_cards.

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: u64,
    winning_numbers: HashSet<u64>,
    numbers: HashSet<u64>,
}

impl Card {
    fn points(&self) -> u64 {
        let winning_num_count = self.winning_numbers.intersection(&self.numbers).count() as u32;
        match winning_num_count {
            0 => 0,
            _ => 2_u64.pow(winning_num_count - 1),
        }
    }

    fn matching_cards_count(&self) -> u64 {
        self.winning_numbers.intersection(&self.numbers).count() as u64
    }

    fn winning_cards(&self, card_map: &HashMap<u64, &Card>) -> Vec<u64> {
        let upto = self.matching_cards_count();
        let mut result = vec![];
        for card_id in 1..=upto {
            if let Some(card) = card_map.get(&(self.id + card_id)) {
                let mut r = card.scratch_cards(&card_map);
                result.append(&mut r);
            }
        }
        result
    }

    fn scratch_cards(&self, card_map: &HashMap<u64, &Card>) -> Vec<u64> {
        let upto = self.matching_cards_count();
        let mut result = vec![];
        for card_id in 0..=upto {
            if card_map.contains_key(&(self.id + card_id)) {
                result.push(self.id + card_id);
            }
        }

        for card_id in 1..=upto {
            if let Some(card) = card_map.get(&(self.id + card_id)) {
                let mut r = card.winning_cards(&card_map);
                result.append(&mut r);
            }
        }

        result
    }
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let mut parts = line.split(": ");
        let (identity, nums) = (parts.next().unwrap(), parts.next().unwrap());
        let mut parts_of_nums = nums.split(" | ");

        Self {
            id: identity
                .strip_prefix("Card ")
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap(),
            winning_numbers: parts_of_nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect(),
            numbers: parts_of_nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect(),
        }
    }
}

fn main() {
    let cards: Vec<Card> = include_str!("input.txt")
        .lines()
        .map(|line| Card::from(line))
        .collect();

    let part1: u64 = cards.iter().map(|card| card.points()).sum();
    dbg!(part1);

    let card_map = cards.iter().fold(HashMap::new(), |mut map, card| {
        map.insert(card.id, card);
        map
    });

    let scratch_cards: Vec<u64> = cards
        .iter()
        .map(|card| card.scratch_cards(&card_map))
        .flatten()
        .collect();
    let part2 = scratch_cards.len();
    dbg!(part2);
}
