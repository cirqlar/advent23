use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn map_card_to_num_value(ch: char) -> u8 {
    match ch {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        x => x.to_digit(10).expect("is digit") as u8,
    }
}

trait HashCounts<V> {
    fn contains(&self, value: V) -> bool;
}

impl<K, V: PartialEq> HashCounts<V> for HashMap<K, V> {
    fn contains(&self, value: V) -> bool {
        self.values().any(|v| *v == value)
    }
}

fn eval_hand_rank(hand: &Vec<u8>) -> u8 {
    let mut counts = HashMap::new();
    for card in hand {
        counts.entry(*card).and_modify(|e| *e += 1).or_insert(1);
    }
    let joker_count = *counts.get(&1).unwrap_or(&0);
    if counts.len() == 1 {
        return 7; // 5
    }
    if counts.len() == 2 {
        if joker_count > 0 {
            return 7;
        }
        if counts.contains(4) {
            return 6; // 4 1
        }
        return 5; // 3 2
    }
    if counts.len() == 3 {
        if counts.contains(3) && joker_count > 0 {
            return 6;
        }
        if counts.contains(3) {
            return 4; // 3 1 1
        }

        if joker_count == 2 {
            return 6;
        }
        if joker_count == 1 {
            return 5;
        }
        return 3; // 2 2 1
    }
    if counts.len() == 4 {
        if joker_count > 0 {
            return 4;
        }
        return 2; // 2 1 1 1
    }
    if joker_count > 0 {
        return 2;
    }

    1
}

fn process(input: &str) -> usize {
    let mut mapped = input
        .lines()
        .map(|line| {
            let mut line = line.split(' ');
            let hand = line
                .next()
                .expect("a hand")
                .chars()
                .map(map_card_to_num_value)
                .collect::<Vec<_>>();
            let bid = line
                .next()
                .expect("a bid")
                .parse::<u16>()
                .expect("valid bid");
            let rank = eval_hand_rank(&hand);
            (hand, bid, rank)
        })
        .collect::<Vec<_>>();
    mapped.sort_by(|a, b| {
        let rank_order = a.2.cmp(&b.2);
        if rank_order != Ordering::Equal {
            return rank_order;
        }

        for (c, d) in a.0.iter().zip(b.0.iter()) {
            let rank_order = c.cmp(d);
            if rank_order != Ordering::Equal {
                return rank_order;
            }
        }
        println!("We have equal stuff {:?}, {:?}", a, b);
        Ordering::Equal
    });
    mapped
        .iter()
        .inspect(|a| {
            if cfg!(test) {
                println!("Debug: {:?}", a);
            }
        })
        .enumerate()
        .fold(0, |acc, (index, (_, value, ..))| {
            acc + (*value as usize) * (index + 1)
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d07_part_2_is_correct() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(process(input), 5905);
    }
}
