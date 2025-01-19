// INCOMPLETE

use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("aoc07-practice.txt").unwrap();

    input.lines().for_each(|l| {
        if let Some((sum, nums)) = l.split_once(':') {
            let sum = usize::from_str(sum);
            let nums: Vec<_> = nums
                .split_whitespace()
                .map(|n| usize::from_str(n))
                .collect();
        }
    });

    // println!("{input:?}")

    // let lines: Vec<_> = input.lines().map(|s| String::from(s)).collect();

    // lines.iter().for_each(|l| println!("{l}"));
    //
    for line in input.lines() {
        println!("{line}");
    }
}
