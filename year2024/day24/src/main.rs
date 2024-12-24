use rustc_hash::FxHashMap;
use std::str::FromStr;
use util::error::Errors;

type Bit = bool;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Name {
    data: [u8; 3],
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&self.data).expect("Valid utf-8 expected")
    }
}

impl FromStr for Name {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 3 {
            return Err(Errors::ParseError);
        }
        Ok(Self {
            data: [bytes[0], bytes[1], bytes[2]],
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Op {
    Xor,
    Or,
    And,
}

#[derive(Copy, Clone, Debug)]
enum Part1Wire {
    Known(Bit),
    Evaluate(EvalWire),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct EvalWire {
    opers: [Name; 2],
    op: Op,
}

impl EvalWire {
    fn new(oper0: Name, op: Op, oper1: Name) -> Self {
        Self {
            opers: [oper0.min(oper1), oper0.max(oper1)],
            op,
        }
    }
}

fn eval_part1(data: &mut FxHashMap<Name, Part1Wire>, name: Name) -> Bit {
    match data.get(&name).copied().expect("Entry for name expected") {
        Part1Wire::Known(known) => known,
        Part1Wire::Evaluate(EvalWire {
            opers: [left, right],
            op,
        }) => {
            let left_val = eval_part1(data, left);
            let right_val = eval_part1(data, right);
            let val = match op {
                Op::Xor => left_val != right_val,
                Op::And => left_val && right_val,
                Op::Or => left_val || right_val,
            };

            *data.get_mut(&name).expect("Entry for name expected") = Part1Wire::Known(val);

            val
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let raw_lines = raw_input.trim_end().split('\n').collect::<Box<[_]>>();
    let raw_line_blocks = raw_lines.split(|s| s.is_empty()).collect::<Box<[_]>>();
    let raw_inputs = raw_line_blocks[0];
    let raw_evaluations = raw_line_blocks[1];

    let (wire_map, targets, gate_to_wire) = {
        let mut wire_map = FxHashMap::default();

        for inp in raw_inputs {
            let parts = inp.split(": ").collect::<Box<[_]>>();
            let name = parts[0].parse::<Name>()?;
            let val = parts[1].parse::<u8>()? == 1;

            wire_map.insert(name, Part1Wire::Known(val));
        }

        let mut targets = Vec::new();

        let mut gate_to_wire = FxHashMap::default();

        for eval in raw_evaluations {
            let parts = eval.split(' ').collect::<Box<[_]>>();
            let name = parts[4].parse::<Name>()?;
            let op = match parts[1] {
                "XOR" => Ok(Op::Xor),
                "AND" => Ok(Op::And),
                "OR" => Ok(Op::Or),
                _ => Err(Errors::ParseError),
            }?;

            let oper0 = parts[0].parse::<Name>()?;
            let oper1 = parts[2].parse::<Name>()?;

            let ew = EvalWire::new(oper0, op, oper1);

            wire_map.insert(name, Part1Wire::Evaluate(ew));

            gate_to_wire.insert(ew, name);

            if name
                .as_ref()
                .chars()
                .nth(0)
                .expect("First character expected")
                == 'z'
            {
                targets.push(name);
            }
        }

        targets.sort();

        (wire_map, targets, gate_to_wire)
    };

    let result1 = {
        let mut wire_map = wire_map;
        targets
            .iter()
            .rev()
            .map(|t| {
                let bit = eval_part1(&mut wire_map, *t);
                match bit {
                    false => 0usize,
                    true => 1usize,
                }
            })
            .fold(0, |acc, next| acc * 2 + next)
    };

    println!("{}", result1);

    let result2 = {
        let max_index =
            *&targets.last().expect("Last target expected").as_ref()[1..3].parse::<usize>()?;

        let mut swaps = Vec::with_capacity(8);

        let mut carry = *gate_to_wire
            .get(&EvalWire::new(
                "x00".parse::<Name>().expect("3 ascii char string expected"),
                Op::And,
                "y00".parse::<Name>().expect("3 ascii char string expected"),
            ))
            .expect("First carry expected");

        for i in 1..max_index {
            let x = format!("x{:02}", i)
                .parse::<Name>()
                .expect("3 ascii char string expected");
            let y = format!("y{:02}", i)
                .parse::<Name>()
                .expect("3 ascii char string expected");
            let z = format!("z{:02}", i)
                .parse::<Name>()
                .expect("3 ascii char string expected");

            let mut and0 = *gate_to_wire
                .get(&EvalWire::new(x, Op::And, y))
                .expect("And gate for inputs expected");

            let mut xor0 = *gate_to_wire
                .get(&EvalWire::new(x, Op::Xor, y))
                .expect("Xor gate for inputs expected");

            if !gate_to_wire.contains_key(&EvalWire::new(carry, Op::And, xor0)) {
                swaps.push(and0);
                swaps.push(xor0);
                std::mem::swap(&mut and0, &mut xor0);
            }

            let mut and1 = *gate_to_wire
                .get(&EvalWire::new(carry, Op::And, xor0))
                .expect("Inner and gate expected");

            let mut xor1 = *gate_to_wire
                .get(&EvalWire::new(carry, Op::Xor, xor0))
                .expect("Inner xor gate expected");

            if and0 == z {
                swaps.push(and0);
                swaps.push(xor1);
                std::mem::swap(&mut and0, &mut xor1);
            }

            if and1 == z {
                swaps.push(and1);
                swaps.push(xor1);
                std::mem::swap(&mut and1, &mut xor1);
            }

            let mut or0 = *gate_to_wire
                .get(&EvalWire::new(and0, Op::Or, and1))
                .expect("Or gate expected");

            if or0 == z {
                swaps.push(or0);
                swaps.push(xor1);
                std::mem::swap(&mut or0, &mut xor1);
            }

            carry = or0;
        }

        assert_eq!(
            carry,
            format!("z{:02}", max_index)
                .parse::<Name>()
                .expect("3 ascii char string expected")
        );

        swaps.sort();
        swaps
            .iter()
            .map(|s| s.as_ref())
            .collect::<Box<[_]>>()
            .join(",")
    };

    println!("{}", result2);

    Ok(())
}
