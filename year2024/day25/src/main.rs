fn transform_iter<'a>(it: impl Iterator<Item = &'a &'a str>) -> [u8; 5] {
    let mut result = [5u8; 5];

    for (num, &line) in it.skip(1).take(5).enumerate() {
        for (numc, c) in line.chars().enumerate() {
            if c == '.' {
                result[numc] = std::cmp::min(result[numc], num as u8);
            }
        }
    }

    result
}

fn transform_lock(lines: &[&str]) -> [u8; 5] {
    transform_iter(lines.iter())
}

fn transform_key(lines: &[&str]) -> [u8; 5] {
    transform_iter(lines.iter().rev())
}

fn matches(lock: [u8; 5], key: [u8; 5]) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let raw_lines = raw_input.trim_end().split('\n').collect::<Box<[_]>>();
    let raw_elements = raw_lines.split(|l| l.is_empty()).collect::<Box<[_]>>();

    let (raw_locks, raw_keys): (Vec<_>, Vec<_>) = raw_elements
        .iter()
        .copied()
        .partition(|e| e.first().unwrap().chars().all(|c| c == '#'));

    let locks = raw_locks
        .iter()
        .map(|l| transform_lock(l))
        .collect::<Box<[_]>>();
    let keys = raw_keys
        .iter()
        .map(|k| transform_key(k))
        .collect::<Box<[_]>>();

    let mut result1 = 0usize;

    for &l in &locks {
        for &k in &keys {
            if matches(l, k) {
                result1 += 1;
            }
        }
    }

    println!("{}", result1);

    Ok(())
}
