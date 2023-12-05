use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use std::collections::HashSet;

use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::{sequence::preceded, IResult};

#[derive(Debug)]
pub struct ScratchCard {
    id: usize,
    winning: HashSet<i32>,
    numbers: Vec<i32>,
}

impl ScratchCard {
    #[inline]
    pub fn new(line: &str) -> Self {
        let winning: HashSet<i32> = HashSet::new();
        let numbers: Vec<i32> = Vec::new();
        let id = 0;

        // computing the result
        parser(line).unwrap().1
    }

    pub fn points(&self) -> i32 {
        // function to compute the number of points worth of the card
        let count: usize = self
            .numbers
            .iter()
            .filter(|&x| self.winning.contains(x))
            .count();

        if count == 0 {
            return 0;
        } else {
            return 2_i32.pow(count as u32 - 1);
        }
    }

    pub fn num_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&x| self.winning.contains(x))
            .count()
    }
}

// implementing the num parser
fn parser(line: &str) -> IResult<&str, ScratchCard> {
    // function to parse a line an return a scratchCard

    let (rest, id) = preceded(
        tuple((tag("Card"), space1)),
        map_res(digit1, str::parse::<usize>),
    )(line)?;
    let (rest, _) = tuple((tag(":"), space1))(rest)?;

    let number_parser = separated_list1(space1, map_res(digit1, str::parse::<i32>));
    let (rest, all_numbers) =
        separated_list1(tuple((space1, tag("|"), space1)), number_parser)(rest)?;

    // parse the numbers
    let numbers = all_numbers[1].clone();
    let winning = all_numbers[0].iter().map(|x| *x).collect::<HashSet<i32>>();

    // parsing the set of numbers
    Ok((
        rest,
        ScratchCard {
            id,
            winning,
            numbers,
        },
    ))
}
