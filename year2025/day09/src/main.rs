use std::str::FromStr;
use util::coord2d::ICoord2D;
use util::error::Errors;

#[derive(Debug, Copy, Clone)]
struct Tile {
    icoord2d: ICoord2D,
}

impl FromStr for Tile {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems = s
            .split(',')
            .map(|e| e.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

        if elems.len() != 2 {
            return Err(Errors::ParseError);
        }

        Ok(Self {
            icoord2d: ICoord2D {
                x: elems[0],
                y: elems[1],
            },
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    from: Tile,
    to: Tile,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: ICoord2D,
    end: ICoord2D,
}

#[derive(Debug, Copy, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
    Degenerate,
}

impl Line {
    fn intersects(&self, other: &Line) -> bool {
        match (self.orientation(), other.orientation()) {
            (Orientation::Degenerate, _) => other.contains_point(self.start),
            (_, Orientation::Degenerate) => self.contains_point(other.start),
            (Orientation::Horizontal, Orientation::Horizontal) => {
                if self.start.y != other.start.y {
                    return false;
                }

                let self_min_x = self.start.x.min(self.end.x);
                let self_max_x = self.start.x.max(self.end.x);

                let other_min_x = other.start.x.min(other.end.x);
                let other_max_x = other.start.x.max(other.end.x);

                let max_of_min_x = self_min_x.max(other_min_x);
                let min_of_max_x = self_max_x.min(other_max_x);

                max_of_min_x <= min_of_max_x
            }
            (Orientation::Vertical, Orientation::Vertical) => {
                if self.start.x != other.start.x {
                    return false;
                }

                let self_min_y = self.start.y.min(self.end.y);
                let self_max_y = self.start.y.max(self.end.y);

                let other_min_y = other.start.y.min(other.end.y);
                let other_max_y = other.start.y.max(other.end.y);

                let max_of_min_y = self_min_y.max(other_min_y);
                let min_of_max_y = self_max_y.min(other_max_y);

                max_of_min_y <= min_of_max_y
            }
            (Orientation::Horizontal, Orientation::Vertical) => {
                let x_min = self.start.x.min(self.end.x);
                let x_max = self.start.x.max(self.end.x);
                let y_min = other.start.y.min(other.end.y);
                let y_max = other.start.y.max(other.end.y);

                (x_min..=x_max).contains(&other.start.x) && (y_min..=y_max).contains(&self.start.y)
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                let x_min = other.start.x.min(other.end.x);
                let x_max = other.start.x.max(other.end.x);
                let y_min = self.start.y.min(self.end.y);
                let y_max = self.start.y.max(self.end.y);

                (x_min..=x_max).contains(&self.start.x) && (y_min..=y_max).contains(&other.start.y)
            }
        }
    }

    fn orientation(&self) -> Orientation {
        if self.start.x == self.end.x {
            if self.start.y == self.end.y {
                Orientation::Degenerate
            } else {
                Orientation::Vertical
            }
        } else {
            debug_assert_eq!(self.start.y, self.end.y);
            Orientation::Horizontal
        }
    }

    fn same_orientation_or_either_is_degenerate(&self, other: &Line) -> bool {
        matches!(
            (self.orientation(), other.orientation()),
            (Orientation::Degenerate, _)
                | (_, Orientation::Degenerate)
                | (Orientation::Horizontal, Orientation::Horizontal)
                | (Orientation::Vertical, Orientation::Vertical)
        )
    }

    fn contains_point(&self, point: ICoord2D) -> bool {
        match self.orientation() {
            Orientation::Horizontal => {
                let x_min = self.start.x.min(self.end.x);
                let x_max = self.start.x.max(self.end.x);
                point.y == self.start.y && (x_min..=x_max).contains(&point.x)
            }
            Orientation::Vertical => {
                let y_min = self.start.y.min(self.end.y);
                let y_max = self.start.y.max(self.end.y);
                point.x == self.start.x && (y_min..=y_max).contains(&point.y)
            }
            Orientation::Degenerate => self.start == point,
        }
    }
}

impl Rectangle {
    fn inner(mut self) -> Self {
        let diff = self.to.icoord2d - self.from.icoord2d;

        let x_sgn = diff.x.signum();
        let y_sgn = diff.y.signum();

        self.from.icoord2d.x += x_sgn;
        self.from.icoord2d.y += y_sgn;
        self.to.icoord2d.x -= x_sgn;
        self.to.icoord2d.y -= y_sgn;

        self
    }

    fn area(&self) -> usize {
        let diff = self.from.icoord2d - self.to.icoord2d;
        let x_size = diff.x.abs() + 1;
        let y_size = diff.y.abs() + 1;

        (x_size * y_size) as usize
    }

    fn line_passes_through(&self, line: Line) -> bool {
        self.inner()
            .lines()
            .iter()
            .any(|inner_line| inner_line.intersects(&line))
    }

    fn extra_corners(&self) -> [Tile; 2] {
        [
            Tile {
                icoord2d: ICoord2D {
                    x: self.from.icoord2d.x,
                    y: self.to.icoord2d.y,
                },
            },
            Tile {
                icoord2d: ICoord2D {
                    x: self.to.icoord2d.x,
                    y: self.from.icoord2d.y,
                },
            },
        ]
    }

    fn lines(&self) -> [Line; 4] {
        let corners = [
            self.from,
            Tile {
                icoord2d: ICoord2D {
                    x: self.from.icoord2d.x,
                    y: self.to.icoord2d.y,
                },
            },
            self.to,
            Tile {
                icoord2d: ICoord2D {
                    x: self.to.icoord2d.x,
                    y: self.from.icoord2d.y,
                },
            },
        ];

        [
            Line {
                start: corners[0].icoord2d,
                end: corners[1].icoord2d,
            },
            Line {
                start: corners[1].icoord2d,
                end: corners[2].icoord2d,
            },
            Line {
                start: corners[2].icoord2d,
                end: corners[3].icoord2d,
            },
            Line {
                start: corners[3].icoord2d,
                end: corners[0].icoord2d,
            },
        ]
    }
}

#[derive(Debug, Clone)]
struct Polygon {
    points: Vec<Tile>,
}

impl FromStr for Polygon {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split('\n')
            .map(|e| e.parse::<Tile>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { points })
    }
}

impl Polygon {
    fn lines(&self) -> impl Iterator<Item = Line> {
        self.points
            .windows(2)
            .map(|p| Line {
                start: p[0].icoord2d,
                end: p[1].icoord2d,
            })
            .chain(std::iter::once(Line {
                start: self.points.last().unwrap().icoord2d,
                end: self.points.first().unwrap().icoord2d,
            }))
    }

