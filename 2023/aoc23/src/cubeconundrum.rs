use std::i8::MAX;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{character::complete::digit1, sequence::preceded, IResult};

#[derive(Debug)]
pub struct Game {
    pub id: u32,            // id for the game
    results: Vec<[u32; 3]>, // results for eac
}

pub const MAX_DRAWS: [u32; 3] = [12, 13, 14];
impl Game {
    #[inline]
    pub fn new(id: u32, results: Vec<[u32; 3]>) -> Self {
        Self { id, results }
    }

    pub fn possible(&self) -> bool {
        for result in self.results.iter() {
            for i in 0..3 {
                if result[i] > MAX_DRAWS[i] {
                    return false;
                }
            }
        }
        true
    }
    pub fn power(&self) -> u32 {
        // function to compute the power of a game
        let min_red = self.results.iter().map(|draw| draw[0]).max().unwrap();
        let min_green = self.results.iter().map(|draw| draw[1]).max().unwrap();
        let min_blue = self.results.iter().map(|draw| draw[2]).max().unwrap();
        return min_red * min_green * min_blue;
    }
}

fn parse_draw(part: &str) -> IResult<&str, (u32, &str)> {
    let (rest, (val, _, color)) =
        tuple((map_res(digit1, str::parse::<u32>), space1, alpha1))(part)?;

    Ok((rest, (val, color)))
}

pub fn parse_game(line: &str) -> IResult<&str, Game> {
    // function to parse the game id

    // removing the game id
    let (rest, id) = preceded(tag("Game "), map_res(digit1, str::parse::<u32>))(line)?;

    // matching the :
    let (rest, _) = tag(":")(rest)?;

    // now we have to match the games
    let mut results = Vec::new();
    for part in rest.split(';') {
        let mut draws = [0u32, 0, 0];
        let part = part.trim_start();
        let (_, parts) = separated_list1(tag(", "), parse_draw)(part)?;
        for (val, color) in parts {
            match color {
                "red" => draws[0] += val,
                "green" => draws[1] += val,
                "blue" => draws[2] += val,
                _ => panic!("unroconized color"),
            }
        }
        results.push(draws);
    }

    let game = Game::new(id, results);
    Ok(("", game))
}

// let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
