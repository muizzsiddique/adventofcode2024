use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("aoc01.txt").unwrap();

    let mut list_left = Vec::<i32>::new();
    let mut list_right = Vec::<i32>::new();

    for line in input.lines() {
        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| i32::from_str(s).unwrap())
            .collect();

        list_left.push(numbers[0]);
        list_right.push(numbers[1]);
    }

    list_left.sort();
    list_right.sort();

    // let mut sum: i32 = 0;
    // for i in 0..list_left.len() {
    //     sum += (list_left[i] - list_right[i]).abs();
    // }

    let mut sum = 0;
    for i in 0..list_left.len() {
        sum += list_left[i] * list_right.iter().filter(|&&n| n == list_left[i]).count() as i32;
    }

    // println!("Left: {list_left:?}\nRight: {list_right:?}");
    println!("Output = {sum}")
}
