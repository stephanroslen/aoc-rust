use regex::Regex;
use std::str::FromStr;
use std::sync::OnceLock;
use util::error::Errors;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Matrix {
    values: [i64; 4],
}

impl Matrix {
    fn determinant(&self) -> i64 {
        self.values[0] * self.values[3] - self.values[1] * self.values[2]
    }

    fn inverse_times_determinant(&self) -> Self {
        Self {
            values: [
                self.values[3],
                -self.values[1],
                -self.values[2],
                self.values[0],
            ],
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Vector {
    values: [i64; 2],
}

impl Vector {
    fn matrix_times_vector(&self, matrix: &Matrix) -> Self {
        Self {
            values: [
                self.values[0] * matrix.values[0] + self.values[1] * matrix.values[1],
                self.values[0] * matrix.values[2] + self.values[1] * matrix.values[3],
            ],
        }
    }

    fn invert_matrix_times_vector(&self, matrix: &Matrix) -> Option<Self> {
        let det = matrix.determinant();
        if det == 0 {
            return None;
        }
        let inv_times_det = matrix.inverse_times_determinant();
        let candidate = self.matrix_times_vector(&inv_times_det);

        if candidate.values[0] % det != 0 || candidate.values[1] % det != 0 {
            return None;
        }

        Some(Vector {
            values: [candidate.values[0] / det, candidate.values[1] / det],
        })
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Entry {
    matrix: Matrix,
    vector: Vector,
}

impl Entry {
    fn solve(&self, vector: &Vector) -> usize {
        let candidate = vector.invert_matrix_times_vector(&self.matrix);

        match candidate {
            Some(Vector { values: [x, y] }) => x as usize * 3 + y as usize,
            None => 0usize,
        }
    }

    fn result1(&self) -> usize {
        self.solve(&self.vector)
    }

    fn result2(&self) -> usize {
        self.solve(&Vector {
            values: [
                self.vector.values[0] + 10000000000000,
                self.vector.values[1] + 10000000000000,
            ],
        })
    }
}

impl FromStr for Entry {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Entry::default();

        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = REGEX.get_or_init(|| {
            Regex::new(
                r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
            ).unwrap()
        });

        let cpt = regex.captures(s).ok_or(Errors::ParseError)?;

        entry.matrix.values[0] = cpt[1].parse::<i64>()?;
        entry.matrix.values[1] = cpt[3].parse::<i64>()?;
        entry.matrix.values[2] = cpt[2].parse::<i64>()?;
        entry.matrix.values[3] = cpt[4].parse::<i64>()?;

        entry.vector.values[0] = cpt[5].parse::<i64>()?;
        entry.vector.values[1] = cpt[6].parse::<i64>()?;

        Ok(entry)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .split("\n\n")
        .map(|s| s.parse::<Entry>())
        .collect::<Result<Vec<_>, _>>()?;

    let result1 = input.iter().map(|entry| entry.result1()).sum::<usize>();

    println!("{}", result1);

    let result2 = input.iter().map(|entry| entry.result2()).sum::<usize>();

    println!("{}", result2);

    Ok(())
}
