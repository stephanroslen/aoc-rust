use std::collections::VecDeque;
use util::coord2d::{ICoord2D, UCoord2D};
use util::direction::Direction;
use util::error::Errors;
use util::grid2d::UGrid2D;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Field {
    Open,
    Closed,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist_map = {
        let raw_input = std::fs::read_to_string("input")?;
        let raw_lines = raw_input.trim_end().split('\n').collect::<Vec<_>>();

        let dim = UCoord2D {
            x: raw_lines[0].len(),
            y: raw_lines.len(),
        };

        let mut start = UCoord2D::default();
        let grid = UGrid2D::generate(dim, |UCoord2D { x, y }| -> Result<Field, Errors> {
            match raw_lines[y]
                .chars()
                .nth(x)
                .expect("Input character expected")
            {
                '#' => Ok(Field::Closed),
                '.' | 'E' => Ok(Field::Open),
                'S' => {
                    start = UCoord2D { x, y };
                    Ok(Field::Open)
                }
                _ => Err(Errors::UncategorizedError("Invalid input".into())),
            }
        })?;

        let mut fringe = VecDeque::from([(start, 0usize)]);
        let mut dist_map = UGrid2D::generate(dim, |_| Ok(usize::MAX))?;

        while let Some((candidate, cost)) = fringe.pop_front() {
            if *grid.get(candidate)? == Field::Closed {
                continue;
            }
            let field_cost = dist_map.get_mut(candidate)?;
            if cost < *field_cost {
                *field_cost = cost;
                for dir in Direction::directions() {
                    let offset = dir.to_offset();
                    if let Some(new_candidate) = grid.icoord_to_grid(candidate + offset) {
                        let new_cost = cost + 1;
                        fringe.push_back((new_candidate, new_cost));
                    }
                }
            }
        }
        dist_map
    };

    println!("{}", solution(&dist_map, 2)?);
    println!("{}", solution(&dist_map, 20)?);

    Ok(())
}

fn solution(dist_map: &UGrid2D<usize>, cheat_time: usize) -> Result<usize, Errors> {
    let cheat_time = cheat_time as isize;
    let dim = dist_map.dim();

    let mut result = 0usize;

    for iy in 0..dim.y {
        for ix in 0..dim.x {
            let from_candidate = UCoord2D { x: ix, y: iy };
            let from_dist = *dist_map.get(from_candidate)?;
            if from_dist == usize::MAX {
                continue;
            }

            for iy_offset in -cheat_time..=cheat_time {
                let iy_offset_abs = iy_offset.abs();
                for ix_offset in -cheat_time + iy_offset_abs..=cheat_time - iy_offset_abs {
                    let offset = ICoord2D {
                        x: ix_offset,
                        y: iy_offset,
                    };
                    if let Some(to_candidate) = dist_map.icoord_to_grid(from_candidate + offset) {
                        let ix_offset_abs = ix_offset.abs();
                        let to_dist = *dist_map.get(to_candidate)?;

                        if to_dist == usize::MAX {
                            continue;
                        }

                        let ix_offset_abs = ix_offset_abs as usize;
                        let iy_offset_abs = iy_offset_abs as usize;

                        assert!(ix_offset_abs + iy_offset_abs <= cheat_time as usize);

                        let cheat_to_dist = from_dist + ix_offset_abs + iy_offset_abs;
                        if to_dist > cheat_to_dist {
                            let saves = to_dist - cheat_to_dist;
                            if saves >= 100 {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}
