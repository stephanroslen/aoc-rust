use util::error::Errors;

#[derive(Clone, Debug)]
struct Line {
    nums: Vec<u32>,
}

impl Line {
    fn iter_is_valid<'a>(iter: impl Iterator<Item = &'a u32> + Clone) -> bool {
        iter.clone()
            .zip(iter.skip(1))
            .fold(
                (true, Option::<bool>::None),
                |(prev_ok, prev_dir), (a, b)| {
                    let dir = a < b;
                    let min = std::cmp::min(a, b);
                    let max = std::cmp::max(a, b);
                    let diff = max - min;
                    (
                        prev_ok && diff >= 1 && diff <= 3 && prev_dir.unwrap_or(dir) == dir,
                        Some(dir),
                    )
                },
            )
            .0
    }
    fn part1_is_valid(&self) -> bool {
        Self::iter_is_valid(self.nums.iter())
    }

    fn part2_is_valid(&self) -> bool {
        let len = self.nums.len();
        for sel_idx in 0..len {
            let iter = self
                .nums
                .iter()
                .enumerate()
                .filter(|(idx, _)| sel_idx != *idx)
                .map(|(_, v)| v);
            if Self::iter_is_valid(iter) {
                return true;
            }
        }
        false
    }
}

impl std::str::FromStr for Line {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            nums: s
                .split_whitespace()
                .map(|v| v.parse::<u32>())
                .collect::<Result<Vec<u32>, std::num::ParseIntError>>()?,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<Line>())
        .collect::<Result<Vec<Line>, Errors>>()?;

    let result1 = input.iter().filter(|l| l.part1_is_valid()).count();
    println!("{}", result1);

    let result2 = input.iter().filter(|l| l.part2_is_valid()).count();
    println!("{}", result2);

    Ok(())
}
