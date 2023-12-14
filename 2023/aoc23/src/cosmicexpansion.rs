use std::collections::HashSet;
use std::fmt::Display;

pub static mut EMPTY_DISTANCE: usize = 2;

#[derive(Debug)]
pub struct Galaxies {
    universe: Vec<Vec<char>>, // representation of the universe
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Galaxies {
    #[inline]
    pub fn new(description: &str) -> Self {
        // function to create the galaxies from the string)
        // description
        let mut universe: Vec<Vec<char>> = Vec::new();
        for line in description.lines() {
            universe.push(line.chars().collect());
        }

        let n = universe.len();
        let m = universe[0].len();
        let mut rows = vec![0; n];
        let mut cols = vec![0; m];

        for (i, row) in universe.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if v == &'#' {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }

        let empty_rows: HashSet<usize> = rows
            .iter()
            .enumerate()
            .filter(|(_i, &v)| v == 0)
            .map(|(i, _v)| i)
            .collect();
        let empty_cols: HashSet<usize> = cols
            .iter()
            .enumerate()
            .filter(|(_i, &v)| v == 0)
            .map(|(i, _v)| i)
            .collect();
        // for

        Self {
            universe,
            empty_rows,
            empty_cols,
        }
    }

    // function to get the list of galaxies;
    pub fn get_galaxies(&self) -> Vec<(usize, usize)> {
        // function to get the list of galaxies
        let mut result = Vec::new();

        for (x, row) in self.universe.iter().enumerate() {
            for (y, v) in row.iter().enumerate() {
                if v == &'#' {
                    result.push((x, y));
                }
            }
        }
        result
    }

    pub fn distance(&self, galaxy1: (usize, usize), galaxy2: (usize, usize)) -> usize {
        // functioon to compute the distance between each galaxy
        let mut result = 0;

        for x in galaxy1.0.min(galaxy2.0)..galaxy1.0.max(galaxy2.0) {
            unsafe {
                result += if self.empty_rows.contains(&x) {
                    EMPTY_DISTANCE
                } else {
                    1
                };
            }
        }
        for y in galaxy1.1.min(galaxy2.1)..galaxy1.1.max(galaxy2.1) {
            unsafe {
                result += if self.empty_cols.contains(&y) {
                    EMPTY_DISTANCE
                } else {
                    1
                };
            }
        }

        result
    }

    pub fn galaxies_distances(&self) -> usize {
        // function to compute the distances
        let mut result = 0usize;

        let galaxies = self.get_galaxies();

        for i in 0..galaxies.len() {
            let (x, y) = galaxies[i];
            for j in i + 1..galaxies.len() {
                let (x2, y2) = galaxies[j];
                result += self.distance((x, y), (x2, y2));
            }
        }
        result
    }
}

impl Display for Galaxies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.universe.iter() {
            for c in row.iter() {
                write!(f, "{c}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
