use std::str::FromStr;
use util::error::Errors;
use util::grid2d::{UCoord2D, UGrid2D};

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(Errors::ParseError),
        }
    }
}

impl Operation {
    fn neutral_element(&self) -> u64 {
        match self {
            Operation::Add => 0,
            Operation::Multiply => 1,
        }
    }

    fn execute(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

#[derive(Debug, Clone)]
struct Field1 {
    ugrid2d: UGrid2D<u64>,
    opers: Vec<Operation>,
}

impl FromStr for Field1 {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim_end().split("\n").collect::<Vec<_>>();
        let num_lines = lines.len();

        let numbers = lines
            .iter()
            .take(num_lines - 1)
            .map(|s| -> Result<Vec<_>, _> {
                s.split_whitespace()
                    .map(|s| s.parse::<u64>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let dim = UCoord2D {
            x: numbers[0].len(),
            y: numbers.len(),
        };

        let ugrid2d = UGrid2D::generate(dim, |coord| Ok(numbers[coord.y][coord.x]))?;

        let opers = lines
            .into_iter()
            .last()
            .ok_or(Errors::ParseError)?
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<Operation>, _>>()?;

        assert_eq!(dim.x, opers.len());

        Ok(Self { ugrid2d, opers })
    }
}

impl Field1 {
    fn result(&self) -> Result<u64, Errors> {
        let mut result = 0;

        let y_size = self.ugrid2d.dim().y;
        for (x, op) in self.opers.iter().enumerate() {
            let mut col_result = op.neutral_element();
            for i_y in 0..y_size {
                let coord = UCoord2D { x, y: i_y };

                col_result = op.execute(col_result, *self.ugrid2d.get(coord)?);
            }

            result += col_result;
        }

        Ok(result)
    }
}

#[derive(Debug, Clone)]
struct Col2 {
    values: Vec<u64>,
    op: Operation,
}

impl Col2 {
    fn result(&self) -> u64 {
        self.values
            .iter()
            .fold(self.op.neutral_element(), |acc, &v| self.op.execute(acc, v))
    }
}

#[derive(Debug, Clone)]
struct Field2 {
    cols: Vec<Col2>,
}

impl FromStr for Field2 {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim_end().split("\n").collect::<Vec<_>>();

        let num_lines = lines.len();
        let last_line = lines[num_lines - 1];

        let mut cols = Vec::new();

        for i_sgn_col in 0.. {
            let mut is_beyond_line_end = true;

            let potential_oper = last_line.get(i_sgn_col..=i_sgn_col);
            let found_oper = if let Some(potential_oper) = potential_oper {
                is_beyond_line_end = false;
                potential_oper.parse::<Operation>().ok()
            } else {
                None
            };

            if let Some(oper) = found_oper {
                let values = Vec::new();
                let op = oper;
                cols.push(Col2 { values, op });
            }

            let mut found_value = None;

            for i_line in 0..num_lines - 1 {
                let potential_value = lines[i_line].get(i_sgn_col..=i_sgn_col);
                if let Some(potential_value) = potential_value {
                    is_beyond_line_end = false;
                    let potential_value = potential_value.parse::<u64>().ok();

                    if let Some(value) = potential_value {
                        if let Some(found_value) = &mut found_value {
                            *found_value = *found_value * 10 + value;
                        } else {
                            found_value = Some(value);
                        }
                    }
                }
            }

            if let Some(found_value) = found_value {
                cols.last_mut().unwrap().values.push(found_value);
            }

            if is_beyond_line_end {
                break;
            }
        }

        Ok(Self { cols })
    }
}

impl Field2 {
    fn result(&self) -> u64 {
        self.cols.iter().map(|c| c.result()).sum()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?;

    let field1 = input.parse::<Field1>()?;

    let result_part1 = field1.result()?;

    let field2 = input.parse::<Field2>()?;

    let result_part2 = field2.result();

    println!("{result_part1}");
    println!("{result_part2}");

    Ok(())
}
