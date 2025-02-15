use std::collections::HashSet;
use util::coord2d::ICoord2D;
use util::direction::Direction;
use util::error::Errors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seq = std::fs::read_to_string("input")?
        .trim_end()
        .chars()
        .map(|c| match c {
            '<' => Ok(Direction::West),
            '>' => Ok(Direction::East),
            '^' => Ok(Direction::North),
            'v' => Ok(Direction::South),
            _ => Err(Errors::ParseError),
        })
        .collect::<Result<Box<_>, util::error::Errors>>()?;

    let part1 = {
        let mut visited_locations = HashSet::new();
        let mut location = ICoord2D { x: 0, y: 0 };

        visited_locations.insert(location);

        for &dir in &seq {
            location += dir.to_offset();
            visited_locations.insert(location);
        }

        visited_locations.len()
    };
    println!("{}", part1);

    let part2 = {
        let mut visited_locations = HashSet::new();
        let mut locations = [ICoord2D { x: 0, y: 0 }; 2];

        visited_locations.insert(locations[0]);

        let mut element = 0;
        for &dir in &seq {
            let location = &mut locations[element];
            *location += dir.to_offset();
            visited_locations.insert(*location);

            element = (element + 1) % 2;
        }

        visited_locations.len()
    };
    println!("{}", part2);

    Ok(())
}
