use std::fs;
// use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("aoc06-practice.txt").unwrap();

    let mut lines: Vec<_> = input.lines().map(|s| String::from(s)).collect();

    'main: loop {
        // println!();
        for y in 0..lines.len() {
            let line = lines.get(y).unwrap();
            // println!("{line}");

            if let Some(x) = line.find(['^', '>', 'v', '<']) {
                let guard = line.chars().nth(x).unwrap();

                match guard {
                    '^' => {
                        // if-let chains with &&s will be here in Rust 2024e, and let-else chains with && :shrugs: ...
                        let Some(new_y) = y.checked_sub(1) else {
                            break 'main;
                        };
                        let Some(line) = lines.get(new_y) else {
                            // println!("DEBUG: Approaching out of bounds.");

                            break 'main;
                        };

                        let mut line: Vec<char> = line.chars().collect();
                        if line[x] == '#' {
                            // println!("DEBUG: Roadblock! Turn right.");

                            let mut line: Vec<char> = lines[y].chars().collect();
                            line[x] = '>';
                            lines[y] = line.iter().collect();
                        } else {
                            // println!("DEBUG: Can move forward.");

                            line[x] = '^';
                            lines[y - 1] = line.iter().collect();

                            let mut line: Vec<char> = lines[y].chars().collect();
                            line[x] = 'X';
                            lines[y] = line.iter().collect();
                        }
                        continue 'main;
                    }
                    '>' => {
                        let mut line: Vec<char> = lines[y].chars().collect();

                        let Some(_) = line.get(x + 1) else {
                            // println!("DEBUG: Approaching out of bounds.");

                            break 'main;
                        };

                        if line[x + 1] == '#' {
                            // println!("DEBUG: Roadblock! Turn right.");

                            line[x] = 'v';
                            lines[y] = line.iter().collect();
                        } else {
                            // println!("DEBUG: Can move forward.");

                            line[x + 1] = '>';
                            line[x] = 'X';
                            lines[y] = line.iter().collect();
                        }
                        continue 'main;
                    }
                    'v' => {
                        let Some(line) = lines.get(y + 1) else {
                            // println!("DEBUG: Approaching out of bounds.");

                            break 'main;
                        };

                        let mut line: Vec<char> = line.chars().collect();
                        if line[x] == '#' {
                            // println!("DEBUG: Roadblock! Turn right.");

                            let mut line: Vec<char> = lines[y].chars().collect();
                            line[x] = '<';
                            lines[y] = line.iter().collect();
                        } else {
                            // println!("DEBUG: Can move forward.");

                            line[x] = 'v';
                            lines[y + 1] = line.iter().collect();

                            let mut line: Vec<char> = lines[y].chars().collect();
                            line[x] = 'X';
                            lines[y] = line.iter().collect();
                        }
                        continue 'main;
                    }
                    '<' => {
                        let mut line: Vec<char> = lines[y].chars().collect();

                        let Some(_) = line.get(x - 1) else {
                            // println!("DEBUG: Approaching out of bounds.");

                            break 'main;
                        };

                        if line[x - 1] == '#' {
                            // println!("DEBUG: Roadblock! Turn right.");

                            line[x] = '^';
                            lines[y] = line.iter().collect();
                        } else {
                            // println!("DEBUG: Can move forward.");

                            line[x - 1] = '<';
                            line[x] = 'X';
                            lines[y] = line.iter().collect();
                        }
                        continue 'main;
                    }
                    _ => (),
                }
            }
        }
    }

    // for line in lines.iter() {
    //     println!("{line}");
    // }

    println!(
        "Output = {:?}",
        lines
            .iter()
            .flat_map(|l| l.chars())
            .filter(|&c| c == 'X')
            .count()
            + 1
    );
}
