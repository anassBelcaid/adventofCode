use nom::{
    bytes::streaming::tag,
    character::complete::digit1,
    character::{complete::newline, streaming::space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Race {
    pub available_time: i64, // available time
    pub distance: i64,       //record on second
}

impl Race {
    pub fn num_ways(&self) -> usize {
        // number of ways to win the Race
        let mut count = 0usize;

        // checking for any charging time
        for speed in 0..=self.available_time {
            // getting the remaining the time
            let remaining = self.available_time - speed;

            // getting the covered distance
            let covered = remaining * speed;
            if covered > self.distance {
                count += 1;
            }
        }
        count
    }
}

#[derive(Debug)]
pub struct Game {
    pub races: Vec<Race>, // set of races to win
}

impl Game {
    #[inline]
    pub fn new(content: &str) -> Self {
        // function to create the Game from the
        // description in the content
        Self::parse(content).unwrap().1
    }

    pub fn margin(&self) -> usize {
        // Get the margin for the game
        // which the multiplication of the number of ways
        // for each race
        self.races.iter().map(Race::num_ways).product()
    }

    pub fn as_single_game(&self) -> Race {
        // function to combine the game as a single game

        let available_time: Vec<String> = self
            .races
            .iter()
            .map(|race| race.available_time.to_string())
            .collect();
        let available_time: i64 = available_time.join("").parse::<i64>().unwrap();

        let distance: Vec<String> = self
            .races
            .iter()
            .map(|race| race.distance.to_string())
            .collect();
        let distance: i64 = distance.join("").parse::<i64>().unwrap();

        // compute the time
        Race {
            available_time,
            distance,
        }
    }

    fn parse(content: &str) -> IResult<&str, Game> {
        // proper parser for the data
        let mut races: Vec<Race> = Vec::new();
        // removing the first line
        let (content, times) = preceded(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, map_res(digit1, str::parse::<i64>)),
        )(content)?;

        let (content, _) = newline(content)?;
        let (content, distances) = preceded(
            tuple((tag("Distance:"), space1)),
            separated_list1(space1, map_res(digit1, str::parse::<i64>)),
        )(content)?;

        for (t, d) in times.iter().zip(distances.iter()) {
            races.push(Race {
                available_time: *t,
                distance: *d,
            });
        }
        Ok(("", Game { races }))
    }
}
