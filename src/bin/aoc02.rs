use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("aoc02.txt").unwrap();

    let reports: i32 = input.lines().map(|line| {
        let report: Vec<_> = line.split_whitespace().map(|s| i32::from_str(s).unwrap()).collect();
        // println!("{report:?}");

        if !is_safe(&report) {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                // println!("{new_report:?}");
                if is_safe(&new_report) {
                    return 1;
                }
            }
            return 0;
        }
        1
    }).sum();

    println!("Output = {reports}");
}


fn is_safe(report: &Vec<i32>) -> bool {
    if !(report.iter().is_sorted() || report.iter().rev().is_sorted()) {
        return false;
    }

    for (a,b) in report.iter().zip(report.iter().skip(1)) {
        // println!("{a} <-> {b} = {}", a-b);
        if !(1..=3).contains(&(a-b).abs()) {
            return false;
        }
    };

    true
}
