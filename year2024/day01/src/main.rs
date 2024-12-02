use util::error::Errors;

#[derive(Copy, Clone, Debug)]
struct Line {
    num0: u32,
    num1: u32,
}

impl std::str::FromStr for Line {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries: Vec<_> = s.split_whitespace().collect();
        if entries.len() != 2 {
            return Err(Errors::ParseError);
        }
        Ok(Self {
            num0: entries[0].parse::<u32>()?,
            num1: entries[1].parse::<u32>()?,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<Line>())
        .collect::<Result<Vec<Line>, Errors>>()?;
    let col0: Vec<_> = input.iter().map(|l| l.num0).collect();
    let col1: Vec<_> = input.iter().map(|l| l.num1).collect();

    let col0sorted = {
        let mut tmp = col0.clone();
        tmp.sort();
        tmp
    };
    let col1sorted = {
        let mut tmp = col1.clone();
        tmp.sort();
        tmp
    };

    let result1 = col0sorted
        .iter()
        .zip(col1sorted.iter())
        .map(|(a, b)| {
            let min = std::cmp::min(a, b);
            let max = std::cmp::max(a, b);
            max - min
        })
        .sum::<u32>();
    println!("{}", result1);

    let occur0 = {
        let mut tmp = std::collections::HashMap::<u32, u32>::new();
        for val in col0sorted {
            *tmp.entry(val).or_default() += 1;
        }
        tmp
    };
    let occur1 = {
        let mut tmp = std::collections::HashMap::<u32, u32>::new();
        for val in col1sorted {
            *tmp.entry(val).or_default() += 1;
        }
        tmp
    };
    let result2 = occur0
        .iter()
        .map(|(val, cnt)| val * occur1.get(val).copied().unwrap_or_default() * cnt)
        .sum::<u32>();
    println!("{}", result2);
    Ok(())
}
