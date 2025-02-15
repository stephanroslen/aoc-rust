use std::str::FromStr;
use util::error::Errors;

#[derive(Debug, Copy, Clone)]
struct Present {
    l: i32,
    w: i32,
    h: i32,
}

impl FromStr for Present {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split('x').collect::<Box<[&str]>>();
        if v.len() != 3 {
            return Err(Self::Err::ParseError);
        }

        let l = v[0].parse::<i32>()?;
        let w = v[1].parse::<i32>()?;
        let h = v[2].parse::<i32>()?;

        Ok(Present { l, w, h })
    }
}

impl Present {
    fn wrapping_paper_needed(&self) -> i32 {
        let areas = [self.l * self.w, self.l * self.h, self.w * self.h];
        let slack = areas.iter().min().expect("min value expected");
        areas.iter().sum::<i32>() * 2 + slack
    }

    fn ribbon_needed(&self) -> i32 {
        let volume = self.l * self.w * self.h;
        let perimeters = [
            (self.l + self.w) * 2,
            (self.l + self.h) * 2,
            (self.w + self.h) * 2,
        ];
        perimeters.iter().min().expect("min value expected") + volume
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .to_string()
        .into_boxed_str();

    let lines = input.as_ref().split('\n').collect::<Box<[&str]>>();
    let presents = lines
        .iter()
        .map(|l| l.parse::<Present>())
        .collect::<Result<Box<[_]>, _>>()?;

    let part1: i32 = presents.iter().map(|p| p.wrapping_paper_needed()).sum();
    println!("{}", part1);

    let part1: i32 = presents.iter().map(|p| p.ribbon_needed()).sum();
    println!("{}", part1);

    Ok(())
}
