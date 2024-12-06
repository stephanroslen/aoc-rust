use std::collections::HashSet;
use util::coord2d::{ICoord2D, UCoord2D};
use util::direction::Direction;
use util::grid2d::UGrid2D;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Elem {
    Open,
    Obstacle,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct GuardState {
    pos: UCoord2D,
    dir: Direction,
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

    let mut guardpos = None;

    let grid = UGrid2D::generate(dim, |UCoord2D { x, y }| {
        match input[y]
            .chars()
            .nth(x)
            .ok_or(util::error::Errors::DimError(format!("{x} is too high")))?
        {
            '#' => Ok(Elem::Obstacle),
            '.' => Ok(Elem::Open),
            '^' => {
                guardpos = Some(UCoord2D { x, y });
                Ok(Elem::Open)
            }
            _ => panic!("Unexpected map element"),
        }
    })?;

    let guard_pos = guardpos.unwrap();

    {
        let mut guard_pos = guard_pos;
        let mut guard_dir = Direction::North;
        let mut visited_positions = HashSet::new();

        loop {
            visited_positions.insert(guard_pos);

            let offset = guard_dir.to_offset();
            let new_pos = guard_pos + offset;
            match new_pos {
                ICoord2D { x, y: _ } if x < 0 => break,
                ICoord2D { x: _, y } if y < 0 => break,
                ICoord2D { x, y: _ } if x >= grid.dim().x as isize => break,
                ICoord2D { x: _, y } if y >= grid.dim().y as isize => break,
                _ => (),
            };

            let new_pos: UCoord2D = new_pos.try_into()?;
            if *grid.get(new_pos)? == Elem::Obstacle {
                guard_dir = guard_dir.rotate_right();
                continue;
            }

            guard_pos = new_pos;
        }
        println!("{}", visited_positions.len());
    }

    {
        let mut result2 = 0u32;
        for iy in 0..grid.dim().y {
            for ix in 0..grid.dim().x {
                let mut grid = grid.clone();
                let c_coord = UCoord2D { x: ix, y: iy };
                if *grid.get(c_coord)? == Elem::Obstacle {
                    continue;
                }
                if c_coord == guard_pos {
                    continue;
                }
                *grid.get_mut(c_coord)? = Elem::Obstacle;
                let grid = grid;

                let mut guard_state = GuardState {
                    pos: guard_pos,
                    dir: Direction::North,
                };

                let mut guard_states = HashSet::new();

                loop {
                    if guard_states.contains(&guard_state) {
                        result2 += 1;
                        break;
                    }
                    guard_states.insert(guard_state);

                    let offset = guard_state.dir.to_offset();
                    let new_pos = guard_state.pos + offset;
                    match new_pos {
                        ICoord2D { x, y: _ } if x < 0 => break,
                        ICoord2D { x: _, y } if y < 0 => break,
                        ICoord2D { x, y: _ } if x >= grid.dim().x as isize => break,
                        ICoord2D { x: _, y } if y >= grid.dim().y as isize => break,
                        _ => (),
                    };

                    let new_pos: UCoord2D = new_pos.try_into()?;
                    if *grid.get(new_pos)? == Elem::Obstacle {
                        guard_state.dir = guard_state.dir.rotate_right();
                        continue;
                    }

                    guard_state.pos = new_pos;
                }
            }
        }
        println!("{}", result2);
    }

    Ok(())
}
