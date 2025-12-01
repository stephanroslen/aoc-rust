use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use util::coord2d::{ICoord2D, UCoord2D};
use util::direction::Direction;

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

    let fields = {
        let mut data: HashMap<ICoord2D, char> = HashMap::new();

        for iy in 0..dim.y {
            for ix in 0..dim.x {
                let coord = ICoord2D {
                    x: ix as isize,
                    y: iy as isize,
                };
                let val = input
                    .get(iy)
                    .ok_or(util::error::Errors::DimError(format!("iy ({iy}) invalid")))?
                    .chars()
                    .nth(ix)
                    .ok_or(util::error::Errors::DimError(format!("ix ({ix}) invalid")))?;
                data.insert(coord, val);
            }
        }

        let mut shrink_data = data;

        let mut fields: Vec<HashSet<ICoord2D>> = Vec::new();
        while !shrink_data.is_empty() {
            let mut queue = VecDeque::new();
            let mut accum = HashSet::new();

            let (start_coord, start_char) = shrink_data
                .iter().next()
                .map(|(coord, char)| (*coord, *char))
                .expect("Map expected to have first value");

            shrink_data.remove(&start_coord);
            accum.insert(start_coord);
            queue.push_back(start_coord);

            while let Some(candidate_coord) = queue.pop_front() {
                for dir in Direction::directions() {
                    let offset = dir.to_offset();
                    let adjacent_coord = candidate_coord + offset;
                    if let Some(adjacent_char) = shrink_data.get(&adjacent_coord).copied() {
                        if adjacent_char == start_char {
                            shrink_data.remove(&adjacent_coord);
                            accum.insert(adjacent_coord);
                            queue.push_back(adjacent_coord);
                        }
                    }
                }
            }
            fields.push(accum);
        }
        fields
    };

    let result1: usize = fields
        .iter()
        .map(|field| {
            field
                .iter()
                .map(|coord| {
                    {
                        Direction::directions()
                            .iter()
                            .filter(|dir| !field.contains(&(*coord + dir.to_offset())))
                            .count()
                    }
                })
                .sum::<usize>()
                * field.len()
        })
        .sum();

    println!("{}", result1);

    let result2: usize = fields
        .iter()
        .map(|field| {
            let area = field.len();
            let mut fences: BTreeSet<(ICoord2D, Direction)> = BTreeSet::new();
            let mut sides = 0usize;
            for inside_field in field {
                for candidate_outside_field_dir in Direction::directions() {
                    let offset = candidate_outside_field_dir.to_offset();
                    let candidate_outside_field = *inside_field + offset;
                    if !field.contains(&candidate_outside_field) {
                        fences.insert((*inside_field, candidate_outside_field_dir));
                    }
                }
            }

            while let Some(fence) = fences.first().copied() {
                fences.remove(&fence);

                let get_next_fence = |from_fence: (ICoord2D, Direction),
                                      fences: &mut BTreeSet<(ICoord2D, Direction)>|
                 -> Option<(ICoord2D, Direction)> {
                    let (from_coord, open_side) = from_fence;
                    let fence_pointing = match open_side {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::South,
                    };
                    let new_coord = from_coord + fence_pointing.to_offset();
                    let result = (new_coord, open_side);
                    if fences.contains(&result) {
                        Some(result)
                    } else {
                        None
                    }
                };

                let mut cand = get_next_fence(fence, &mut fences);
                while let Some(found) = cand {
                    fences.remove(&found);
                    cand = get_next_fence(found, &mut fences);
                }
                sides += 1;
            }
            area * sides
        })
        .sum();

    println!("{}", result2);

    Ok(())
}
