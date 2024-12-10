use std::collections::{HashSet, VecDeque};
use util::coord2d::UCoord2D;
use util::direction::Direction;
use util::grid2d::UGrid2D;

fn score(loc: UCoord2D, grid: &UGrid2D<u32>) -> Result<(usize, usize), util::error::Errors> {
    let mut check = VecDeque::from([loc]);
    let mut reached = HashSet::new();

    let mut result2 = 0usize;

    while let Some(candidate) = check.pop_front() {
        let val = *grid.get(candidate)?;

        if val == 9 {
            result2 += 1;
            reached.insert(candidate);
            continue;
        }

        for dir in Direction::directions() {
            let offset = dir.to_offset();
            let gloc = grid.icoord_to_grid(candidate + offset);
            if let Some(gloc) = gloc {
                if *grid.get(gloc)? == val + 1 {
                    check.push_back(gloc);
                }
            }
        }
    }

    Ok((reached.len(), result2))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<_> = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|s| s.to_owned())
        .collect();
    let dim = UCoord2D {
        x: input[0].len(),
        y: input.len(),
    };

    let mut starts = Vec::new();

    let grid = UGrid2D::generate(dim, |loc| {
        let UCoord2D { x, y } = loc;
        let val = input[y]
            .chars()
            .nth(x)
            .ok_or(util::error::Errors::DimError(format!("{x} is too high")))?
            .to_digit(10)
            .ok_or(util::error::Errors::UncategorizedError(
                "Invalid digit".into(),
            ))?;
        if val == 0 {
            starts.push(loc);
        }
        Ok(val)
    })?;

    let result = starts
        .iter()
        .map(|start| -> Result<_, util::error::Errors> { Ok(score(*start, &grid)?) })
        .try_fold(
            (0usize, 0usize),
            |(r0, r1), s| -> Result<_, util::error::Errors> {
                let (s0, s1) = s?;
                Ok((r0 + s0, r1 + s1))
            },
        )?;

    println!("{}", result.0);
    println!("{}", result.1);

    Ok(())
}
