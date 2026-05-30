use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (res1, res2) = solve(&input);
    println!("{:?} {:?}", res1, res2);
}

fn solve(input: &str) -> (i32, i32) {
    let mut floor = 0;
    let mut basement = 0;

    for (idx, instruction) in input.chars().enumerate() {
        match instruction {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => (),
        }
        if floor == -1 && basement == 0 {
            basement = (idx + 1) as i32;
        }
    }

    (floor, basement)
}
