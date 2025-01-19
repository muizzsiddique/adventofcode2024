use std::fs;
use std::str::FromStr;
use regex::Regex;

// RegEx 1: mul\(\d+,\d+\)
// RegEx 2: (?:mul\(\d+,\d+\)|do\(\)|don't\(\))

fn main() {
    let input = fs::read_to_string("aoc03.txt").unwrap();

    let operation = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let multiply = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result = operation
        .find_iter(&input)
        .map(|i| i.as_str())
        .fold((true, 0), |init, op| {
            // println!("{op:?}");
            match op {
                "do()" => (true, init.1),
                "don't()" => (false, init.1),
                mul => {
                    let mut sum = init.1;
                    if init.0 {
                        let (_, [n1, n2]) = multiply.captures(mul).unwrap().extract();
                        let n1 = i32::from_str(n1).unwrap();
                        let n2 = i32::from_str(n2).unwrap();
                        sum += n1 * n2;
                    }
                    (init.0, sum)
                },
            }
        });

    println!("Output = {:?}", result.1);
}
