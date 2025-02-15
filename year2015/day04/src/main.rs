fn check(base: &str, num: u32, count: usize) -> bool {
    let mut tmp = base.to_owned();
    tmp += &num.to_string();

    let digest = chksum_md5::chksum(tmp);
    let hex = digest.unwrap().to_hex_lowercase();

    let first_digits = &hex[0..count];

    first_digits.chars().all(|c| c == '0')
}

fn find_solution(base: &str, count: usize) -> u32 {
    let mut x = 0;
    loop {
        if check(base, x, count) {
            return x;
        }
        x += 1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?.trim_end().to_string();

    let part1 = find_solution(&input, 5);
    println!("{}", part1);

    let part2 = find_solution(&input, 6);
    println!("{}", part2);

    Ok(())
}