    fn contains_tiles(&self, tiles: [Tile; 2]) -> bool {
        // must be huge enough to cross the entire polygon on the x-axis - to make sure we catch all intersections
        let huge_integer = 100000000;

        let mut on_border = [false; 2];
        let mut odd_crossings = [false; 2];

        let long_lines = tiles.map(|t| Line {
            start: t.icoord2d,
            end: t.icoord2d
                + ICoord2D {
                    x: huge_integer,
                    y: 0,
                },
        });

        for line in self.lines() {
            for (i, tile) in tiles.iter().enumerate() {
                if line.contains_point(tile.icoord2d) {
                    on_border[i] = true;

                    if on_border == [true; 2] {
                        return true;
                    }
                }

                if !line.same_orientation_or_either_is_degenerate(&long_lines[i])
                    && line.intersects(&long_lines[i])
                {
                    odd_crossings[i] = !odd_crossings[i];
                }
            }
        }

        (on_border[0] || odd_crossings[0]) && (on_border[1] || odd_crossings[1])
    }

    fn largest_rectangle_largest_rectangle_inside_polygon(&self) -> (usize, usize) {
        let mut largest_rectangle = None;
        let mut largest_rectangle_inside_polygon = None;

        let rectangles_descending = {
            let mut rectangles = self
                .points
                .iter()
                .enumerate()
                .flat_map(|(i, tile)| {
                    self.points.iter().skip(i + 1).map(|other_tile| Rectangle {
                        from: *tile,
                        to: *other_tile,
                    })
                })
                .collect::<Vec<_>>();

            rectangles.sort_unstable_by(|a, b| a.area().cmp(&b.area()).reverse());
            rectangles
        };

        for rectangle in rectangles_descending.into_iter() {
            if largest_rectangle.is_none() {
                largest_rectangle = Some(rectangle.area());
            }

            let extra_corners = rectangle.extra_corners();

            if !self.contains_tiles(extra_corners) {
                continue;
            }

            if self.lines().any(|line| rectangle.line_passes_through(line)) {
                continue;
            }

            largest_rectangle_inside_polygon =
                Some(largest_rectangle_inside_polygon.unwrap_or(rectangle.area()));

            break;
        }

        (
            largest_rectangle.unwrap(),
            largest_rectangle_inside_polygon.unwrap(),
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let polygon = std::fs::read_to_string("input")?
        .trim_end()
        .parse::<Polygon>()?;

    let (part1, part2) = polygon.largest_rectangle_largest_rectangle_inside_polygon();

    println!("{part1}");
    println!("{part2}");

    Ok(())
}
