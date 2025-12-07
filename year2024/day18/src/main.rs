use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::num::ParseIntError;
use util::coord2d::UCoord2D;
use util::direction::Direction;
use util::error::Errors;
use util::grid2d::UGrid2D;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
enum Field {
    #[default]
    Open,
    Closed,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let raw_lines = raw_input.trim_end().split('\n').collect::<Vec<_>>();

    let coords = raw_lines
        .iter()
        .map(|line| -> Result<UCoord2D, Errors> {
            let coord_vals = line
                .split(',')
                .map(|num| -> Result<usize, ParseIntError> { num.parse::<usize>() })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(UCoord2D {
                x: *coord_vals.first().ok_or(Errors::ParseError)?,
                y: *coord_vals.get(1).ok_or(Errors::ParseError)?,
            })
        })
        .collect::<Result<Vec<_>, Errors>>()?;

    let dim = UCoord2D {
        x: coords
            .iter()
            .map(|&UCoord2D { x, y: _ }| x)
            .max()
            .expect("Values expected in input")
            + 1,
        y: coords
            .iter()
            .map(|&UCoord2D { x: _, y }| y)
            .max()
            .expect("Values expected in input")
            + 1,
    };

    {
        let n = 1024usize;

        let result1 = calculate_cost(make_work_grid(dim, coords.iter().take(n))?)?;

        println!("{}", result1);
    }

    {
        let mut costs: Vec<Option<usize>> = vec![None; coords.len()];
        let mut min = 1;
        let mut max = coords.len() - 1;

        let solution_part2 = loop {
            let candidate = min + (max - min) / 2;
            {
                let cost_cadidate = &mut costs[candidate];
                let cost_candidate = cost_cadidate.get_or_insert_with(|| {
                    calculate_cost(
                        make_work_grid(dim, coords.iter().take(candidate + 1))
                            .expect("Valid grid expected"),
                    )
                    .expect("Cost expected")
                });

                if *cost_candidate < usize::MAX {
                    min = candidate + 1;
                    continue;
                }
            }
            {
                let cost_prev = &mut costs[candidate - 1];
                let cost_prev = cost_prev.get_or_insert_with(|| {
                    calculate_cost(
                        make_work_grid(dim, coords.iter().take(candidate))
                            .expect("Valid grid expected"),
                    )
                    .expect("Cost expected")
                });

                if *cost_prev == usize::MAX {
                    max = candidate - 1;
                    continue;
                }
            }

            break coords[candidate];
        };

        println!("{},{}", solution_part2.x, solution_part2.y);
    }

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct PathSearchCandidate {
    coord: UCoord2D,
    cost: usize,
}

impl PartialEq for PathSearchCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for PathSearchCandidate {}

impl PartialOrd for PathSearchCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathSearchCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

fn calculate_cost(grid: UGrid2D<Field>) -> Result<usize, Errors> {
    let dim = grid.dim();
    let start = dim - UCoord2D { x: 1, y: 1 };

    let mut visit_queue = BinaryHeap::from([PathSearchCandidate {
        coord: start,
        cost: 0,
    }]);
    let mut cost_grid = UGrid2D::generate(dim, |_| Ok(usize::MAX))?;

    while let Some(PathSearchCandidate { coord, cost }) = visit_queue.pop() {
        if *grid.get(coord)? == Field::Closed {
            continue;
        }
        let cost_on_map_mut = cost_grid.get_mut(coord)?;
        if cost < *cost_on_map_mut {
            *cost_on_map_mut = cost;
            for dir in Direction::directions() {
                let offset = dir.to_offset();
                let candidate = coord + offset;
                if let Some(candidate) = grid.coord_to_grid(candidate) {
                    visit_queue.push(PathSearchCandidate {
                        coord: candidate,
                        cost: cost + 1,
                    });
                }
            }
        }
    }

    let &new_cost = cost_grid.get(UCoord2D { x: 0, y: 0 })?;
    Ok(new_cost)
}

fn make_work_grid<'a>(
    dim: UCoord2D,
    coords: impl Iterator<Item = &'a UCoord2D>,
) -> Result<UGrid2D<Field>, Errors> {
    let mut result = UGrid2D::from_default(dim);

    for &coord in coords {
        *result.get_mut(coord)? = Field::Closed;
    }

    Ok(result)
}
