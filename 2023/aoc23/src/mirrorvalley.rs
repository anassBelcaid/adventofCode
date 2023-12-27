use std::fmt::Display;

#[derive(Debug)]
pub struct Valley {
    rows: Vec<Vec<char>>, // rows
    cols: Vec<Vec<char>>, // columns
    n: usize,
    m: usize,
}

impl Valley {
    #[inline]
    pub fn new(content: &str) -> Self {
        // function to create the Valley configuration from the content

        // let mut rows: Vec<String> = Vec::new();
        let n = content.lines().count();
        let m = content.lines().nth(0).unwrap().len();

        let rows: Vec<Vec<char>> = content
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let mut cols = vec![Vec::new(); m];
        // now collecting the columns
        for line in content.lines() {
            for (j, c) in line.chars().enumerate() {
                cols[j].push(c);
            }
        }

        Self { rows, cols, n, m }
    }
    fn hamming_distance(left: &Vec<char>, right: &Vec<char>) -> usize {
        // function to compute the hamming distance between left and rigth

        left.iter()
            .zip(right.iter())
            .map(|(x, y)| if x == y { 0 } else { 1 })
            .sum()
    }

    fn row_symmetry_distance(&self, row: usize) -> usize {
        // function to compute the number of indeffirence between row
        let mut i = row as i32;
        let mut j = row as i32 + 1;
        let mut count = 0;

        while i >= 0 && (j as usize) < self.rows.len() {
            count += Self::hamming_distance(&self.rows[i as usize], &self.rows[j as usize]);

            i -= 1;
            j += 1;
        }

        count
    }

    fn col_symmetry_distance(&self, col: usize) -> usize {
        // function to compute the number of indeffirence between row
        let mut i = col as i32;
        let mut j = col as i32 + 1;
        let mut count = 0;

        while i >= 0 && (j as usize) < self.cols.len() {
            count += Self::hamming_distance(&self.cols[i as usize], &self.cols[j as usize]);

            i -= 1;
            j += 1;
        }

        count
    }

    pub fn reflexion_value(&self) -> usize {
        // function to compute the reflexion value with perfect match
        // first check the rows
        for row in 0..self.rows.len() - 1 {
            if self.row_symmetry_distance(row) == 0 {
                return 100 * (row + 1);
            }
        }
        for col in 0..self.cols.len() - 1 {
            if self.col_symmetry_distance(col) == 0 {
                return col + 1;
            }
        }
        0
    }

    pub fn smugged_reflexion_value(&self) -> usize {
        // function to compute the reflexion value with perfect match
        // first check the rows
        for row in 0..self.rows.len() - 1 {
            if self.row_symmetry_distance(row) == 1 {
                return 100 * (row + 1);
            }
        }
        for col in 0..self.cols.len() - 1 {
            if self.col_symmetry_distance(col) == 1 {
                return col + 1;
            }
        }
        0
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.rows.iter() {
            for c in line.iter() {
                write!(f, "{c}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
