use util::error::Errors;

#[derive(Clone, Debug)]
struct Line {
    left: u64,
    right: Vec<u64>,
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Line {
    fn eval_part_1(&self) -> bool {
        let op_cnt = self.right.len() - 1;
        let max = 2u64.pow(op_cnt as u32);
        for i in 0..max {
            let mut ops = Vec::new();
            for io in 0..op_cnt {
                let op = match (i >> io) & 1 {
                    1 => Op::Mul,
                    _ => Op::Add,
                };
                ops.push(op);
            }

            let mut accum = self.right[0];
            for (op, num) in ops.iter().zip(self.right.iter().skip(1)) {
                accum = match op {
                    Op::Mul => accum * num,
                    _ => accum + num,
                }
            }

            if accum == self.left {
                return true;
            }
        }

        false
    }
}

impl Line {
    fn eval_part_2(&self) -> bool {
        let op_cnt = self.right.len() - 1;
        let max = 3u64.pow(op_cnt as u32);
        for i in 0..max {
            let mut itmp = i;
            let mut ops = Vec::new();
            for _ in 0..op_cnt {
                let op = match itmp % 3 {
                    2 => Op::Mul,
                    1 => Op::Add,
                    _ => Op::Concat,
                };
                ops.push(op);
                itmp /= 3;
            }

            let mut accum = self.right[0];
            for (op, num) in ops.iter().zip(self.right.iter().skip(1)) {
                let nv = match op {
                    Op::Mul => accum * num,
                    Op::Add => accum + num,
                    Op::Concat => accum * 10u64.pow(num.to_string().len() as u32) + num,
                };
                accum = nv;
            }

            if accum == self.left {
                return true;
            }
        }

        false
    }
}

impl std::str::FromStr for Line {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let left_right: Vec<_> = s.split(": ").collect();
        if left_right.len() != 2 {
            return Err(Errors::ParseError);
        }
        let vec_right = left_right[1]
            .split(" ")
            .map(|e| -> Result<u64, Errors> { Ok(e.parse::<u64>()?) })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            left: left_right[0].parse::<u64>()?,
            right: vec_right,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<Line>())
        .collect::<Result<Vec<Line>, Errors>>()?;

    println!(
        "{}",
        input
            .iter()
            .filter(|l| l.eval_part_1())
            .map(|l| l.left)
            .sum::<u64>()
    );

    println!(
        "{}",
        input
            .iter()
            .filter(|l| l.eval_part_2())
            .map(|l| l.left)
            .sum::<u64>()
    );
    Ok(())
}
