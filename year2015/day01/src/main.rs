use util::error::Errors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seq = std::fs::read_to_string("input")?
        .trim_end()
        .chars()
        .map(|c| match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            _ => Err(Errors::ParseError),
        })
        .collect::<Result<Box<_>, util::error::Errors>>()?;

    let part1: i32 = seq.iter().sum();
    println!("{}", part1);

    let part2 = seq
        .iter()
        .scan(0, |state, &v| {
            *state += v;
            Some(*state)
        })
        .enumerate()
        .find(|(_, val)| *val == -1)
        .ok_or(Errors::UncategorizedError("Level -1 not reached".into()))?
        .0
        + 1;
    println!("{}", part2);

    Ok(())
}
