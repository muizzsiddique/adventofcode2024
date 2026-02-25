use std::fs;
use std::str::FromStr;

use itertools::Itertools;

enum LevelType {
    Increasing = 1,
    Decreasing = -1,
}

impl TryFrom<(&i32, &i32)> for LevelType {
    type Error = String;
    fn try_from(value: (&i32, &i32)) -> Result<Self, Self::Error> {
        if value.0 > value.1 {
            Ok(Self::Increasing)
        } else if value.0 < value.1 {
            Ok(Self::Decreasing)
        } else {
            Err("levels are unsafe".to_owned())
        }
    }
}

fn main() {
    let input = fs::read_to_string("aoc02-practice.txt").unwrap();

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    reports
        .iter()
        .filter(|report| {
            let mut report_iter = report.iter().tuple_windows();
            let mut report_type: LevelType = report_iter
                .take(1)
                .map(|levels: (&i32, &i32)| LevelType::try_from(levels).unwrap())
                .collect::<Vec<_>>()[0];

            report_iter
        })
        .count();
}
