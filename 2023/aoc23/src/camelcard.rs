use std::collections::HashMap;
use std::fmt::Display;

use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::{character::complete::alphanumeric1, IResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Card {
    J,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    T,
    Q,
    K,
    A,
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TWO => write!(f, "2")?,
            Self::THREE => write!(f, "3")?,
            Self::FOUR => write!(f, "4")?,
            Self::FIVE => write!(f, "5")?,
            Self::SIX => write!(f, "6")?,
            Self::SEVEN => write!(f, "7")?,
            Self::EIGHT => write!(f, "8")?,
            Self::NINE => write!(f, "9")?,
            Self::T => write!(f, "T")?,
            Self::J => write!(f, "J")?,
            Self::Q => write!(f, "Q")?,
            Self::K => write!(f, "K")?,
            Self::A => write!(f, "A")?,
        }

        Ok(())
    }
}

impl Card {
    #[inline]
    pub fn from(c: char) -> Self {
        match c {
            '2' => Self::TWO,
            '3' => Self::THREE,
            '4' => Self::FOUR,
            '5' => Self::FIVE,
            '6' => Self::SIX,
            '7' => Self::SEVEN,
            '8' => Self::EIGHT,
            '9' => Self::NINE,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("non recognasable value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    HIGHCARD,
    ONEPAIR,
    TWOPAIR,
    THREEKIND,
    FULLHOUSE,
    FOURKIND,
    FIVEKIND,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub bidding: i64,
    cards: Vec<Card>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}{}{}{}{}]",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4]
        )?;

        Ok(())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_rank = self.rank();
        let other_rank = other.rank();

        if my_rank < other_rank {
            return Some(std::cmp::Ordering::Less);
        } else if my_rank > other_rank {
            return Some(std::cmp::Ordering::Greater);
        } else {
            // now we need to compare the values
            for i in 0..5 {
                if self.cards[i] < other.cards[i] {
                    return Some(std::cmp::Ordering::Less);
                } else if self.cards[i] > other.cards[i] {
                    return Some(std::cmp::Ordering::Greater);
                }
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    #[inline]
    pub fn new(line: &str) -> Self {
        Self::parse(&line).unwrap().1
    }

    pub fn rank(&self) -> Rank {
        // first we create the map
        let mut counts: HashMap<Card, usize> = HashMap::new();
        let mut jokers = 0;

        // the jokers need to be counted apart
        for card in self.cards.iter() {
            if card != &Card::J {
                *counts.entry(card.clone()).or_default() += 1;
            } else {
                jokers += 1;
            }
        }

        let mut counts: Vec<usize> = counts.iter().map(|(_, v)| *v).collect();
        counts.sort();
        // checking for five kind

        // now we add the jokers at the end to increaes the most existent ones
        if jokers == 5 {
            return Rank::FIVEKIND;
        } else {
            let n = counts.len();
            counts[n - 1] += jokers;
        }

        match counts[..] {
            [5] => Rank::FIVEKIND,
            [1, 4] => Rank::FOURKIND,
            [2, 3] => Rank::FULLHOUSE,
            [1, 1, 3] => Rank::THREEKIND,
            [1, 2, 2] => Rank::TWOPAIR,
            [1, 1, 1, 2] => Rank::ONEPAIR,
            _ => Rank::HIGHCARD,
        }
    }

    fn parse(line: &str) -> IResult<&str, Hand> {
        // function to parse a single line and produce a hand
        // Reading the  first part
        let (rest, cards) = alphanumeric1(line)?;
        let cards: Vec<Card> = cards.chars().map(|c| Card::from(c)).collect();

        // remove the space
        let (rest, _) = space1(rest)?;

        // parsing the rest
        let (rest, bidding) = map_res(digit1, str::parse::<i64>)(rest)?;
        let hand = Hand { bidding, cards };
        Ok((rest, hand))
    }
}
