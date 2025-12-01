use std::str::FromStr;
use util::error::Errors;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Turn {
    direction: Direction,
    steps: u32,
}

impl FromStr for Turn {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = &s[0..1];
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(Errors::ParseError),
        };
        let steps: u32 = s[1..].parse()?;
        Ok(Self { direction, steps })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<Turn>())
        .collect::<Result<Vec<_>, Errors>>()?;

    let mut result_part1 = 0usize;
    let mut result_part2 = 0usize;

    let mut position = 50_i32;
    for turn in &input {
        let abs = turn.steps;
        let dir = turn.direction.value();

        for _ in 0..abs {
            position += dir;

            if position < 0 {
                position += 100;
            }
            if position > 99 {
                position -= 100;
            }

            if position == 0 {
                result_part2 += 1;
            }
        }

        if position == 0 {
            result_part1 += 1;
        }
    }

    println!("{result_part1}");
    println!("{result_part2}");

    Ok(())
}
