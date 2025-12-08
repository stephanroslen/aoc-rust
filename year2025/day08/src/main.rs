use ahash::AHashSet as HashSet;
use std::str::FromStr;
use util::coord3d::ICoord3D;
use util::error::Errors;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    pos: ICoord3D,
}

impl FromStr for Coord {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems = s
            .split(',')
            .map(|e| e.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

        if elems.len() != 3 {
            return Err(Errors::ParseError);
        }

        Ok(Self {
            pos: ICoord3D {
                x: elems[0],
                y: elems[1],
                z: elems[2],
            },
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct CoordPair {
    from: Coord,
    to: Coord,
}

impl CoordPair {
    fn dist(&self) -> f64 {
        (self.from.pos - self.to.pos).magnitude()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct CoordPairDist {
    dist: f64,
    pair: CoordPair,
}

#[derive(Debug, Clone, Default)]
struct CoordCluster {
    members: HashSet<Coord>,
}

impl CoordCluster {
    fn new_from_coord(coord: Coord) -> Self {
        Self {
            members: HashSet::from([coord]),
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.members.contains(coord)
    }

    fn merge(&mut self, other: CoordCluster) {
        self.members.extend(other.members);
    }

    fn len(&self) -> usize {
        self.members.len()
    }
}

fn solutions(coords: &[Coord], coord_pair_dists: &[CoordPairDist]) -> (usize, isize) {
    let mut unconnected = coords.iter().cloned().collect::<HashSet<_>>();

    let mut clusters = Vec::new();

    let mut solution_part1 = None;

    for (
        idx,
        CoordPairDist {
            dist: _,
            pair: CoordPair { from, to },
        },
    ) in coord_pair_dists.iter().enumerate()
    {
        let cluster_idx_from = clusters
            .iter()
            .position(|c: &CoordCluster| c.contains(from));
        let mut cluster_from = cluster_idx_from
            .map(|i| clusters.remove(i))
            .unwrap_or(CoordCluster::new_from_coord(*from));

        let cluster_idx_to = clusters.iter().position(|c: &CoordCluster| c.contains(to));
        let cluster_to = cluster_idx_to
            .map(|i| clusters.remove(i))
            .unwrap_or(CoordCluster::new_from_coord(*to));

        cluster_from.merge(cluster_to);
        clusters.push(cluster_from);

        unconnected.remove(from);
        unconnected.remove(to);

        if idx == 1000 {
            clusters.sort_by(|a, b| b.len().cmp(&a.len()));
            solution_part1 = Some(clusters.iter().take(3).map(|c| c.len()).product());
        }

        if unconnected.is_empty() && clusters.len() == 1 {
            return (solution_part1.unwrap(), from.pos.x * to.pos.x);
        }
    }

    panic!("no coord should remain unconnected");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coords = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<Coord>())
        .collect::<Result<Vec<_>, _>>()?;

    let coord_pair_dists = {
        let coord_pairs = {
            let mut coord_pairs = Vec::new();

            for (i0, coord0) in coords.iter().enumerate() {
                for (i1, coord1) in coords.iter().enumerate() {
                    if i0 < i1 {
                        coord_pairs.push(CoordPair {
                            from: *coord0,
                            to: *coord1,
                        });
                    }
                }
            }

            coord_pairs
        };

        let mut coord_pair_dists = coord_pairs
            .into_iter()
            .map(|p| CoordPairDist {
                dist: p.dist(),
                pair: p,
            })
            .collect::<Vec<_>>();

        coord_pair_dists.sort_unstable_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        coord_pair_dists
    };

    let (part1, part2) = solutions(&coords, &coord_pair_dists);

    println!("{part1}");
    println!("{part2}");

    Ok(())
}
