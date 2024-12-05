use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?.trim_end().to_string();
    let input_parts: Vec<_> = raw_input.split("\n\n").collect();
    println!("Hello, world!");

    let mut rules_a_before_b: HashMap<u32, HashSet<u32>> = HashMap::new();

    input_parts[0]
        .split('\n')
        .map(|line| {
            let s: Vec<_> = line.split('|').collect();
            let before: u32 = s.get(0).unwrap().parse().unwrap();
            let after: u32 = s.get(1).unwrap().parse().unwrap();
            (before, after)
        })
        .for_each(|(before, after)| {
            rules_a_before_b.entry(before).or_default().insert(after);
        });

    let lists: Vec<_> = input_parts[1]
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|p| p.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut result1 = 0u32;

    let mut for_part2 = Vec::new();

    for list in lists {
        let mut valid = true;
        for idx0 in 1..list.len() {
            let val0 = list.get(idx0).unwrap();
            if let Some(e) = rules_a_before_b.get(val0) {
                for idx1 in 0..idx0 {
                    let val1 = list.get(idx1).unwrap();
                    if e.contains(val1) {
                        valid = false;
                    }
                }
            }
        }
        if valid {
            result1 += list.get(list.len() / 2).unwrap();
        } else {
            for_part2.push(list);
        }
    }

    println!("{}", result1);

    let mut result2 = 0u32;

    for mut list in for_part2 {
        loop {
            let mut changed = false;
            'oloop: for idx0 in 1..list.len() {
                let val0 = list.get(idx0).unwrap().to_owned();
                if let Some(e) = rules_a_before_b.get(&val0) {
                    for idx1 in 0..idx0 {
                        let val1 = list.get(idx1).unwrap().to_owned();
                        if e.contains(&val1) {
                            let tmp = list[idx0];
                            list[idx0] = list[idx1];
                            list[idx1] = tmp;
                            changed = true;
                            break 'oloop;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        result2 += list.get(list.len() / 2).unwrap();
    }

    println!("{}", result2);

    Ok(())
}
