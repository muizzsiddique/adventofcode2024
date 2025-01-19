use std::fs;

fn main() {
    let input = fs::read_to_string("aoc04.txt").unwrap();

    let size = input.lines().count();
    let mut xmas_count = 0;

    let mut prev = input.lines();
    let mut curr = prev.clone().skip(1);
    let mut next = curr.clone().skip(1);

    while let (Some(prev_line), Some(curr_line), Some(next_line)) = (prev.next(), curr.next(), next.next()) {
        // println!("  {prev_line:?}\n> {curr_line:?}\n  {next_line:?}\n");

        let mut tl = prev_line.chars();
        let mut tr = prev_line.chars().skip(2);
        let mut main = curr_line.chars().skip(1);
        let mut bl = next_line.chars();
        let mut br = next_line.chars().skip(2);

        while let (Some(tl_char), Some(tr_char), Some(main_char), Some(bl_char), Some(br_char)) = (tl.next(), tr.next(), main.next(), bl.next(), br.next()) {
            if main_char == 'A' && (tl_char != br_char) && (tr_char != bl_char) {
                // println!("{tl_char} {tr_char}\n {main_char} \n{bl_char} {br_char}");
                let surroundings: String = vec![tl_char, tr_char, bl_char, br_char].iter().collect();
                if surroundings.chars().filter(|&c| c == 'M').count() == 2 && surroundings.chars().filter(|&c| c == 'S').count() == 2 {
                     // println!("Forms \"MAS\" twice!!");
                     xmas_count += 1;
                }
            }
        }
    }

    println!("Output = {}", xmas_count);
}
