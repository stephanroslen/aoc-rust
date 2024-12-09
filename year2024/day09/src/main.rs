use std::cmp::Reverse;
use std::collections::BinaryHeap;
use util::error::Errors;

#[derive(Copy, Clone, Debug)]
struct Range {
    start: usize,
    length: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?
        .trim_end()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(Errors::UncategorizedError("Parse problem".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    {
        let mut data: Vec<Option<u32>> = Vec::new();

        {
            let mut is_file = true;
            let mut file_num = 0u32;

            for v in &input {
                let v = *v;
                if is_file {
                    assert!(v > 0);
                }
                for _ in 0..v {
                    data.push(if is_file { Some(file_num) } else { None });
                }
                is_file = !is_file;
                if is_file {
                    file_num += 1;
                }
            }
        }

        {
            let mut free_indices = BinaryHeap::new();

            for idx in 0..data.len() {
                let data_at_idx = data[idx];
                match data_at_idx {
                    None => {
                        free_indices.push(Reverse(idx));
                    }
                    Some(_) => (),
                }
            }
            for idx in (0..data.len()).rev() {
                let data_at_idx = data[idx];
                match data_at_idx {
                    None => (),
                    Some(_) => {
                        if let Some(Reverse(to_idx)) = free_indices.pop() {
                            if to_idx > idx {
                                break;
                            }
                            data[to_idx] = data_at_idx;
                            data[idx] = None;
                            free_indices.push(Reverse(idx));
                        }
                    }
                }
            }
        }

        let result1: usize = data
            .iter()
            .enumerate()
            .map(|(idx, val)| idx * val.unwrap_or_default() as usize)
            .sum();

        println!("{}", result1);
    }

    {
        let mut files: Vec<(u32, Range)> = Vec::new();
        let mut free_spaces: Vec<Range> = Vec::new();

        {
            let mut is_file = true;
            let mut file_num = 0u32;
            let mut idx = usize::default();

            for v in &input {
                let v = *v;
                let range = Range {
                    start: idx,
                    length: v as usize,
                };
                if is_file {
                    assert!(v > 0);
                    files.push((file_num, range));
                } else {
                    free_spaces.push(range);
                };

                idx += v as usize;
                is_file = !is_file;
                if is_file {
                    file_num += 1;
                }
            }
        }

        let mut space_idx = 0usize;
        let mut file_idx = 1usize;
        loop {
            let mut next_space = false;
            if let Some(space) = free_spaces.get(space_idx) {
                let space = space.to_owned();
                for file_back_idx in (0..files.len()).rev() {
                    if files[file_back_idx].1.start < space.start {
                        next_space = true;
                        break;
                    }
                    if files[file_back_idx].1.length <= space.length {
                        let file_data = files[file_back_idx];
                        files.insert(
                            file_idx,
                            (
                                file_data.0,
                                Range {
                                    start: space.start,
                                    length: file_data.1.length,
                                },
                            ),
                        );
                        file_idx += 1;
                        files.remove(file_back_idx + 1);
                        if space.length == file_data.1.length {
                            free_spaces.remove(space_idx);
                            break;
                        } else {
                            free_spaces[space_idx].length -= file_data.1.length;
                            free_spaces[space_idx].start += file_data.1.length;
                            break;
                        }
                    }
                }
            }
            if next_space {
                space_idx += 1;
            }
            if space_idx >= free_spaces.len() {
                break;
            }
        }

        let result2: usize = files
            .iter()
            .map(|(id, Range { start, length })| {
                (*start..(*start + *length)).sum::<usize>() * (*id as usize)
            })
            .sum();

        println!("{}", result2);
    }

    Ok(())
}
