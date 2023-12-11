// TODO: Part-2 is incorrect

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use core::cmp::Ordering;

lazy_static! {
    static ref CARDS: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('2', 0);
        m.insert('3', 1);
        m.insert('4', 2);
        m.insert('5', 3);
        m.insert('6', 4);
        m.insert('7', 5);
        m.insert('8', 6);
        m.insert('9', 7);
        m.insert('T', 8);
        m.insert('J', 9);
        m.insert('Q', 10);
        m.insert('K', 11);
        m.insert('A', 12);
        m
    };

    static ref CARDS_PART2: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('J', 0);
        m.insert('2', 1);
        m.insert('3', 2);
        m.insert('4', 3);
        m.insert('5', 4);
        m.insert('6', 5);
        m.insert('7', 6);
        m.insert('8', 7);
        m.insert('9', 8);
        m.insert('T', 9);
        m.insert('Q', 10);
        m.insert('K', 11);
        m.insert('A', 12);
        m
    };
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u64,
}

impl<'a> Hand<'a> {
    fn hand_type(&self) -> HandType {
        let mut h: HashMap<&u8, u64> = HashMap::new();
        for c in self.cards.as_bytes() {
            h.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut values: Vec<_> = h.values().collect();
        values.sort();

        if values == [&5_u64] {
            HandType::FiveOfAKind
        } else if values == [&1_u64, &4_u64] {
            HandType::FourOfAKind
        } else if values == [&2_u64, &3_u64] {
            HandType::FullHouse
        } else if values == [&1_u64, &1_u64, &3_u64] {
            HandType::ThreeOfAKind
        } else if values == [&1_u64, &2_u64, &2_u64] {
            HandType::TwoPair
        } else if values == [&1_u64, &1_u64, &1_u64, &2_u64] {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn hand_type_part2(&self) -> HandType {
        let mut h: HashMap<&u8, u64> = HashMap::new();
        let mut j_count = 0;
        for c in self.cards.as_bytes() {
            if c == &b'J' {
                j_count += 1;
            } else {
                h.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        let mut values: Vec<u64> = h.values().map(|r| *r).collect::<Vec<_>>();
        values.sort();
        let v_len = values.len();
        if v_len == 0{
            values = vec![5];
        } else{
            values[v_len-1] += j_count;
        }

        if values == [5_u64] {
            HandType::FiveOfAKind
        } else if values == [1_u64, 4_u64] {
            HandType::FourOfAKind
        } else if values == [2_u64, 3_u64] {
            HandType::FullHouse
        } else if values == [1_u64, 1_u64, 3_u64] {
            HandType::ThreeOfAKind
        } else if values == [1_u64, 2_u64, 2_u64] {
            HandType::TwoPair
        } else if values == [1_u64, 1_u64, 1_u64, 2_u64] {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn rank(&self) -> Vec<u8> {
        self.cards.chars().map(|c| {
            CARDS[&c]
        }).collect()
    }

    fn rank_part2(&self) -> Vec<u8> {
        self.cards.chars().map(|c| {
            CARDS_PART2[&c]
        }).collect()
    }

    fn lte(&self, other: &Hand<'a>) -> bool {
        let type_order = self.type_order(other);
        let rank_order = self.rank_order(other);

        if type_order == Ordering::Less {
            true
        } else if type_order == Ordering::Equal && rank_order == Ordering::Less || rank_order == Ordering::Equal{
            true
        } else {
            false
        }
    }

    fn lte2(&self, other: &Hand<'a>) -> bool {
        let type_order = self.type_order_part2(other);
        let rank_order = self.rank_order_part2(other);

        if type_order == Ordering::Less {
            true
        } else if type_order == Ordering::Equal && (rank_order == Ordering::Less || rank_order == Ordering::Equal) {
            true
        } else {
            false
        }
    }

    fn type_order(&self, other: &Hand<'a>) -> Ordering {
        if self.hand_type() < other.hand_type() {
            Ordering::Less
        } else if self.hand_type() > other.hand_type() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn type_order_part2(&self, other: &Hand<'a>) -> Ordering {
        if self.hand_type_part2() < other.hand_type_part2() {
            Ordering::Less
        } else if self.hand_type_part2() > other.hand_type_part2() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn rank_order(&self, other: &Hand<'a>) -> Ordering {
        let s_rank = self.rank();
        let o_rank = other.rank();

        let diff_orders: Vec<_> = s_rank.iter().zip(o_rank.iter()).filter(|(s, o)|{
            s != o
        }).collect();
        let (s, o) = diff_orders[0];
        if s < o {
            Ordering::Less
        } else if s > o {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn rank_order_part2(&self, other: &Hand<'a>) -> Ordering {
        let s_rank = self.rank_part2();
        let o_rank = other.rank_part2();

        let diff_orders: Vec<_> = s_rank.iter().zip(o_rank.iter()).filter(|(s, o)|{
            s != o
        }).collect();
        let (s, o) = diff_orders[0];
        if s < o {
            Ordering::Less
        } else if s > o {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(line: &'a str) -> Hand {
        let mut split = line.split_whitespace();
        Hand {
            cards: split.next().expect(format!("Cards not found in {}", line).as_str()),
            bid: split.next().expect(format!("Bid not found in {}", line).as_str()).parse::<u64>().expect(format!("Invalid bid in {}", line).as_str())
        }
    }
}

fn quicksort(arr: &mut [Hand], part: &str) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr, part);
    quicksort(&mut arr[0..pivot_index], part);
    quicksort(&mut arr[pivot_index + 1..], part);
}

fn partition(arr: &mut [Hand], part: &str) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);

    let mut i = 0;
    for j in 0..arr.len() - 1 {
        let lte = if part == "part1" {
            arr[j].lte(&arr[arr.len() - 1])
        } else {
            arr[j].lte2(&arr[arr.len() - 1])
        };
        if lte {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}

fn main() {
    let mut hands: Vec<_> = include_str!("input.txt").lines().map(|line| {
        Hand::from(line)
    }).collect();
    quicksort(&mut hands, "part1");
    let part1: u64 = hands.iter().enumerate().map(|(i, hand)| {
        let pos = (i as u64)+1;
        pos * hand.bid
    }).sum();
    dbg!(part1);

    quicksort(&mut hands, "part2");
    let part2: u64 = hands.iter().enumerate().map(|(i, hand)| {
        let pos = (i as u64)+1;
        pos * hand.bid
    }).sum();
    dbg!(part2);
}
