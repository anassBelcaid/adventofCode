use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

// Cache type for the dp computation in the num of ways
type Cache = Vec<Vec<i64>>;

#[derive(Debug)]
pub struct Spring {
    description: Vec<char>,
    config: Vec<usize>,
}

impl Spring {
    pub fn fold(&mut self) {
        // function to fold the values 5 times
        let initial_config = self.config.clone();
        self.description.push('?');
        let initial_des = self.description.clone();
        for _ in 0..4 {
            self.config.extend(initial_config.clone());
            self.description.extend(initial_des.clone());
        }
        self.description.pop();
    }
    pub fn is_spring(c: char) -> bool {
        return c == '#';
    }

    pub fn num_ways(&self) -> i64 {
        // compute the number of ways we could fill the unknons
        // ? values by either a spring or empty

        // First we need to create the dp table
        //   Dp{i,j} == how many ways to fill S[::i], where S
        //   is the representation of the spring and, using  I[::j]
        //   where I is the list of items
        let m = self.description.len();
        let n = self.config.len();
        let mut dp = vec![vec![-1; m + 1]; n + 1];

        dp[0][0] = 1;

        // base case for ending the item but still we have a list of character
        // They must all be different from #
        for j in 1..=m {
            dp[0][j] = if self.description[..j].iter().all(|x| *x != '#') {
                1
            } else {
                0
            };
        }

        // base case for ending characters but still we have some item in that case 0
        for i in 1..=n {
            dp[i][0] = 0; // we attain the end of the string but still have some items left
        }

        self.compute_value(n, m, &mut dp)
    }
    fn compute_value(&self, item: usize, index: usize, dp: &mut Cache) -> i64 {
        // function to compute the dp value

        // println!("call with index = {index} \t item = {item}");
        if dp[item][index] == -1 {
            let c = self.description[index - 1];
            let l = self.config[item - 1];

            if c == '.' {
                dp[item][index] = self.compute_value(item, index - 1, dp);
            }

            if c == '#' {
                if self.ending_plateau(index - 1, l) {
                    // compute the start
                    let mut start = index - l;
                    if start > 0 {
                        start -= 1; // we need to forcefuly skip start as it is an ending char
                    }
                    dp[item][index] = self.compute_value(item - 1, start, dp)
                } else {
                    dp[item][index] = 0;
                }
            } else {
                dp[item][index] = self.compute_value(item, index - 1, dp);

                // adding the second part
                if self.ending_plateau(index - 1, l) {
                    let mut start = index - l;
                    if start > 0 {
                        start -= 1;
                    }

                    dp[item][index] += self.compute_value(item - 1, start, dp);
                }
            }
        }

        dp[item][index]
    }

    fn ending_plateau(&self, tail: usize, l: usize) -> bool {
        // function to check if we could end a pleateau of lenght [l] at the position [index]

        // first check if we could not pass 0
        if tail < l - 1 {
            return false;
        }

        // computing the start of the pleateau
        let start = tail + 1 - l;

        // we need that all the caracter in the range [start..=tail] to be different from .
        if self.description[start..=tail].iter().any(|x| x == &'.') {
            return false;
        }

        // now if start is not null we need that ending chracter not be equal to #
        if start > 0 && self.description[start - 1] == '#' {
            return false;
        }

        true
    }

    #[inline]
    pub fn new(description: &str) -> Self {
        Self::parse(&description).unwrap().1
    }

    fn parse(line: &str) -> IResult<&str, Self> {
        // function to parse the content
        // taking the firest part
        let (rest, description) = take_till(|c| c == ' ')(line)?;
        let description: Vec<char> = description.chars().collect();

        // parsing the empty space
        let (rest, _) = tag(" ")(rest)?;

        // now we map the numbers
        let (rest, config) = separated_list1(tag(","), map_res(digit1, str::parse::<usize>))(rest)?;

        Ok((
            rest,
            Self {
                description,
                config,
            },
        ))
    }
}
