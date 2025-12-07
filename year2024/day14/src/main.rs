use regex::Regex;
use std::cmp::max;
use std::str::FromStr;
use std::sync::OnceLock;
use util::coord2d::ICoord2D;
use util::error::Errors;
use util::grid2d::{UCoord2D, UGrid2D};

#[derive(Copy, Clone, Debug, Default)]
struct Robot {
    p: ICoord2D,
    v: ICoord2D,
}

impl FromStr for Robot {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot = Robot::default();
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = REGEX.get_or_init(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

        let cpt = regex.captures(s).ok_or(Errors::ParseError)?;

        robot.p.x = cpt[1].parse()?;
        robot.p.y = cpt[2].parse()?;
        robot.v.x = cpt[3].parse()?;
        robot.v.y = cpt[4].parse()?;

        Ok(robot)
    }
}

fn robot_position_after(dim: ICoord2D, robot: &Robot, steps: usize) -> ICoord2D {
    let p_congruent = robot.p + robot.v * steps as isize;
    ICoord2D {
        x: p_congruent.x.rem_euclid(dim.x),
        y: p_congruent.y.rem_euclid(dim.y),
    }
}

fn robot_positions_after(
    dim: ICoord2D,
    robots: &Vec<Robot>,
    steps: usize,
) -> impl Iterator<Item = ICoord2D> + Clone + use<'_> {
    robots
        .iter()
        .map(move |robot| robot_position_after(dim, robot, steps))
}

fn result1(dim: ICoord2D, robot_positions: impl Iterator<Item = ICoord2D>) -> usize {
    let pivot = ICoord2D {
        x: (dim.x - 1) / 2,
        y: (dim.y - 1) / 2,
    };

    let mut counts = [0usize; 4];

    for p in robot_positions {
        match p {
            ICoord2D { x, y } if x < pivot.x && y < pivot.y => counts[0] += 1,
            ICoord2D { x, y } if x > pivot.x && y < pivot.y => counts[1] += 1,
            ICoord2D { x, y } if x < pivot.x && y > pivot.y => counts[2] += 1,
            ICoord2D { x, y } if x > pivot.x && y > pivot.y => counts[3] += 1,
            _ => {}
        }
    }

    counts.iter().product()
}

fn print_map(dim: ICoord2D, robot_positions: impl Iterator<Item = ICoord2D>) -> Result<(), Errors> {
    let mut map = UGrid2D::<u32>::from_default(dim.try_into()?);

    for p in robot_positions {
        let pg = map
            .coord_to_grid(p)
            .ok_or(Errors::DimError("Coord outside grid".into()))?;
        *map.get_mut(pg)? += 1;
    }

    for iy in 0..dim.y as usize {
        for ix in 0..dim.x as usize {
            print!(
                "{}",
                match *map.get(UCoord2D { x: ix, y: iy })? {
                    0 => '.',
                    _ => '#',
                }
            );
        }
        println!();
    }

    Ok(())
}

fn robot_positions_var(robot_positions: impl Iterator<Item = ICoord2D> + Clone) -> f32 {
    let mut avg_x = 0f32;
    let mut avg_y = 0f32;

    let positions: Vec<_> = robot_positions.collect();

    for &ICoord2D { x, y } in &positions {
        avg_x += x as f32;
        avg_y += y as f32;
    }
    avg_x /= positions.len() as f32;
    avg_y /= positions.len() as f32;

    let mut var_x = 0f32;
    let mut var_y = 0f32;

    for &ICoord2D { x, y } in &positions {
        var_x += (x as f32 - avg_x) * (x as f32 - avg_x);
        var_y += (y as f32 - avg_y) * (y as f32 - avg_y);
    }
    var_x /= positions.len() as f32;
    var_y /= positions.len() as f32;

    (var_x + var_y).sqrt()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let robots = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|s| s.parse::<Robot>())
        .collect::<Result<Vec<_>, _>>()?;

    let dim = {
        let mut dim = ICoord2D::default();
        for robot in &robots {
            let dim_candidate = robot.p + ICoord2D { x: 1, y: 1 };
            dim.x = max(dim.x, dim_candidate.x);
            dim.y = max(dim.y, dim_candidate.y);
        }
        dim
    };

    println!("{}", result1(dim, robot_positions_after(dim, &robots, 100)));

    let mut min_var = f32::MAX;
    let mut cnt_at_min_var = 0usize;

    for i in 0..dim.x * dim.y {
        let var = robot_positions_var(robot_positions_after(dim, &robots, i as usize + 1));

        if var < min_var {
            min_var = var;
            cnt_at_min_var = i as usize + 1;
        }
    }

    print_map(dim, robot_positions_after(dim, &robots, cnt_at_min_var))?;
    println!("{}", cnt_at_min_var);

    Ok(())
}
