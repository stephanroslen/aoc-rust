use std::str::FromStr;
use util::coord2d::UCoord2D;
use util::direction8::Direction8;
use util::error::Errors;
use util::grid2d::UGrid2D;

#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Paper,
}

impl FromStr for Slot {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Slot::Empty),
            "@" => Ok(Slot::Paper),
            _ => Err(Errors::ParseError),
        }
    }
}

#[derive(Debug, Clone)]
struct Field {
    grid: UGrid2D<Slot>,
}

impl FromStr for Field {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();

        let dim = UCoord2D {
            x: lines[0].len(),
            y: lines.len(),
        };

        let grid = UGrid2D::generate(dim, |coord| {
            Ok(Slot::from_str(&lines[coord.y][coord.x..=coord.x])?)
        })?;

        Ok(Self { grid })
    }
}

impl Field {
    fn part1(&self) -> Result<usize, Errors> {
        let mut result = 0_usize;
        let dim = self.grid.dim();

        for y in 0..dim.y {
            for x in 0..dim.x {
                let coord = UCoord2D { x, y };
                let slot = self.grid.get(coord)?;

                if let Slot::Empty = slot {
                    continue;
                }

                let mut count = 0;

                for dir in Direction8::directions() {
                    let offset = dir.to_offset();
                    let new_coord = coord + offset;

                    if let Some(new_coord) = self.grid.icoord_to_grid(new_coord) {
                        let new_slot = self.grid.get(new_coord)?;

                        if let Slot::Paper = new_slot {
                            count += 1;
                        }
                    }
                }

                if count < 4 {
                    result += 1;
                }
            }
        }

        Ok(result)
    }

    fn part2_mut(&mut self) -> Result<usize, Errors> {
        let mut result = 0_usize;

        let dim = self.grid.dim();

        loop {
            let mut changed = false;

            for y in 0..dim.y {
                for x in 0..dim.x {
                    let coord = UCoord2D { x, y };
                    let slot = self.grid.get(coord)?;

                    if let Slot::Empty = slot {
                        continue;
                    }

                    let mut count = 0;

                    for dir in Direction8::directions() {
                        let offset = dir.to_offset();
                        let new_coord = coord + offset;

                        if let Some(new_coord) = self.grid.icoord_to_grid(new_coord) {
                            let new_slot = self.grid.get(new_coord)?;

                            if let Slot::Paper = new_slot {
                                count += 1;
                            }
                        }
                    }

                    if count < 4 {
                        result += 1;
                        *self.grid.get_mut(coord)? = Slot::Empty;
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }

        Ok(result)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut field = std::fs::read_to_string("input")?
        .trim_end()
        .parse::<Field>()?;

    let result_part1 = field.part1()?;
    let result_part2 = field.part2_mut()?;

    println!("{result_part1}");
    println!("{result_part2}");

    Ok(())
}
