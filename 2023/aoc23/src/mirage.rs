#[derive(Debug)]
pub struct History {
    vals: Vec<i64>,
}
impl History {
    #[inline]
    pub fn new(values: &str) -> Self {
        let vals: Vec<_> = values
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        Self { vals }
    }
    pub fn extraploate_value(&self) -> i64 {
        // function to create the extrapolated value

        let table = self.create_table();

        table.iter().map(|row| row.last().unwrap()).sum()
    }

    pub fn extraploate_left(&self) -> i64 {
        let table = self.create_table();
        let mut value = 0;

        for row in table.iter().rev() {
            value = row[0] - value;
        }
        value
    }

    // function to create the reduction process until the end
    pub fn create_table(&self) -> Vec<Vec<i64>> {
        // function to create reduction table by subtracting each
        // two neighbors until we get the set of zeros or a single value

        let mut table = Vec::new();

        // pushing the first value
        table.push(self.vals.clone());

        while table.last().unwrap().len() > 1 && table.last().unwrap().iter().any(|x| *x != 0) {
            // Create the diff table
            let mut new_diff = Vec::new();
            for wind in table.last().unwrap().windows(2) {
                new_diff.push(wind[1] - wind[0]);
            }
            table.push(new_diff);
        }

        table
    }
}
