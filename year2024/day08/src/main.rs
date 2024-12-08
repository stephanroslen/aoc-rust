use std::collections::{HashMap, HashSet};
use util::coord2d::ICoord2D;
use util::coord2d::UCoord2D;

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

    let mut map: HashMap<char, HashSet<ICoord2D>> = HashMap::new();

    for iy in 0..dim.y {
        for ix in 0..dim.x {
            let c = input[iy]
                .chars()
                .nth(ix)
                .ok_or(util::error::Errors::UncategorizedError(format!(
                    "No character at location {ix}"
                )))?;
            if c != '.' {
                map.entry(c).or_default().insert(ICoord2D {
                    x: ix as isize,
                    y: iy as isize,
                });
            }
        }
    }

    let mut antinodes1: HashSet<ICoord2D> = HashSet::new();

    for (_, locs) in &map {
        for loc0 in locs {
            for loc1 in locs {
                if loc0 == loc1 {
                    continue;
                }
                let antinode_loc = *loc0 - (*loc1 - *loc0);
                if antinode_loc.x < 0
                    || antinode_loc.y < 0
                    || antinode_loc.x >= dim.x as isize
                    || antinode_loc.y >= dim.y as isize
                {
                    continue;
                }
                antinodes1.insert(antinode_loc);
            }
        }
    }

    println!("{}", antinodes1.len());

    let mut antinodes2: HashSet<ICoord2D> = HashSet::new();

    for (_, locs) in &map {
        for loc0 in locs {
            for loc1 in locs {
                if loc0 == loc1 {
                    continue;
                }
                let diff = *loc1 - *loc0;
                let mut antinode_loc = *loc0;
                loop {
                    if antinode_loc.x < 0
                        || antinode_loc.y < 0
                        || antinode_loc.x >= dim.x as isize
                        || antinode_loc.y >= dim.y as isize
                    {
                        break;
                    }
                    antinodes2.insert(antinode_loc);
                    antinode_loc -= diff;
                }
            }
        }
    }

    println!("{}", antinodes2.len());

    Ok(())
}
