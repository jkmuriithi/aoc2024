use std::{collections::HashMap, env, error::Error, fs};

fn distance(left: &mut [u32], right: &mut [u32]) -> u32 {
    assert_eq!(left.len(), right.len(), "lists must be the same length");

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum()
}

fn similarity(left: &[u32], right: &[u32]) -> u32 {
    let mut freq: HashMap<u32, u32> = HashMap::new();

    right
        .iter()
        .for_each(|&num| *freq.entry(num).or_default() += 1);

    left.iter()
        .map(|num| num * freq.get(num).copied().unwrap_or_default())
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_filename = env::args().nth(1).expect("filename given");
    let input =
        fs::read_to_string(input_filename.as_str()).expect("valid file");

    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in input.lines() {
        let mut nums = line.split_whitespace();

        let left = nums.next().unwrap().parse()?;
        let right = nums.next().unwrap().parse()?;

        left_list.push(left);
        right_list.push(right);
    }

    println!("Distance: {}", distance(&mut left_list, &mut right_list));
    println!("Similarity: {}", similarity(&left_list, &right_list));

    Ok(())
}
