//! Advent of Code Day 5 Solution

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

type Num = u32;

/// Record of page ordering rules
#[derive(Debug, Clone, Default)]
struct Rules {
    relation: HashMap<Num, HashSet<Num>>,
}

impl Rules {
    fn new() -> Self {
        Self::default()
    }

    /// Add a page ordering rule.
    fn add(&mut self, comes_before: Num, comes_after: Num) {
        self.relation.entry(comes_before).or_default().insert(comes_after);
    }

    /// Returns true iff a "comes before" relation `~<` exists such that
    /// `a ~< b`.
    fn comes_before(&self, a: Num, b: Num) -> bool {
        self.relation.get(&a).map(|set| set.contains(&b)).unwrap_or(false)
    }

    // Returns true iff a violation of the ordering rules exists in `arr`
    fn ordered(&self, arr: &[Num]) -> bool {
        for i in 0..(arr.len() - 1) {
            for j in (i + 1)..arr.len() {
                if self.comes_before(arr[j], arr[i]) {
                    return false;
                }
            }
        }

        true
    }

    /// Reorders `arr` to conform to the ordering rules
    fn reorder(&self, arr: &mut [Num]) {
        let mut changed = true;

        while changed {
            changed = false;

            for i in 0..(arr.len() - 1) {
                for j in (i + 1)..arr.len() {
                    if self.comes_before(arr[j], arr[i]) {
                        arr.swap(i, j);
                        changed = true;
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rules = Rules::new();

    for line in std::io::stdin().lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let rule: Vec<_> =
            line.split('|').map(str::parse::<Num>).collect::<Result<_, _>>()?;

        rules.add(rule[0], rule[1])
    }

    let mut middle_sum_ordered = 0;
    let mut middle_sum_reordered = 0;

    for line in std::io::stdin().lines() {
        let line = line?;

        let mut nums: Vec<_> =
            line.split(',').map(str::parse::<Num>).collect::<Result<_, _>>()?;

        if rules.ordered(&nums) {
            middle_sum_ordered += nums[nums.len() / 2];
        } else {
            rules.reorder(&mut nums);
            middle_sum_reordered += nums[nums.len() / 2];
        }
    }

    println!("Sum of ordered middle numbers: {}", middle_sum_ordered);
    println!("Sum of reordered middle numbers: {}", middle_sum_reordered);

    Ok(())
}
