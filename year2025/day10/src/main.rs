use std::str::FromStr;
use util::error::Errors;
use z3::{
    Optimize,
    ast::{Bool, Int},
};

#[derive(Debug, Clone)]
struct Vals {
    vals: Vec<usize>,
}

impl Vals {
    fn len(&self) -> usize {
        self.vals.len()
    }
}

impl Vals {
    fn from_str_lights(s: &str) -> Result<Self, Errors> {
        let len = s.len();
        if &s[0..=0] != "[" {
            return Err(Errors::ParseError);
        };
        if &s[len - 1..=len - 1] != "]" {
            return Err(Errors::ParseError);
        };
        let s = &s[1..=len - 2];

        let vals = s
            .chars()
            .map(|c| match c {
                '.' => Ok(0),
                '#' => Ok(1),
                _ => Err(Errors::ParseError),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { vals })
    }

    fn from_str_button(s: &str, n: usize) -> Result<Self, Errors> {
        let len = s.len();
        if &s[0..=0] != "(" {
            return Err(Errors::ParseError);
        };
        if &s[len - 1..=len - 1] != ")" {
            return Err(Errors::ParseError);
        };
        let s = &s[1..=len - 2];

        let indices = s
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut vals = vec![0; n];

        indices.iter().for_each(|i| vals[*i] = 1);

        Ok(Self { vals })
    }

    fn from_str_joltages(s: &str) -> Result<Self, Errors> {
        let len = s.len();
        if &s[0..=0] != "{" {
            return Err(Errors::ParseError);
        };
        if &s[len - 1..=len - 1] != "}" {
            return Err(Errors::ParseError);
        };
        let s = &s[1..=len - 2];

        let vals = s
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { vals })
    }
}

#[derive(Debug, Clone)]
struct Machine {
    lights: Vals,
    buttons: Vec<Vals>,
    joltages: Vals,
}

fn combine_xor(slice: &[&Bool]) -> Bool {
    slice
        .iter()
        .fold(Bool::from_bool(false), |acc, x| acc.xor(*x))
}

fn bool_to_int(b: &Bool) -> Int {
    b.ite(&Int::from_u64(1), &Int::from_u64(0))
}

fn sum_bool(slice: &[&Bool]) -> Int {
    let ints: Vec<_> = slice.into_iter().map(|b| bool_to_int(b)).collect();
    Int::add(&ints)
}

trait FreshConst {
    fn fresh_const(s: &str) -> Self;
}

impl FreshConst for Int {
    fn fresh_const(s: &str) -> Self {
        Int::fresh_const(s)
    }
}

impl FreshConst for Bool {
    fn fresh_const(s: &str) -> Self {
        Bool::fresh_const(s)
    }
}

impl Machine {
    fn make_button_presses<T: FreshConst>(&self) -> Vec<T> {
        let mut button_presses = Vec::with_capacity(self.buttons.len());
        for i_button in 0..self.buttons.len() {
            button_presses.push(T::fresh_const(&std::format!("button{}", i_button)));
        }
        button_presses
    }

    fn collect_buttons<'lt_button_presses, T>(
        &self,
        button_presses: &'lt_button_presses [T],
        i_light: usize,
    ) -> Vec<&'lt_button_presses T> {
        let mut buttons = Vec::new();
        for i_button in 0..self.buttons.len() {
            if self.buttons[i_button].vals[i_light] == 1 {
                buttons.push(&button_presses[i_button]);
            }
        }
        buttons
    }

    fn part1(&self) -> Result<usize, Errors> {
        let optimize = Optimize::new();

        let button_presses: Vec<Bool> = self.make_button_presses();

        for i_light in 0..self.lights.len() {
            let target = self.lights.vals[i_light] == 1;

            let buttons = self.collect_buttons(&button_presses, i_light);

            let xored = combine_xor(&buttons);

            optimize.assert(&xored.eq(Bool::from_bool(target)));
        }

        let button_presses_refs: Vec<&Bool> = button_presses.iter().collect();
        let sum = sum_bool(&button_presses_refs);

        Self::minimize_and_extract(optimize, &sum)
    }

    fn part2(&self) -> Result<usize, Errors> {
        let optimize = Optimize::new();

        let button_presses: Vec<Int> = self.make_button_presses();

        for button_press in button_presses.iter() {
            optimize.assert(&button_press.ge(Int::from(0)));
        }

        for i_joltage in 0..self.joltages.len() {
            let target = self.joltages.vals[i_joltage];

            let buttons = self.collect_buttons(&button_presses, i_joltage);

            optimize.assert(&Int::add(&buttons).eq(Int::from(target as u64)));
        }

        let button_presses_refs: Vec<&Int> = button_presses.iter().collect();
        let sum = Int::add(&button_presses_refs);

        Self::minimize_and_extract(optimize, &sum)
    }

    fn minimize_and_extract(optimize: Optimize, sum: &Int) -> Result<usize, Errors> {
        optimize.minimize(sum);

        if !matches!(&optimize.check(&[]), z3::SatResult::Sat) {
            return Err(Errors::UncategorizedError("z3 error".into()));
        }

        let model = optimize
            .get_model()
            .ok_or(Errors::UncategorizedError("z3 error".into()))?;

        Ok(model
            .eval(sum, true)
            .and_then(|i| i.as_u64())
            .ok_or(Errors::UncategorizedError("z3 error".into()))? as usize)
    }
}

impl FromStr for Machine {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elem = s.split_whitespace().collect::<Vec<_>>();

        let len = elem.len();

        let lights = Vals::from_str_lights(elem[0])?;
        let num_lights = lights.len();

        let buttons = elem[1..=len - 2]
            .iter()
            .map(|s| Vals::from_str_button(s, num_lights))
            .collect::<Result<Vec<_>, _>>()?;

        let joltages = Vals::from_str_joltages(elem[len - 1])?;

        Ok(Self {
            lights,
            buttons,
            joltages,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let machines = std::fs::read_to_string("input")?
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<Machine>())
        .collect::<Result<Vec<_>, _>>()?;

    let part1 = machines
        .iter()
        .map(|machine| machine.part1())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<usize>();
    let part2 = machines
        .iter()
        .map(|machine| machine.part2())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<usize>();

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
