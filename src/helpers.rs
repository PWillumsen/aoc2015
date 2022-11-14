/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use aoc::helpers::example_fn;`.
 */

use anyhow::Result;
use std::str::FromStr;

pub fn parse_input<T>(input: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(input
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}
