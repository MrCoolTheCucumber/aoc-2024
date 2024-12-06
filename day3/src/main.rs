use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").expect("unable to read input");

    let re = Regex::new(r"mul\(([\d]+),([\d]+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;

    for cap in re.captures_iter(&input) {
        match &cap[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                let a: i32 = cap[1].parse().unwrap();
                let b: i32 = cap[2].parse().unwrap();
                sum += a * b;
            }
            _ => {}
        }
    }

    println!("sum: {sum}");
}
