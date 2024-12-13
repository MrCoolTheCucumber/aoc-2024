use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

use scanf::sscanf;

#[derive(Debug, Clone, Copy, Default)]
struct Machine {
    a: Point,
    b: Point,
    goal: Point,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn get_lines() -> impl Iterator<Item = String> {
    let file = File::open("../input.txt").expect("no input file found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("unable to read line"))
}

fn parse_input() -> Vec<Machine> {
    let lines = get_lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    lines
        .chunks_exact(3)
        .map(|window| {
            let (mut ax, mut ay, mut bx, mut by, mut gx, mut gy) = (0, 0, 0, 0, 0, 0);

            sscanf!(&window[0], "Button A: X+{}, Y+{}", ax, ay).unwrap();
            sscanf!(&window[1], "Button B: X+{}, Y+{}", bx, by).unwrap();
            sscanf!(&window[2], "Prize: X={}, Y={}", gx, gy).unwrap();

            Machine {
                a: Point { x: ax, y: ay },
                b: Point { x: bx, y: by },
                goal: Point { x: gx, y: gy },
            }
        })
        .collect::<Vec<_>>()
}

fn solve(machine: &Machine) -> i64 {
    fn determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
        a * d - b * c
    }

    let d = determinant(machine.a.x, machine.b.x, machine.a.y, machine.b.y);
    let d1 = determinant(machine.goal.x, machine.goal.y, machine.b.x, machine.b.y);
    let d2 = determinant(machine.a.x, machine.a.y, machine.goal.x, machine.goal.y);

    // if div by 0 or non-integer solution
    if d == 0 || d1 % d != 0 || d2 % d != 0 {
        return 0;
    }

    let a = d1 / d;
    let b = d2 / d;
    (a * 3) + b
}

fn main() {
    let mut input = parse_input();

    let min_tokens_pt1 = input.iter().map(solve).sum::<i64>();
    println!("min tokens: {min_tokens_pt1}");

    let min_tokens_pt2 = input
        .iter_mut()
        .map(|machine| {
            machine.goal.x += 10000000000000;
            machine.goal.y += 10000000000000;
            solve(machine)
        })
        .sum::<i64>();
    println!("min tokens pt2: {min_tokens_pt2}");
}
