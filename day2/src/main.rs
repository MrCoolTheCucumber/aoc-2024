use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_lines() -> Vec<String> {
    let file = File::open("../input.txt").expect("no input file found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("unable to read line"))
        .collect::<Vec<_>>()
}

fn main() {
    let (num_safe, num_safe_dampened) = get_lines()
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold((0, 0), |state @ (safe, safe_dampened), report| {
            if is_safe(&report) {
                return (safe + 1, safe_dampened + 1);
            }

            // since the number of levels in a report are so small, just brute force
            for i in 0..report.len() {
                let mut candiate = report.clone();
                candiate.remove(i);

                if is_safe(&candiate) {
                    return (safe, safe_dampened + 1);
                }
            }

            state
        });

    println!("num of safe levels: {num_safe}");
    println!("num safe dampened: {num_safe_dampened}");
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let mut increasing: Option<bool> = None;

    for window in report.windows(2) {
        let &[a, b] = window else {
            panic!("Unexpected window slice length")
        };

        match increasing {
            // tracking an increase, but b decreased
            Some(true) if a > b => return false,
            // tracking a decrease, but b increased
            Some(false) if a < b => return false,
            Some(_) => {}
            None => increasing = Some(a < b),
        };

        let abs_diff = a.abs_diff(b);
        if !(1..=3).contains(&abs_diff) {
            return false;
        }
    }

    true
}
