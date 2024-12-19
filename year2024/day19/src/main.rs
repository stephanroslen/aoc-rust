use std::collections::HashMap;

fn num_assemblies<'a>(
    target: &'a str,
    splits: &Vec<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    let mut result = 0usize;
    if target.is_empty() {
        return 1;
    }

    if let Some(&cache_entry) = cache.get(target) {
        return cache_entry;
    }

    for s in splits {
        if target.starts_with(s) {
            let new_target = &target[s.len()..target.len()];
            result += num_assemblies(new_target, splits, cache);
        }
    }

    cache.insert(target, result);

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let raw_blocks = raw_input.trim_end().split("\n\n").collect::<Vec<_>>();

    let towels = raw_blocks[0].split(", ").collect::<Vec<_>>();
    let targets = raw_blocks[1].split('\n').collect::<Vec<_>>();

    let mut result1 = 0usize;
    let mut result2 = 0usize;

    let mut cache = HashMap::new();

    for target in targets {
        let tmp = num_assemblies(target, &towels, &mut cache);
        if tmp != 0 {
            result1 += 1;
        }
        result2 += tmp;
    }

    println!("{}", result1);
    println!("{}", result2);

    Ok(())
}
