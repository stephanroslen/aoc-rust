use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use util::coord2d::{ICoord2D, UCoord2D};
use util::direction::Direction;
use util::error::Errors;
use util::grid2d::UGrid2D;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MapElem {
    Space,
    Wall,
}

#[derive(Clone, Debug)]
struct World {
    map: UGrid2D<MapElem>,
    box_positions: HashSet<ICoord2D>,
    robot_position: ICoord2D,
}

impl World {
    fn transform_part2(&self) -> Result<World, Errors> {
        let mut dim = self.map.dim();
        dim.x *= 2;

        let map = UGrid2D::generate(dim, |UCoord2D { x, y }| {
            Ok(*(self.map.get(UCoord2D { x: x / 2, y })?))
        })?;

        let box_positions = self
            .box_positions
            .iter()
            .map(|&ICoord2D { x, y }| ICoord2D { x: x * 2, y })
            .collect::<HashSet<_>>();

        let mut robot_position = self.robot_position;
        robot_position.x *= 2;

        Ok(Self {
            map,
            box_positions,
            robot_position,
        })
    }

    fn simulate_part1(&mut self, dir: Direction) -> Result<(), Errors> {
        let offset = dir.to_offset();

        let mut check = self.robot_position + offset;
        while self.box_positions.contains(&check) {
            check = check + offset;
        }

        if *self.map.get(
            self.map
                .coord_to_grid(check)
                .ok_or(Errors::DimError("Coord outside map".into()))?,
        )? == MapElem::Wall
        {
            return Ok(());
        }

        let move_to = self.robot_position + offset;
        if self.box_positions.remove(&move_to) {
            self.box_positions.insert(check);
        }
        self.robot_position = move_to;

        Ok(())
    }

    fn simulate_part2(&mut self, dir: Direction) -> Result<(), Errors> {
        let move_offset = dir.to_offset();

        if *self.map.get(
            self.map
                .coord_to_grid(self.robot_position + move_offset)
                .ok_or(Errors::DimError("Invalid coordinate for map".into()))?,
        )? == MapElem::Wall
        {
            return Ok(());
        }

        let move_check_box_offsets = match dir {
            Direction::North | Direction::South => {
                vec![move_offset, move_offset + Direction::West.to_offset()]
            }
            Direction::East => vec![move_offset],
            Direction::West => vec![move_offset * 2],
        };

        let box_check_box_offsets = match dir {
            Direction::West | Direction::East => vec![dir.to_offset() * 2],
            Direction::North | Direction::South => vec![
                dir.to_offset(),
                dir.to_offset() + Direction::West.to_offset(),
                dir.to_offset() + Direction::East.to_offset(),
            ],
        };

        let box_check_wall_offsets = match dir {
            Direction::West => vec![dir.to_offset()],
            Direction::East => vec![dir.to_offset() * 2],
            Direction::North | Direction::South => vec![
                dir.to_offset(),
                dir.to_offset() + Direction::East.to_offset(),
            ],
        };

        let mut box_check_queue = VecDeque::new();

        for &o in &move_check_box_offsets {
            box_check_queue.push_back(self.robot_position + o);
        }

        let mut push_boxes = HashSet::new();

        while let Some(box_check_coord) = box_check_queue.pop_front() {
            if self.box_positions.contains(&box_check_coord) {
                for &o in &box_check_wall_offsets {
                    if *self.map.get(
                        self.map
                            .coord_to_grid(box_check_coord + o)
                            .ok_or(Errors::DimError("Invalid coordinate for map".into()))?,
                    )? == MapElem::Wall
                    {
                        return Ok(());
                    }
                }
                push_boxes.insert(box_check_coord);
                for &o in &box_check_box_offsets {
                    box_check_queue.push_back(box_check_coord + o);
                }
            }
        }

        for &o in &push_boxes {
            self.box_positions.remove(&o);
        }
        for &o in &push_boxes {
            self.box_positions.insert(o + move_offset);
        }

        self.robot_position = self.robot_position + move_offset;

        Ok(())
    }
}

fn to_gps(ICoord2D { x, y }: ICoord2D) -> usize {
    x as usize + 100 * y as usize
}

fn result(world: &World) -> usize {
    world
        .box_positions
        .iter()
        .map(|coord| to_gps(*coord))
        .sum::<usize>()
}

impl FromStr for World {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').collect::<Vec<_>>();

        let dim = UCoord2D {
            x: lines[0].len(),
            y: lines.len(),
        };

        let mut robot_position = ICoord2D::default();
        let mut box_positions = HashSet::new();

        let map = UGrid2D::generate(dim, |UCoord2D { x, y }| {
            match lines[y]
                .chars()
                .nth(x)
                .ok_or(Errors::DimError("Invalid index for string".into()))?
            {
                '#' => Ok(MapElem::Wall),
                '.' => Ok(MapElem::Space),
                'O' => {
                    box_positions.insert(ICoord2D {
                        x: x as isize,
                        y: y as isize,
                    });
                    Ok(MapElem::Space)
                }
                '@' => {
                    robot_position = ICoord2D {
                        x: x as isize,
                        y: y as isize,
                    };
                    Ok(MapElem::Space)
                }
                _ => Err(Errors::UncategorizedError("Invalid map input".into())),
            }
        })?;

        Ok(Self {
            map,
            robot_position,
            box_positions,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let input: Vec<_> = raw_input.trim_end().split("\n\n").collect();

    let world = input[0].parse::<World>()?;
    let input_directions = input[1]
        .split("\n")
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .map(|c| match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>();

    {
        let mut world_part1 = world.clone();
        for &dir in &input_directions {
            world_part1.simulate_part1(dir)?;
        }
        println!("{}", result(&world_part1));
    }

    {
        let mut world_part2 = world.transform_part2()?;
        for &dir in &input_directions {
            world_part2.simulate_part2(dir)?;
        }
        println!("{}", result(&world_part2));
    }

    Ok(())
}
