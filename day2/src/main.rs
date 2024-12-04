//! Advent of Code Day 2 Solution

use std::error::Error;

const MIN_DIFF: u32 = 1;
const MAX_DIFF: u32 = 3;

fn safe(report: &[i32]) -> bool {
    let incr = report[0] < report[1];

    for (i, num) in report.iter().enumerate().skip(1) {
        let last = report[i - 1];

        if (last < *num) != incr {
            return false;
        }

        if !(MIN_DIFF..=MAX_DIFF).contains(&num.abs_diff(last)) {
            return false;
        }
    }

    true
}

fn safe_dampened(report: &[i32]) -> bool {
    let incr = report[0] < report[1];

    for (i, num) in report.iter().enumerate().skip(1) {
        let last = report[i - 1];

        if (last < *num) != incr {
            // The monotonic invariant can be broken on element i=2 BECAUSE
            // element i=0 is wrong
            let skip_zero = &report[1..];
            let skip_this = [&report[..i], &report[(i + 1)..]].concat();
            let skip_last = [&report[..(i - 1)], &report[i..]].concat();

            return safe(skip_zero) || safe(&skip_this) || safe(&skip_last);
        }

        if !(MIN_DIFF..=MAX_DIFF).contains(&num.abs_diff(last)) {
            let skip_this = [&report[..i], &report[(i + 1)..]].concat();
            let skip_last = [&report[..(i - 1)], &report[i..]].concat();

            return safe(&skip_this) || safe(&skip_last);
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut num_safe = 0;
    let mut num_safe_dampened = 0;

    for line in std::io::stdin().lines() {
        let report: Vec<i32> = line?
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;

        num_safe += safe(&report) as i32;
        num_safe_dampened += safe_dampened(&report) as i32;
    }

    println!("Number of Safe Reports: {}", num_safe);
    println!("Number of Safe Reports (Dampened): {}", num_safe_dampened);

    Ok(())
}
