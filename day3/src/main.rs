//! Adevnt of Code Day 3 Solution

use std::{error::Error, io::Read};

use regex::{Captures, Regex};

fn eval_mul_instr(instr: Captures) -> u32 {
    let one: u32 = instr
        .get(1)
        .and_then(|s| s.as_str().parse().ok())
        .expect("first operand");

    let two: u32 = instr
        .get(2)
        .and_then(|s| s.as_str().parse().ok())
        .expect("second operand");

    one * two
}

// Lexing logic for enabled substrings
enum Token {
    Disable(usize),
    Enable(usize),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        use Token::*;

        matches!(
            (self, other),
            (Enable(_), Enable(_)) | (Disable(_), Disable(_))
        )
    }
}

const ENABLE_TOKEN: &str = r"do\(\)";
const DISABLE_TOKEN: &str = r"don't\(\)";

fn enabled_strs(input: &str) -> Vec<&str> {
    let enable_re = Regex::new(ENABLE_TOKEN).unwrap();
    let disable_re = Regex::new(DISABLE_TOKEN).unwrap();

    let enable_tokens = enable_re.find_iter(input).map(|m| Token::Enable(m.end()));
    let disable_tokens = disable_re
        .find_iter(input)
        .map(|m| Token::Disable(m.start()));

    let mut tokens: Vec<_> = enable_tokens.chain(disable_tokens).collect();

    tokens.sort_by_key(|token| match token {
        Token::Disable(idx) | Token::Enable(idx) => *idx,
    });

    if let Some(Token::Enable(_)) = tokens.first() {
        tokens.remove(0);
    }

    // Relies on PartialEq implementation
    tokens.dedup();

    // The token vector is now of the form
    // [Disable(..), Enable(..), Disable(..), ...]
    let mut result = vec![];
    let mut enabled_idx = 0;

    for token in tokens.iter() {
        match token {
            Token::Disable(start) => result.push(&input[enabled_idx..*start]),
            Token::Enable(end) => enabled_idx = *end,
        }
    }

    if let Some(Token::Enable(_)) = tokens.last() {
        result.push(&input[enabled_idx..]);
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mul_instr = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf)?;

    let sum: u32 = mul_instr.captures_iter(&buf).map(eval_mul_instr).sum();

    let sum_enabled: u32 = enabled_strs(&buf)
        .iter()
        .flat_map(|s| mul_instr.captures_iter(s).map(eval_mul_instr))
        .sum();

    println!("Sum of all mul() instructions: {}", sum);
    println!("Sum of all enabled mul() instructions: {}", sum_enabled);

    Ok(())
}
