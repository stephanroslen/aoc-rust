use ahash::{HashMap, HashMapExt};
use std::{fmt::Display, str::FromStr};
use util::error::Errors;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    name: [u8; 3],
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#""{}""#,
            str::from_utf8(&self.name).map_err(|_| std::fmt::Error)?
        )
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(&self.name).map_err(|_| std::fmt::Error)?
        )
    }
}

impl FromStr for Node {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.as_bytes();
        if name.len() != 3 {
            return Err(Errors::ParseError);
        }

        let name = [name[0], name[1], name[2]];

        Ok(Self { name })
    }
}

#[derive(Debug, Clone)]
struct Graph {
    edges: HashMap<Node, Vec<Node>>,
}

impl FromStr for Graph {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges = HashMap::new();

        for line in s.split('\n') {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            if parts.len() < 2 {
                return Err(Errors::ParseError);
            }
            if parts[0].len() != 4 {
                return Err(Errors::ParseError);
            }
            if &parts[0][3..=3] != ":" {
                return Err(Errors::ParseError);
            }

            let from = parts[0][0..3].parse::<Node>()?;
            let to = parts
                .iter()
                .skip(1)
                .map(|node| node.parse::<Node>())
                .collect::<Result<Vec<_>, _>>()?;

            edges.insert(from, to);
        }

        Ok(Self { edges })
    }
}

type Memo = HashMap<Node, usize>;

impl Graph {
    fn connections_from_to(&self, from: &Node, to: &Node, memo: &mut Memo) -> usize {
        if from == to {
            return 1;
        }

        if let Some(&result) = memo.get(&from) {
            return result;
        }

        let mut result = 0;

        for node in self.edges.get(from).map(|v| v.iter()).into_iter().flatten() {
            result += self.connections_from_to(node, to, memo);
        }

        memo.insert(*from, result);

        result
    }

    fn part1(&self) -> usize {
        self.connections_from_to(
            &"you".parse().unwrap(),
            &"out".parse().unwrap(),
            &mut HashMap::new(),
        )
    }

    fn part2(&self) -> usize {
        // acyclic graph - it must be either direction
        let dac_fft = self.connections_from_to(
            &"dac".parse().unwrap(),
            &"fft".parse().unwrap(),
            &mut HashMap::new(),
        );

        let fft_dac = self.connections_from_to(
            &"fft".parse().unwrap(),
            &"dac".parse().unwrap(),
            &mut HashMap::new(),
        );

        match (dac_fft, fft_dac) {
            (0, fft_dac) => {
                self.connections_from_to(
                    &"svr".parse().unwrap(),
                    &"fft".parse().unwrap(),
                    &mut HashMap::new(),
                ) * fft_dac
                    * self.connections_from_to(
                        &"dac".parse().unwrap(),
                        &"out".parse().unwrap(),
                        &mut HashMap::new(),
                    )
            }
            (dac_fft, 0) => {
                self.connections_from_to(
                    &"svr".parse().unwrap(),
                    &"dac".parse().unwrap(),
                    &mut HashMap::new(),
                ) * dac_fft
                    * self.connections_from_to(
                        &"fft".parse().unwrap(),
                        &"out".parse().unwrap(),
                        &mut HashMap::new(),
                    )
            }
            _ => panic!("Invalid graph"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = std::fs::read_to_string("input")?
        .trim_end()
        .parse::<Graph>()?;

    let part1 = graph.part1();
    let part2 = graph.part2();

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
