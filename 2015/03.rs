use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (res1, res2) = solve(&input);
    println!("{:?} {:?}", res1, res2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited2: HashSet<(i32, i32)> = HashSet::new();

    let mut part1 = (0, 0);
    let mut robot = (0, 0);
    let mut santa = (0, 0);

    let mut diff = (0, 0);

    visited.insert((0, 0));
    visited2.insert((0, 0));

    for (idx, instruction) in input.chars().enumerate() {
        diff = match instruction {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _   => diff,
        };
        // 1
        part1 = (part1.0 + diff.0, part1.1 + diff.1);
        visited.insert(part1);

        // 2
        if idx % 2 == 0 {
            santa = (santa.0 + diff.0, santa.1 + diff.1);
            visited2.insert(santa);
            continue;
        }

        robot = (robot.0 + diff.0, robot.1 + diff.1);
        visited2.insert(robot);
    }

    (visited.len(), visited2.len())
}
