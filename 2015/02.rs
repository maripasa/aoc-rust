use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (res1, res2) = solve(&input);
    println!("{:?} {:?}", res1, res2);
}

fn solve(input: &str) -> (i32, i32) {
    let res1 = input.lines().map(|x| wrapping_size(parse_input(x))).sum::<i32>();
    let res2 = input.lines().map(|x| ribbon_size(parse_input(x))).sum::<i32>();
    (res1, res2)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.trim().split('x').map(|x| x.trim().parse().expect("huh")).collect()
}

fn wrapping_size(bx: Vec<i32>) -> i32 {
    let l = bx[0];
    let w = bx[1];
    let h = bx[2];

    let sides = [l * w, w * h, h * l];
    sides.iter().map(|x| 2 * x).sum::<i32>() + sides.iter().min().unwrap()
}

fn ribbon_size(bx: Vec<i32>) -> i32 {
    let l = bx[0];
    let w = bx[1];
    let h = bx[2];

    let perimeters = [l + w, w + h, h + l];
    perimeters.iter().map(|x| 2*x).min().unwrap() + l * w * h
}
