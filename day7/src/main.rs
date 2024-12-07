use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead as _, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add = 0,
    Mult = 1,
    Elephant = 2,
}

const OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mult, Operator::Elephant];

#[derive(Debug)]
struct CalibrationEquation {
    target: u64,
    elements: Vec<u64>,
}

impl CalibrationEquation {
    pub fn has_valid_equation(&self) -> bool {
        if self.elements.len() == 1 {
            return self.elements[0] == self.target;
        }

        let mut queue = VecDeque::new();
        let mut el_itr = self.elements.iter().copied();

        let sum = el_itr.next().unwrap();
        OPERATORS
            .iter()
            .for_each(|op| queue.push_front((sum, el_itr.clone(), *op)));

        while let Some((mut sum, mut el_itr, op)) = queue.pop_back() {
            match el_itr.next() {
                Some(rhs) => {
                    match op {
                        Operator::Add => sum += rhs,
                        Operator::Mult => sum *= rhs,
                        Operator::Elephant => sum = format!("{sum}{rhs}").parse::<u64>().unwrap(),
                    }

                    // prune
                    if sum > self.target {
                        continue;
                    }

                    OPERATORS
                        .iter()
                        .for_each(|op| queue.push_front((sum, el_itr.clone(), *op)));
                }
                None => {
                    if sum == self.target {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn get_lines() -> impl Iterator<Item = String> {
    let file = File::open("../input.txt").expect("no input file found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("unable to read line"))
}

fn parse_input() -> Vec<CalibrationEquation> {
    let lines = get_lines();

    lines
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            let target = split
                .next()
                .map(|s| s[0..s.len() - 1].parse::<u64>().unwrap())
                .unwrap();

            CalibrationEquation {
                target,
                elements: split.map(|s| s.parse().unwrap()).collect::<Vec<_>>(),
            }
        })
        .collect()
}

fn main() {
    let input = parse_input();

    let total_calibration_result = input
        .iter()
        .filter(|cal| cal.has_valid_equation())
        .map(|cal| cal.target)
        .sum::<u64>();

    println!("total_calibration_result: {total_calibration_result}");
}
