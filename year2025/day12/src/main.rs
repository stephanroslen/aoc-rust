use std::str::FromStr;
use util::error::Errors;
use util::grid2d::UCoord2D;

#[derive(Debug, Clone, Copy)]
struct FieldSetup {
    dim: UCoord2D,
    present_num: [usize; 6],
}

impl FromStr for FieldSetup {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split_ascii_whitespace().collect::<Vec<_>>();
        if chunks.len() != 7 {
            return Err(Errors::ParseError);
        }

        let dim_chunk = chunks.remove(0);
        if !(dim_chunk.ends_with(":")) {
            return Err(Errors::ParseError);
        }
        let dim_chunk = &dim_chunk[..dim_chunk.len() - 1];

        let dims = dim_chunk
            .split('x')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        if dims.len() != 2 {
            return Err(Errors::ParseError);
        }
        let dim = UCoord2D {
            x: dims[0],
            y: dims[1],
        };

        let mut present_num = [0; 6];

        for i in 0..6 {
            present_num[i] = chunks[i].parse::<usize>()?;
        }

        Ok(Self { dim, present_num })
    }
}

impl FieldSetup {
    fn slots(&self) -> usize {
        let reduced_dim = UCoord2D {
            x: self.dim.x / 3,
            y: self.dim.y / 3,
        };
        reduced_dim.x * reduced_dim.y
    }

    fn presents(&self) -> usize {
        self.present_num.iter().sum::<usize>()
    }

    fn fits(&self) -> bool {
        self.slots() >= self.presents()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let field_setups = std::fs::read_to_string("input")?
        .trim_end()
        .split("\n\n")
        .last()
        .ok_or(Errors::ParseError)?
        .split("\n")
        .map(|s| s.parse::<FieldSetup>())
        .collect::<Result<Vec<_>, _>>()?;

    let result = field_setups.iter().filter(|fs| fs.fits()).count();

    println!("{}", result);

    Ok(())
}
