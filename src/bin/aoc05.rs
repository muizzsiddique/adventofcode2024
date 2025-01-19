use std::fs;
use std::str::FromStr;

fn main() {
    let mut input = fs::read_to_string("aoc05.txt").unwrap();

    let rules: Vec<_> = input.lines().take_while(|&l| l != "").collect();
    let updates: Vec<_> = input.lines().skip_while(|&l| l.contains('|') || l == "").collect();

    let mut sum = 0;

    'up:
    for update in updates.iter() {
        let pages: Vec<_> = update.split(',').map(|p| isize::from_str(p).unwrap()).collect();

        for rule in rules.iter() {
            let (left, right) = rule.split_once('|').unwrap();
            let left = isize::from_str(left).unwrap();
            let right = isize::from_str(right).unwrap();

            let (Some(left_i), Some(right_i)) = (pages.iter().position(|v| v==&left), pages.iter().position(|v| v==&right)) else {
                continue;
            };

            if left_i > right_i {
                continue 'up;
            }
        }

        sum += pages.get(pages.len()/2).unwrap();
    }

    println!("Sum = {}", sum);
}
