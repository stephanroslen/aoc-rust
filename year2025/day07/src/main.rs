use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use util::coord2d::ICoord2D;
use util::error::Errors;
use util::grid2d::{UCoord2D, UGrid2D};

#[derive(Debug, Clone, Copy)]
enum Element {
    Splitter,
    Empty,
}

#[derive(Debug, Clone)]
struct Field {
    ugrid2d: UGrid2D<Element>,
    start: UCoord2D,
}

impl FromStr for Field {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').collect::<Vec<_>>();

        let dim = UCoord2D {
            x: lines[0].len(),
            y: lines.len(),
        };

        let mut start = UCoord2D { x: 0, y: 0 };
        let ugrid2d = UGrid2D::generate(dim, |UCoord2D { x, y }| match &lines[y][x..=x] {
            "." => Ok(Element::Empty),
            "^" => Ok(Element::Splitter),
            "S" => {
                start = UCoord2D { x, y };
                Ok(Element::Empty)
            }
            _ => Err(Errors::ParseError),
        })?;

        Ok(Field { ugrid2d, start })
    }
}

#[derive(Debug, Copy, Clone)]
enum BeamOutcomeReason {
    Terminate,
    Split,
}

#[derive(Debug, Copy, Clone)]
struct BeamOutcome {
    reason: BeamOutcomeReason,
    pos: UCoord2D,
}

impl BeamOutcome {
    fn new_terminate_at(pos: UCoord2D) -> Self {
        Self {
            reason: BeamOutcomeReason::Terminate,
            pos,
        }
    }

    fn new_split_at(pos: UCoord2D) -> Self {
        Self {
            reason: BeamOutcomeReason::Split,
            pos,
        }
    }
}

impl Field {
    fn simulate_beam(&self, beam_start: UCoord2D) -> Result<BeamOutcome, Errors> {
        let mut beam_pos = beam_start;

        loop {
            beam_pos += UCoord2D { x: 0, y: 1 };

            let beam_pos_on_grid = self.ugrid2d.coord_to_grid(beam_pos);

            match beam_pos_on_grid {
                None => return Ok(BeamOutcome::new_terminate_at(beam_pos)),
                Some(beam_pos_on_grid) => {
                    let element = self.ugrid2d.get(beam_pos_on_grid)?;
                    match element {
                        Element::Splitter => {
                            return Ok(BeamOutcome::new_split_at(beam_pos_on_grid));
                        }
                        Element::Empty => continue,
                    }
                }
            }
        }
    }

    fn result(&self) -> Result<(usize, usize), Errors> {
        #[derive(Debug, Copy, Clone)]
        struct OutcomePathDetail {
            path_count: usize,
            reason: BeamOutcomeReason,
        }

        let mut outcome_paths = HashMap::new();

        #[derive(Eq, PartialEq, Debug, Copy, Clone)]
        struct FringeElement {
            beam_start: UCoord2D,
            beam_cause: UCoord2D,
        }

        impl PartialOrd for FringeElement {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for FringeElement {
            fn cmp(&self, other: &Self) -> Ordering {
                other.beam_start.y.cmp(&self.beam_start.y)
            }
        }

        let mut fringe = BinaryHeap::from([FringeElement {
            beam_start: self.start,
            beam_cause: self.start,
        }]);

        while let Some(FringeElement {
            beam_start,
            beam_cause,
        }) = fringe.pop()
        {
            let BeamOutcome { pos, reason } = self.simulate_beam(beam_start)?;

            let was_unseen = !outcome_paths.contains_key(&pos);

            let addition = outcome_paths
                .get(&beam_cause)
                .unwrap_or(&OutcomePathDetail {
                    path_count: 1,
                    reason: BeamOutcomeReason::Split,
                })
                .path_count;

            outcome_paths
                .entry(pos)
                .or_insert(OutcomePathDetail {
                    path_count: 0,
                    reason,
                })
                .path_count += addition;

            if matches!(reason, BeamOutcomeReason::Split) && was_unseen {
                let left = self
                    .ugrid2d
                    .coord_to_grid(pos + ICoord2D { x: -1, y: 0 })
                    .ok_or(Errors::UncategorizedError("Not on grid".into()))?;
                let right = self
                    .ugrid2d
                    .coord_to_grid(pos + ICoord2D { x: 1, y: 0 })
                    .ok_or(Errors::UncategorizedError("Not on grid".into()))?;

                fringe.push(FringeElement {
                    beam_start: left,
                    beam_cause: pos,
                });
                fringe.push(FringeElement {
                    beam_start: right,
                    beam_cause: pos,
                });
            }
        }

        let splits = outcome_paths
            .iter()
            .filter(
                |(
                    _,
                    OutcomePathDetail {
                        path_count: _,
                        reason,
                    },
                )| matches!(reason, BeamOutcomeReason::Split),
            )
            .count();

        let worlds = outcome_paths
            .iter()
            .map(
                |(_, OutcomePathDetail { path_count, reason })| match reason {
                    BeamOutcomeReason::Split => 0,
                    BeamOutcomeReason::Terminate => *path_count,
                },
            )
            .sum();

        Ok((splits, worlds))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let field = std::fs::read_to_string("input")?
        .trim_end()
        .parse::<Field>()?;

    let (result_part1, result_part2) = field.result()?;

    println!("{result_part1}");
    println!("{result_part2}");

    Ok(())
}
