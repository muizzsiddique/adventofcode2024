use std::fs;
use std::str::FromStr;
// use std::cmp::Ordering;

fn main() {
    let input = fs::read_to_string("aoc05.txt").unwrap();

    //  v mut
    let rules: Vec<_> = input.lines().take_while(|&l| l != "").map(|rule_str| {
        let (left, right) = rule_str.split_once('|').unwrap();
        let left = isize::from_str(left).unwrap();
        let right = isize::from_str(right).unwrap();

        (left, right)
    }).collect();

    let updates: Vec<_> = input.lines().skip_while(|&l| l.contains('|') || l == "").collect();

    let mut sum = 0;

    for update in updates.iter() {
        let pages: Vec<_> = update.split(',').map(|p| isize::from_str(p).unwrap()).collect();

        // Yet another wasted programming effort:
        // rules.sort_by(|r1, r2| {
        //     let (r1_left, r1_right) = r1;
        //     let (r2_left, r2_right) = r2;
        //
        //     let r1_left = pages.iter().position(|e| e == r1_left);
        //     let r2_left = pages.iter().position(|e| e == r2_left);
        //
        //     match r1_left.cmp(&r2_left) {
        //         Ordering::Equal => {
        //             r1_right.cmp(&r2_right)
        //         },
        //         cmp => cmp
        //     }
        // });

        let mut pages_sorted: Vec<isize> = vec![];

        for page in pages.iter() {
            // println!("{pages_sorted:?}");
            let index = rules
                .iter()
                .filter_map(|(left, right)| {

                    if left == page {
                        // println!("({left:?}, {right:?})");
                        pages_sorted.iter().position(|e| e==right)
                    } else {
                        None
                    }
                })
                .min();

            // println!("{page:?} => {index:?}");
            if let Some(index) = index {
                pages_sorted.insert(index, page.clone());
                continue;
            }
            pages_sorted.push(page.clone());
        }

        // println!("{pages_sorted:?}");

        if pages_sorted != pages {
            sum += pages_sorted.get(pages_sorted.len()/2).unwrap();
        }
    }

    println!("Sum = {}", sum);
}
