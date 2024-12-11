use std::collections::HashMap;

fn split_even_num(num: u64) -> Option<(u64, u64)> {
    let digits = num.ilog10() + 1;
    if digits % 2 == 1 {
        return None;
    }

    let digits_half = digits / 2;
    let divider = 10u64.pow(digits_half);

    Some((num / divider, num % divider))
}

fn stone_expands_to(
    remaining_iterations: u8,
    num: u64,
    cache: &mut HashMap<(u8, u64), usize>,
) -> usize {
    if remaining_iterations == 0 {
        return 1;
    }
    let cache_key = (remaining_iterations, num);
    if let Some(result) = cache.get(&cache_key).copied() {
        return result;
    }
    let result = match num {
        0 => stone_expands_to(remaining_iterations - 1, 1u64, cache),
        i => match split_even_num(i) {
            None => stone_expands_to(remaining_iterations - 1, i * 2024, cache),
            Some((s0, s1)) => {
                stone_expands_to(remaining_iterations - 1, s0, cache)
                    + stone_expands_to(remaining_iterations - 1, s1, cache)
            }
        },
    };
    cache.insert(cache_key, result);
    result
}

fn stones_expand_to(
    remaining_iterations: u8,
    data: &Vec<u64>,
    cache: &mut HashMap<(u8, u64), usize>,
) -> usize {
    data.iter()
        .map(|num| stone_expands_to(remaining_iterations, *num, cache))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?.trim_end().to_owned();

    let data = input
        .split(' ')
        .map(|v| v.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut cache = HashMap::new();

    println!("{}", stones_expand_to(25, &data, &mut cache));
    println!("{}", stones_expand_to(75, &data, &mut cache));

    Ok(())
}
