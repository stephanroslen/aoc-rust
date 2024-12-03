use regex::Regex;

enum Elem {
    Do,
    Dont,
    Mul(u32, u32),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?.trim_end().to_owned();

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)")?;

    let parsed = re
        .captures_iter(input.as_ref())
        .map(|c| -> Result<Elem, util::error::Errors> {
            match &c[0] {
                "do()" => Ok(Elem::Do),
                "don't()" => Ok(Elem::Dont),
                _ => Ok(Elem::Mul(c[1].parse()?, c[2].parse()?)),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let result1 = parsed
        .iter()
        .filter_map(|elem| match elem {
            Elem::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum::<u32>();

    println!("{}", result1);

    let result2 = {
        let mut active = true;
        parsed
            .iter()
            .filter_map(|elem| match elem {
                Elem::Mul(a, b) => {
                    if active {
                        Some(a * b)
                    } else {
                        None
                    }
                }
                Elem::Do => {
                    active = true;
                    None
                }
                Elem::Dont => {
                    active = false;
                    None
                }
            })
            .sum::<u32>()
    };

    println!("{}", result2);

    Ok(())
}
