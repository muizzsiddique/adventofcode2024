// INCOMPLETE

// TODO: Day 6 part ii
// - [x] Phase 1: mark all turn tiles
// - [ ] Phase 2: iterate a window of 3 consecutive paths, and then try to short-curcuit it with the fourth path
// - [ ] Phase 3: count each successful short-circuit as an obstruction 'O' that causes a loop!

use array2d::Array2D;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

// The 0 indexed element is the starting position

fn left_iter(map: &Array2D<char>, pos: Pos) -> impl Iterator<Item = &char> {
    map.row_iter(pos.y)
        .unwrap()
        .rev()
        .skip(map.row_len() - pos.x - 1)
}

fn right_iter(map: &Array2D<char>, pos: Pos) -> impl Iterator<Item = &char> {
    map.row_iter(pos.y).unwrap().skip(pos.x)
}

fn up_iter(map: &Array2D<char>, pos: Pos) -> impl Iterator<Item = &char> {
    map.column_iter(pos.x)
        .unwrap()
        .rev()
        .skip(map.column_len() - pos.y - 1)
}

fn down_iter(map: &Array2D<char>, pos: Pos) -> impl Iterator<Item = &char> {
    map.column_iter(pos.x).unwrap().skip(pos.y)
}

fn main() {
    let input = fs::read_to_string("aoc06-practice.txt").unwrap();

    let mut map = Array2D::from_rows(
        &input
            .lines()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let mut guard = map
        .rows_iter()
        .enumerate()
        .find_map(|(y, r)| r.into_iter().position(|&c| c == '^').map(|x| Pos { x, y }))
        .unwrap();

    map.set(guard.y, guard.x, '.').unwrap();

    'a: loop {
        for i in 0..4 {
            match i {
                0 => {
                    let Some(obstruction) = up_iter(&map, guard).position(|&c| c == '#') else {
                        break 'a;
                    };
                    let y = guard.y - (obstruction - 1);
                    guard.y = y;
                    map.set(guard.y, guard.x, '+').unwrap();
                }
                1 => {
                    let Some(obstruction) = right_iter(&map, guard).position(|&c| c == '#') else {
                        break 'a;
                    };
                    let x = guard.x + (obstruction - 1);
                    guard.x = x;
                    map.set(guard.y, guard.x, '+').unwrap();
                }
                2 => {
                    let Some(obstruction) = down_iter(&map, guard).position(|&c| c == '#') else {
                        break 'a;
                    };
                    let y = guard.y + (obstruction - 1);
                    guard.y = y;
                    map.set(guard.y, guard.x, '+').unwrap();
                }
                3 => {
                    let Some(obstruction) = left_iter(&map, guard).position(|&c| c == '#') else {
                        break 'a;
                    };
                    let x = guard.x - (obstruction - 1);
                    guard.x = x;
                    map.set(guard.y, guard.x, '+').unwrap();
                }
                _ => panic!(),
            }
        }
    }

    map.rows_iter()
        .for_each(|r| println!("{:?}", r.collect::<Vec<_>>()));
}
