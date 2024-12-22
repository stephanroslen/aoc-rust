type UsedInteger = isize;

const MAX_ABS_PRICE_DIFF: usize = 9;
const MAX_PRICE_DIFF_VALUES: usize = MAX_ABS_PRICE_DIFF * 2 + 1;
const ARRAY_SIZE: usize =
    MAX_PRICE_DIFF_VALUES * MAX_PRICE_DIFF_VALUES * MAX_PRICE_DIFF_VALUES * MAX_PRICE_DIFF_VALUES;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
struct PriceChangeSequence {
    sequence: [UsedInteger; 4],
}

impl PriceChangeSequence {
    fn push_back(&mut self, price_change: UsedInteger) {
        self.sequence = [
            self.sequence[1],
            self.sequence[2],
            self.sequence[3],
            price_change,
        ];
    }

    fn to_idx(self) -> usize {
        let s = self
            .sequence
            .map(|v| (v + MAX_ABS_PRICE_DIFF as isize) as usize);
        s[0] + (s[1] + (s[2] + s[3] * MAX_PRICE_DIFF_VALUES) * MAX_PRICE_DIFF_VALUES)
            * MAX_PRICE_DIFF_VALUES
    }
}

fn new_secret(old_secret: UsedInteger) -> UsedInteger {
    const PRUNE: UsedInteger = 16777216;
    let mut result = old_secret;

    result = ((result * 64) ^ result) % PRUNE;
    result = ((result / 32) ^ result) % PRUNE;
    result = ((result * 2048) ^ result) % PRUNE;

    result
}

fn price(secret: UsedInteger) -> UsedInteger {
    secret % 10
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let nums = raw_input
        .trim_end()
        .split('\n')
        .map(|s| s.parse::<UsedInteger>())
        .collect::<Result<Vec<_>, _>>()?;

    let (result1, result2) = {
        let mut price_change_sequence_profit = [u16::default(); ARRAY_SIZE];
        let mut result1 = UsedInteger::default();

        const STEPS: usize = 2000;

        for num in nums {
            let mut secret = num;
            let mut old_price = price(secret);
            let mut price_change_sequence = PriceChangeSequence::default();
            let mut occurred_price_change_sequences = [false; ARRAY_SIZE];
            for i in 0..STEPS {
                secret = new_secret(secret);
                let price = price(secret);

                price_change_sequence.push_back(old_price - price);

                let idx = price_change_sequence.to_idx();

                if i >= 3 {
                    if !occurred_price_change_sequences[idx] {
                        price_change_sequence_profit[idx] += price as u16;

                        occurred_price_change_sequences[idx] = true;
                    }
                }

                old_price = price;
            }
            result1 += secret;
        }

        (
            result1,
            price_change_sequence_profit
                .iter()
                .max()
                .copied()
                .expect("Best value expected"),
        )
    };

    println!("{}", result1);
    println!("{}", result2);

    Ok(())
}
