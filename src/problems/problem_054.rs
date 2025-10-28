use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[test]
fn problem_054() -> Result<(), Box<dyn std::error::Error>> {
    let result: Vec<(Hand, Hand)> = read_to_string("src/problems/data/0054_poker.txt")?
        .lines()
        .map(|line| line.split(" ").map(|x| Card::parse(x)).collect::<Vec<_>>())
        .map(|cards| {
            (
                Hand {
                    cards: cards[0..5].to_vec(),
                },
                Hand {
                    cards: cards[5..].to_vec(),
                },
            )
        })
        .filter(|(one, two)| one.winner(two))
        .collect();
    println!("{}", result.len());
    Ok(())
}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn winner(&self, other: &Hand) -> bool {
        match (self.royal_flush(), other.royal_flush()) {
            (true, false) => return true,
            (false, true) => return false,
            (true, true) => return false,
            _ => ()
        }

        match (self.straight_flush(), other.straight_flush()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        match (self.four_of_a_kind(), other.four_of_a_kind()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        match (self.full_house(), other.full_house()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        match (self.flush(), other.flush()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        match (self.straight(), other.straight()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        match (self.three_of_a_kind(), other.three_of_a_kind()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        match (self.two_pair(), other.two_pair()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        match (self.pair(), other.pair()) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c1), Some(c2)) => return c1 > c2,
            _ => ()
        }

        self.high_card().rank > other.high_card().rank
    }

    fn ranks(&self) -> Vec<u8> {
        self.cards.iter().map(|x| x.rank).collect()
    }

    fn rank_counts(&self) -> HashMap<u8, u8> {
        let mut counts = HashMap::new();
        for card in &self.cards {
            *counts.entry(card.rank).or_insert(0) += 1;
        }
        counts
    }

    fn royal_flush(&self) -> bool {
        let ranks = self.ranks();
        ranks.contains(&10)
            && ranks.contains(&11)
            && ranks.contains(&12)
            && ranks.contains(&13)
            && ranks.contains(&14)
            && self.flush().is_some()
    }

    fn straight_flush(&self) -> Option<u8> {
        if self.straight().is_some() && self.flush().is_some() {
            Some(self.high_card().rank)
        } else {
            None
        }
    }

    fn four_of_a_kind(&self) -> Option<u8> {
        self.rank_counts().iter().filter(|(_, count)| **count == 4).map(|(card, _)| *card).next()
    }

    fn full_house(&self) -> Option<u8> {
        if let (Some(three_rank), Some(_)) = (self.three_of_a_kind(), self.pair()) {
            Some(three_rank)
        } else {
            None
        }
    }

    fn three_of_a_kind(&self) -> Option<u8> {
        self.rank_counts()
            .iter()
            .filter(|(_, count)| **count == 3)
            .map(|(card, _)| *card)
            .next()
    }

    fn two_pair(&self) -> Option<(u8, u8)> {
        let mut pairs = self.rank_counts()
            .iter()
            .filter(|(_, count)| **count == 2)
            .map(|(rank, _)| *rank)
            .collect::<Vec<u8>>();
        pairs.sort();
        if pairs.len() == 2 {
            Some((pairs[1], pairs[0]))
        } else {
            None
        }
    }

    fn pair(&self) -> Option<u8> {
        self.rank_counts()
            .iter()
            .filter(|(_, count)| **count == 2)
            .map(|(rank, _)| *rank)
            .next()
    }

    fn flush(&self) -> Option<u8> {
        let suit = self.cards[0].suit;
        if self.cards.iter().map(|x| x.suit).all(|x| x == suit) {
            Some(self.high_card().rank)
        } else {
            None
        }
    }

    fn straight(&self) -> Option<u8> {
        let mut ranks = self.ranks();
        ranks.sort();
        ranks.dedup();
        if ranks.len() == 5 && ranks[4] - ranks[0] == 4 {
            Some(ranks[4])
        } else {
            None
        }
    }

    fn high_card(&self) -> Card {
        self.cards()[4].clone()
    }

    fn cards(&self) -> Vec<Card> {
        let mut new = self.cards.clone();
        new.sort_by_key(|card| card.rank);
        new
    }
}

#[derive(Clone, Debug)]
struct Card {
    rank: u8,
    suit: char,
}

impl Card {
    fn parse(s: &str) -> Self {
        let rank_char = s.chars().nth(0).unwrap();
        let suit_char = s.chars().nth(1).unwrap();
        let rank = match rank_char {
            '2'..='9' => rank_char as u8 - '0' as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("{}", rank_char),
        };
        Self {
            rank,
            suit: suit_char,
        }
    }
}
