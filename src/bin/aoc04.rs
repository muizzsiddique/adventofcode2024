use std::fs;
use std::str::FromStr;
use regex::Regex;
use std::iter;

fn main() {
    let input = fs::read_to_string("aoc04.txt").unwrap();

    let size: usize = input.lines().count();
    let word = Regex::new("XMAS").unwrap();

    let ltr_count = input.lines().fold(0, |sum, line| {
        sum + word.find_iter(line).count()
    });

    let rtl_count = input.lines().fold(0, |sum, line| {
        let rev: String = line.chars().rev().collect();
        let matches: Vec<_> = word.find_iter(&rev).map(|w| w.as_str()).collect();
        sum + matches.iter().count()
    });

    let mut input_transposed: Vec<String> = vec![];
    {
        let mut lines_iter: Vec<_> = input.lines().map(|l| l.chars()).collect();

        'transpose:
        for i in 0..size {
            let mut line = String::new();

            for iter in lines_iter.iter_mut() {
                let Some(next) = iter.next() else { break 'transpose };
                line.push(next);
            }

            input_transposed.push(line);
        }
    }

    let ttb_count = input_transposed.iter().fold(0, |sum, line| {
        sum + word.find_iter(line).count()
    });

    let btt_count = input_transposed.iter().fold(0, |sum, line| {
        let rev: String = line.chars().rev().collect();
        let matches: Vec<_> = word.find_iter(&rev).map(|w| w.as_str()).collect();
        sum + matches.iter().count()
    });

    let mut input_rotated_northeast: Vec<String> = vec![];
    {
        let mut lines_iter: Vec<_> = input.lines().map(|l| l.chars()).collect();
        // println!("Lines: \n{lines_iter:?}");

        let range1 = 1..=size;
        let range2 = iter::repeat_n(size, size-1);
        for max in range1.chain(range2) {
            let mut line = String::new();
            // println!("Line: {max}");

            for line_iter in lines_iter.iter_mut().rev().skip(size-max) {
                // println!("Iteration: {line_iter:?}");
                let Some(next) = line_iter.next() else {
                    break;
                };
                line.push(next);
            }

            input_rotated_northeast.push(line);
        }
    }

    let blttr_count = input_rotated_northeast.iter().fold(0, |sum, line| {
        sum + word.find_iter(line).count()
    });

    let trtbl_count = input_rotated_northeast.iter().fold(0, |sum, line| {
        let rev: String = line.chars().rev().collect();
        let matches: Vec<_> = word.find_iter(&rev).map(|w| w.as_str()).collect();
        sum + matches.iter().count()
    });

    let mut input_rotated_southeast: Vec<String> = vec![];
    {
        let mut lines_iter: Vec<_> = input.lines().map(|l| l.chars().rev()).collect();
        // println!("Lines: \n{lines_iter:?}");

        let range1 = 1..=size;
        let range2 = iter::repeat_n(size, size-1);
        for max in range1.chain(range2) {
            let mut line = String::new();
            // println!("Line: {max}");

            for line_iter in lines_iter.iter_mut().rev().skip(size-max) {
                // println!("Iteration: {line_iter:?}");
                let Some(next) = line_iter.next() else {
                    break;
                };
                line.push(next);
            }

            input_rotated_southeast.push(line);
        }
    }

    let brttl_count = input_rotated_southeast.iter().fold(0, |sum, line| {
        sum + word.find_iter(line).count()
    });

    let tltbr_count = input_rotated_southeast.iter().fold(0, |sum, line| {
        let rev: String = line.chars().rev().collect();
        let matches: Vec<_> = word.find_iter(&rev).map(|w| w.as_str()).collect();
        sum + matches.iter().count()
    });

    let count = ltr_count + rtl_count + ttb_count + btt_count + blttr_count + trtbl_count + brttl_count + tltbr_count;
    println!("Final Output = {}", count);
}
