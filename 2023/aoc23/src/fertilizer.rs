use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
#[derive(Debug)]
pub struct Garden {
    pub seeds: Vec<i64>,          // Vector of the seeds
    pub maps: Vec<Vec<Vec<i64>>>, // vector of the association
}

impl Garden {
    #[inline]
    pub fn new(content: &str) -> Self {
        Self::parse_garden(content).unwrap().1
    }

    pub fn parse_garden(content: &str) -> IResult<&str, Garden> {
        // function to parse the garden

        // processing the tag
        let (content, _) = tag("seeds: ")(content)?;

        // mapping the seeds
        let (content, seeds) =
            separated_list1(tag(" "), map_res(digit1, str::parse::<i64>))(content)?;

        let (content, _) = tag("\n\n")(content)?;
        // mappping the rest
        let (rest, maps) = separated_list1(tag("\n\n"), Self::parse_section)(content)?;

        let garden = Garden { seeds, maps };

        Ok((rest, garden))
    }

    pub fn reverse_map(&self, value: i64, map_index: usize) -> i64 {
        // function to get the reverse value of a given position
        let matrix = &self.maps[map_index];

        // loop ove the maps
        for row in matrix.iter() {
            // computing the diff
            let diff = value - row[0];

            if diff >= 0 && diff < row[2] {
                return diff + row[1];
            }
        }

        // in case no match we return the value
        value
    }
    pub fn value_at_map(&self, value: i64, map_index: usize) -> i64 {
        // function to get the value of [value] in the map indexed by [map_index]
        //
        let matrix = &self.maps[map_index];
        // println!("{matrix:?}");

        // no values with the current map
        if matrix.is_empty() {
            return value;
        }

        if value < matrix[0][1] {
            return value;
        }

        let index = matrix.partition_point(|row| value >= row[1] + row[2]);

        if index == matrix.len() {
            return value;
        }

        matrix[index][0] + value - matrix[index][1]
    }

    // function to parse a section
    fn parse_section(content: &str) -> IResult<&str, Vec<Vec<i64>>> {
        // function to parse the content of a given

        // remove the first line
        let (rest, _) = preceded(take_until("\n"), newline)(content)?;

        let (rest, maps) = separated_list1(
            newline,
            separated_list1(tag(" "), map_res(digit1, str::parse::<i64>)),
        )(rest)?;

        Ok((rest, maps))
    }

    pub fn sort_sections(&mut self) {
        for section in self.maps.iter_mut() {
            section.sort_unstable_by_key(|row| row[1]);
        }
    }
    pub fn is_seed(&self, seed: i64) -> bool {
        // function to verify if a given seed is correct
        for range in self.seeds.chunks(2) {
            if seed >= range[0] && seed < range[0] + range[1] {
                return true;
            }
        }
        false
    }

    pub fn location_seed(&self, location: i64) -> Option<i64> {
        // function to get if we get the reverse location
        let mut value = location;
        for index in (0..self.maps.len()).rev() {
            value = self.reverse_map(value, index);
        }

        match self.is_seed(value) {
            true => Some(value),
            false => None,
        }
    }
    pub fn seed_soil_number(&self, seed: i64) -> i64 {
        // function to get the seed value
        let mut seed = seed;
        // println!("{seed}");
        for index in 0..self.maps.len() {
            seed = self.value_at_map(seed, index);
            // println!("{seed}");
        }
        seed
    }
}
