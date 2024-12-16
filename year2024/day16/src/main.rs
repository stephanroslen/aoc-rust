use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use util::coord2d::{ICoord2D, UCoord2D};
use util::direction::Direction;
use util::error::Errors;
use util::grid2d::UGrid2D;

#[derive(Copy, Clone, Debug, PartialEq)]
enum MapElem {
    Space,
    Wall,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct ReindeerState {
    position: ICoord2D,
    direction: Direction,
}

impl ReindeerState {
    fn next_states(&self) -> [ReindeerCheckState; 3] {
        [
            ReindeerCheckState {
                state: ReindeerState {
                    position: self.position + self.direction.to_offset(),
                    direction: self.direction,
                },
                cost: 1,
            },
            ReindeerCheckState {
                state: ReindeerState {
                    position: self.position,
                    direction: self.direction.rotate_left(),
                },
                cost: 1000,
            },
            ReindeerCheckState {
                state: ReindeerState {
                    position: self.position,
                    direction: self.direction.rotate_right(),
                },
                cost: 1000,
            },
        ]
    }

    fn prev_states(&self) -> [ReindeerCheckState; 3] {
        [
            ReindeerCheckState {
                state: ReindeerState {
                    position: self.position - self.direction.to_offset(),
                    direction: self.direction,
                },
                cost: 1,
            },
            ReindeerCheckState {
                state: ReindeerState {
                    position: self.position,
                    direction: self.direction.rotate_right(),
                },
                cost: 1000,
            },
            ReindeerCheckState {
                state: ReindeerState {
                    position: self.position,
                    direction: self.direction.rotate_left(),
                },
                cost: 1000,
            },
        ]
    }
}

#[derive(Copy, Clone, Debug)]
struct ReindeerCheckState {
    state: ReindeerState,
    cost: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<_> = std::fs::read_to_string("input")?
        .trim_end()
        .split("\n")
        .map(|s| s.to_owned())
        .collect();

    let (start, end, map) = {
        let mut start = ICoord2D::default();
        let mut end = ICoord2D::default();

        let dim = UCoord2D {
            x: input[0].len(),
            y: input.len(),
        };

        let map = UGrid2D::generate(dim, |UCoord2D { x, y }| {
            let c = input[y]
                .chars()
                .nth(x)
                .ok_or(Errors::DimError("Invalid dimension".into()))?;
            match c {
                '.' => Ok(MapElem::Space),
                '#' => Ok(MapElem::Wall),
                'S' => {
                    start = ICoord2D {
                        x: x as isize,
                        y: y as isize,
                    };
                    Ok(MapElem::Space)
                }
                'E' => {
                    end = ICoord2D {
                        x: x as isize,
                        y: y as isize,
                    };
                    Ok(MapElem::Space)
                }
                _ => Err(Errors::UncategorizedError(
                    "Invalid map input character".into(),
                )),
            }
        })?;
        (start, end, map)
    };

    let reindeer_start = ReindeerState {
        position: start,
        direction: Direction::East,
    };

    let state_map = {
        let mut state_map = HashMap::new();
        let mut check_queue = VecDeque::new();

        state_map.insert(reindeer_start, 0usize);
        for possible_next_state in reindeer_start.next_states() {
            check_queue.push_back(possible_next_state);
        }

        while let Some(next_state) = check_queue.pop_front() {
            if *map.get(next_state.state.position)? == MapElem::Wall {
                continue;
            }

            let mut update = false;
            if let Some(cost_ref) = state_map.get_mut(&next_state.state) {
                if next_state.cost < *cost_ref {
                    *cost_ref = next_state.cost;
                    update = true;
                }
            } else {
                state_map.insert(next_state.state, next_state.cost);
                update = true;
            }

            if update {
                for possible_next_state in next_state.state.next_states() {
                    check_queue.push_back(ReindeerCheckState {
                        state: possible_next_state.state,
                        cost: next_state.cost + possible_next_state.cost,
                    });
                }
            }
        }

        state_map
    };

    let end_states = Direction::directions().map(|dir| ReindeerState {
        position: end,
        direction: dir,
    });

    let result1 = end_states
        .map(|state| *state_map.get(&state).expect("Endstate expected present"))
        .iter()
        .copied()
        .min()
        .expect("Array with 4 elements expected to have a minimum");

    println!("{}", result1);

    let on_winning_paths = {
        let mut on_winning_paths = HashSet::new();
        on_winning_paths.insert(start);
        on_winning_paths.insert(end);

        let mut backtrack_queue = VecDeque::new();

        for end_state in end_states {
            let end_state_cost = *state_map
                .get(&end_state)
                .expect("Endstate expected present");
            if end_state_cost == result1 {
                for possible_prev_state in end_state.prev_states() {
                    backtrack_queue.push_back(ReindeerCheckState {
                        state: possible_prev_state.state,
                        cost: end_state_cost - possible_prev_state.cost,
                    });
                }
            }
        }

        while let Some(prev_state) = backtrack_queue.pop_front() {
            if *map.get(prev_state.state.position)? == MapElem::Wall {
                continue;
            }

            if let Some(&cost) = state_map.get(&prev_state.state) {
                if prev_state.cost == cost {
                    on_winning_paths.insert(prev_state.state.position);
                    for possible_prev_state in prev_state.state.prev_states() {
                        let (new_cost, overflow) =
                            prev_state.cost.overflowing_sub(possible_prev_state.cost);
                        if overflow {
                            continue;
                        }
                        backtrack_queue.push_back(ReindeerCheckState {
                            state: possible_prev_state.state,
                            cost: new_cost,
                        });
                    }
                }
            }
        }

        on_winning_paths
    };

    let result2 = on_winning_paths.len();

    println!("{}", result2);

    Ok(())
}
