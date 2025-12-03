use std::str::FromStr;
use util::error::Errors;

#[derive(Debug, Clone)]
struct Line {
    entries: Vec<u8>,
}

impl FromStr for Line {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .chars()
            .map(|c| {
                let val = c.to_digit(10).ok_or(Errors::ParseError)?;
                Ok(val as u8)
            })
            .collect::<Result<Vec<_>, Errors>>()?;
        Ok(Self { entries })
    }
}

impl Line {
    fn solve(&self, n: usize) -> u64 {
        let mut progression = 0_usize;
        let mut result = 0_u64;

        let len = self.entries.len();

        for i in 0..n {
            result *= 10;

            let (slot, digit) = self
                .entries
                .iter()
                .enumerate()
                // we need to keep elements in the end to fill the n digits.
                .take(self.entries.len() - n + 1 + i)
                // skip the last picked one - and those before.
                .skip(progression)
                // pick the highest possible digit greedily - prefer early occurrences to allow wiggle room for later ones.
                .max_by_key(|(slot, v)| (**v, len - *slot))
                .expect("Element expected to exist");
            progression = slot + 1;

            result += *digit as u64;
        }

        result
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<Line>())
        .collect::<Result<Vec<_>, Errors>>()?;

    let mut result_part1 = 0_u64;
    let mut result_part2 = 0_u64;

    for line in input {
        result_part1 += line.solve(2);
        result_part2 += line.solve(12);
    }

    println!("{result_part1}");
    println!("{result_part2}");

    Ok(())
}
