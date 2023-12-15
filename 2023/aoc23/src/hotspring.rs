use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    number::complete::u32,
    IResult,
};

#[derive(Debug)]
pub struct Spring {
    description: String,
    config: Vec<usize>,
}

impl Spring {
    pub fn is_spring(c: char) -> bool {
        return c == '#';
    }

    #[inline]
    pub fn new(description: &str) -> Self {
        Self::parse(&description).unwrap().1
    }

    fn parse(line: &str) -> IResult<&str, Self> {
        // function to parse the content
        // taking the firest part
        let (rest, description) = take_till(|c| c == ' ')(line)?;

        // parsing the empty space
        let (rest, _) = tag(" ")(rest)?;

        // now we map the numbers
        let (rest, config) = separated_list1(tag(","), map_res(digit1, str::parse::<usize>))(rest)?;

        Ok((
            rest,
            Self {
                description: description.to_string(),
                config,
            },
        ))
    }
}
