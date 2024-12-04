use std::usize;
use util::coord2d::UCoord2D;
use util::error::Errors;
use util::grid2d::UGrid2D;

#[derive(PartialEq)]
enum Found {
    N,
    X,
    XM,
    XMA,
    XMAS,
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

    let grid = UGrid2D::generate(dim, |UCoord2D { x, y }| {
        input[y]
            .chars()
            .nth(x)
            .ok_or(Errors::UncategorizedError(format!(
                "Couldn't access character number {} in input line",
                x
            )))
    })?;

    let result1 = {
        let mut result = usize::default();
        let mut grid = grid.clone();

        for _ in 0..4 {
            grid = grid.rotate_left();
            for iy in 0..grid.dim().y {
                let mut found = Found::N;
                for ix in 0..grid.dim().x {
                    let coord = UCoord2D { x: ix, y: iy };
                    let c = grid.get(coord)?;
                    found = match (found, c) {
                        (_, 'X') => Found::X,
                        (Found::X, 'M') => Found::XM,
                        (Found::XM, 'A') => Found::XMA,
                        (Found::XMA, 'S') => Found::XMAS,
                        _ => Found::N,
                    };
                    if found == Found::XMAS {
                        result += 1;
                    }
                }
                found = Found::N;
                for ix in 0..grid.dim().x {
                    let cy = ix + iy;
                    if cy >= grid.dim().y {
                        break;
                    }
                    let coord = UCoord2D { x: ix, y: cy };
                    let c = grid.get(coord)?;
                    found = match (found, c) {
                        (_, 'X') => Found::X,
                        (Found::X, 'M') => Found::XM,
                        (Found::XM, 'A') => Found::XMA,
                        (Found::XMA, 'S') => Found::XMAS,
                        _ => Found::N,
                    };
                    if found == Found::XMAS {
                        result += 1;
                    }
                }
            }
            for ix in 1..grid.dim().x {
                let mut found = Found::N;
                for iy in 0..grid.dim().y {
                    let cx = ix + iy;
                    if cx >= grid.dim().x {
                        break;
                    }
                    let coord = UCoord2D { x: cx, y: iy };
                    let c = grid.get(coord)?;
                    found = match (found, c) {
                        (_, 'X') => Found::X,
                        (Found::X, 'M') => Found::XM,
                        (Found::XM, 'A') => Found::XMA,
                        (Found::XMA, 'S') => Found::XMAS,
                        _ => Found::N,
                    };
                    if found == Found::XMAS {
                        result += 1;
                    }
                }
            }
        }
        result
    };

    println!("{}", result1);

    let result2 = {
        let mut result = usize::default();

        for iy in 0..grid.dim().y - 2 {
            for ix in 0..grid.dim().x - 2 {
                let mut subgrid =
                    grid.sub_grid(UCoord2D { x: ix, y: iy }, UCoord2D { x: 3, y: 3 })?;
                for _ in 0..4 {
                    subgrid = subgrid.rotate_left();
                    if *subgrid.get(UCoord2D { x: 0, y: 0 })? == 'M'
                        && *subgrid.get(UCoord2D { x: 0, y: 2 })? == 'M'
                        && *subgrid.get(UCoord2D { x: 1, y: 1 })? == 'A'
                        && *subgrid.get(UCoord2D { x: 2, y: 0 })? == 'S'
                        && *subgrid.get(UCoord2D { x: 2, y: 2 })? == 'S'
                    {
                        result += 1;
                    }
                }
            }
        }
        result
    };

    println!("{}", result2);

    Ok(())
}
