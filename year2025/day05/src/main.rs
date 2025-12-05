use std::borrow::Borrow;
use std::ops::RangeInclusive;
use std::str::FromStr;
use util::error::Errors;

#[derive(Debug, Copy, Clone)]
struct IngredientRange {
    min: usize,
    max: usize,
}

impl FromStr for IngredientRange {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-').ok_or_else(|| Errors::ParseError)?;
        Ok(Self {
            min: min.parse()?,
            max: max.parse()?,
        })
    }
}

impl Into<RangeInclusive<usize>> for IngredientRange {
    fn into(self) -> RangeInclusive<usize> {
        self.min..=self.max
    }
}

impl IngredientRange {
    fn contains(self, ingredient: impl Borrow<usize>) -> bool {
        let range: RangeInclusive<_> = self.into();
        range.contains(ingredient.borrow())
    }

    fn len(&self) -> usize {
        self.max - self.min + 1
    }
}

#[derive(Debug, Clone)]
struct IngredientRanges {
    ranges: Vec<IngredientRange>,
}

impl IngredientRanges {
    fn contains(&self, ingredient: impl Borrow<usize> + Copy) -> bool {
        self.ranges.iter().any(|range| range.contains(ingredient))
    }

    fn merge_ranges(mut self) -> Self {
        let mut ranges = std::mem::take(&mut self.ranges);

        ranges.sort_unstable_by_key(|range| range.min);

        let mut new_ranges = Vec::new();
        let mut current_range: Option<IngredientRange> = None;
        for next_range in ranges {
            if let Some(current_range) = &mut current_range {
                if current_range.max >= next_range.min - 1 {
                    current_range.max = next_range.max.max(current_range.max);
                } else {
                    new_ranges.push(std::mem::replace(current_range, next_range));
                }
            } else {
                current_range = Some(next_range);
            }
        }

        if let Some(current_range) = current_range {
            new_ranges.push(current_range);
        }

        self.ranges = new_ranges;

        self
    }

    fn len(&self) -> usize {
        self.ranges.iter().map(|range| range.len()).sum()
    }
}

impl FromStr for IngredientRanges {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            ranges: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    id: usize,
}

impl FromStr for Ingredient {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { id: s.parse()? })
    }
}

impl Borrow<usize> for Ingredient {
    fn borrow(&self) -> &usize {
        &self.id
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?;
    let input_split = input.trim_end().split("\n\n").collect::<Vec<_>>();
    let ingredient_ranges = input_split[0].parse::<IngredientRanges>()?.merge_ranges();
    let ingredients = input_split[1]
        .lines()
        .map(|line| line.parse::<Ingredient>())
        .collect::<Result<Vec<_>, _>>()?;

    let result_part1 = {
        let mut result_part1 = 0_usize;

        for ingredient in ingredients {
            if ingredient_ranges.contains(ingredient) {
                result_part1 += 1;
            }
        }

        result_part1
    };

    let result_part2 = ingredient_ranges.len();

    println!("{result_part1}");
    println!("{result_part2}");

    Ok(())
}
