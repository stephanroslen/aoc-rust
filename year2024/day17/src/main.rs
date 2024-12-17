use std::fmt::{Display, Formatter};
use util::error::Errors;

type Value = u64;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<Value> for Instruction {
    fn from(value: Value) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction number"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ComboOperand {
    Literal(Value),
    Register(Value),
}

impl From<Value> for ComboOperand {
    fn from(value: Value) -> Self {
        match value {
            0 | 1 | 2 | 3 => ComboOperand::Literal(value),
            4 | 5 | 6 => ComboOperand::Register(value - 4),
            _ => panic!("Invalid operand number"),
        }
    }
}

impl Display for ComboOperand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComboOperand::Literal(0) => write!(f, "0"),
            ComboOperand::Literal(1) => write!(f, "1"),
            ComboOperand::Literal(2) => write!(f, "2"),
            ComboOperand::Literal(3) => write!(f, "3"),
            ComboOperand::Register(0) => write!(f, "A"),
            ComboOperand::Register(1) => write!(f, "B"),
            ComboOperand::Register(2) => write!(f, "C"),
            _ => write!(f, "Invalid combo operand"),
        }
    }
}

#[derive(Clone, Debug)]
struct Machine {
    registers: [Value; 3],
    program: Vec<Value>,
    instruction_ptr: usize,
}

impl Machine {
    fn halted(&self) -> bool {
        self.instruction_ptr >= self.program.len() - 1
    }

    fn calculate_combo_operand_value(&self, raw_operand_value: Value) -> Value {
        let combo_operand = ComboOperand::from(raw_operand_value);
        match combo_operand {
            ComboOperand::Literal(value) => value,
            ComboOperand::Register(idx) => self.registers[idx as usize],
        }
    }

    fn step(&mut self) -> Option<Value> {
        let mut output = Option::default();

        let instruction = Instruction::from(self.program[self.instruction_ptr]);
        let raw_operand_value = self.program[self.instruction_ptr + 1];

        let mut next_instruction_ptr = self.instruction_ptr + 2;

        match instruction {
            Instruction::Adv | Instruction::Bdv | Instruction::Cdv => {
                let target_register = match instruction {
                    Instruction::Adv => 0,
                    Instruction::Bdv => 1,
                    cdv => {
                        if cdv != Instruction::Cdv {
                            panic!("value must be cdv");
                        }
                        2
                    }
                };
                self.registers[target_register] = self.registers[0]
                    / (2 as Value)
                        .pow(self.calculate_combo_operand_value(raw_operand_value) as u32);
            }
            Instruction::Bxl => {
                self.registers[1] = self.registers[1] ^ raw_operand_value;
            }
            Instruction::Bst => {
                self.registers[1] = self.calculate_combo_operand_value(raw_operand_value) & 0x7;
            }
            Instruction::Jnz => {
                if self.registers[0] != 0 {
                    next_instruction_ptr = raw_operand_value as usize;
                }
            }
            Instruction::Bxc => {
                self.registers[1] = self.registers[1] ^ self.registers[2];
            }
            Instruction::Out => {
                output = Some(self.calculate_combo_operand_value(raw_operand_value) & 0x7);
            }
        }

        self.instruction_ptr = next_instruction_ptr;
        output
    }

    fn disassemble(&self) -> Vec<String> {
        let mut ip = 0usize;
        let mut result = Vec::new();
        loop {
            let instruction = Instruction::from(self.program[ip]);
            let raw_operand_value = self.program[ip + 1];

            result.push(format!(
                "{:2}: {}",
                ip,
                match instruction {
                    Instruction::Adv | Instruction::Bdv | Instruction::Cdv => {
                        let target_register = match instruction {
                            Instruction::Adv => 'A',
                            Instruction::Bdv => 'B',
                            cdv => {
                                if cdv != Instruction::Cdv {
                                    panic!("value must be cdv");
                                }
                                'C'
                            }
                        };

                        format!(
                            "{target_register} = A / (2 ** {})",
                            ComboOperand::from(raw_operand_value)
                        )
                    }
                    Instruction::Bxl => {
                        format!("B = B ^ {raw_operand_value}")
                    }
                    Instruction::Bst => {
                        format!("B = {} mod 8", ComboOperand::from(raw_operand_value))
                    }
                    Instruction::Jnz => {
                        format!("if A != 0 jmp {}", raw_operand_value)
                    }
                    Instruction::Bxc => {
                        "B = B ^ C".into()
                    }
                    Instruction::Out => {
                        format!("output {} mod 8", ComboOperand::from(raw_operand_value))
                    }
                },
            ));
            ip += 2;
            if ip >= self.program.len() - 1 {
                break;
            }
        }

        result
    }

    fn output(mut self) -> Vec<Value> {
        let mut output = Vec::new();
        while !self.halted() {
            if let Some(output_value) = self.step() {
                output.push(output_value);
            }
        }
        output
    }

    fn output_for_a(mut self, a: Value) -> Vec<Value> {
        self.registers[0] = a;
        self.output()
    }
}

fn extract_register(raw_line: &str) -> Result<Value, Errors> {
    Ok(raw_line
        .split(' ')
        .nth(2)
        .expect("Third substring expected")
        .parse::<Value>()?)
}

fn extract_program(raw_line: &str) -> Result<Vec<Value>, Errors> {
    Ok(raw_line
        .split(' ')
        .nth(1)
        .expect("Second substring expected")
        .split(',')
        .map(|s| s.parse::<Value>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn join_output(output: &[Value]) -> String {
    output
        .iter()
        .map(|v| v.to_string())
        .reduce(|a, v| a + "," + &v)
        .expect("Elements to reduce expected")
}

fn find_solution_2(machine: Machine, base: Value) -> Option<Value> {
    for offset in 0..8 {
        let candidate = base + offset;
        let output = machine.clone().output_for_a(candidate);
        let nth_from_back = output.len() - 1;
        if machine
            .program
            .iter()
            .rev()
            .nth(nth_from_back)
            .expect("Nth from back expected for program")
            == output
                .iter()
                .rev()
                .nth(nth_from_back)
                .expect("Nth from back expected for output")
        {
            if nth_from_back == machine.program.len() - 1 {
                return Some(candidate);
            }
            let new_base = candidate * 8;
            if let Some(result) = find_solution_2(machine.clone(), new_base) {
                return Some(result);
            }
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let raw_lines = raw_input.trim_end().split('\n').collect::<Vec<_>>();

    let machine = {
        let registers = [
            extract_register(raw_lines[0])?,
            extract_register(raw_lines[1])?,
            extract_register(raw_lines[2])?,
        ];
        let program = extract_program(raw_lines[4])?;

        Machine {
            registers,
            program,
            instruction_ptr: 0,
        }
    };

    println!("{}", join_output(&machine.clone().output()));

    println!("\n{}\n", machine.disassemble().join("\n"));

    let result2 = find_solution_2(machine.clone(), 0).expect("Solution expected for part 2");

    println!("{}", result2);

    Ok(())
}
