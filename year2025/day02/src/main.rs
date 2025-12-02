use std::ops::RangeInclusive;
use std::str::FromStr;
use util::error::Errors;

#[derive(Debug, Copy, Clone)]
struct IdRange {
    min: i64,
    max: i64,
}

impl FromStr for IdRange {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elems = s.split('-');
        let min_elem = elems.next().ok_or(Errors::ParseError)?;
        let max_elem = elems.next().ok_or(Errors::ParseError)?;
        if elems.next().is_some() {
            return Err(Errors::ParseError);
        }
        let min = min_elem.parse::<i64>()?;
        let max = max_elem.parse::<i64>()?;
        Ok(Self { min, max })
    }
}

impl From<IdRange> for RangeInclusive<i64> {
    fn from(id_range: IdRange) -> Self {
        id_range.min..=id_range.max
    }
}

fn is_invalid_1(id: i64) -> bool {
    let str = id.to_string();
    let len = str.len();
    if len % 2 == 1 {
        return false;
    }

    let first_half = &str[..len / 2];
    let second_half = &str[len / 2..];
    first_half == second_half
}

fn is_invalid_2(id: i64) -> bool {
    let str = id.to_string();
    let len = str.len();

    for sublen in 1..=len / 2 {
        if len % sublen != 0 {
            continue;
        }
        let substr = str.get(..sublen).unwrap();
        let num = len / sublen;
        let mut invalid = true;
        for i in 1..num {
            if substr != &str[sublen * i..sublen * (i + 1)] {
                invalid = false;
                break;
            }
        }
        if invalid {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .split(',')
        .map(|line| line.parse::<IdRange>())
        .collect::<Result<Vec<_>, Errors>>()?;

    let mut result_part1 = 0_i64;
    let mut result_part2 = 0_i64;

    for id_range in input {
        let id_range: RangeInclusive<i64> = id_range.into();
        for id in id_range {
            if is_invalid_1(id) {
                result_part1 += id;
            }
            if is_invalid_2(id) {
                result_part2 += id;
            }
        }
    }

    println!("{result_part1}");
    println!("{result_part2}");

    Ok(())
}
