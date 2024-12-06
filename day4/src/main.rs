use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn get_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("no input file found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("unable to read line"))
        .collect::<Vec<_>>()
}

fn get_diagonals(lines: &[String]) -> Vec<String> {
    let width = lines[0].len();
    let height = lines.len();

    let mut diagonals: Vec<String> = Vec::new();

    // Diagonals from the left/top, going downward/right
    for x in 0..height {
        let mut x = x;
        let mut y = 0;

        let mut diagonal = String::new();

        while x < height && y < width {
            diagonal += lines[x].chars().collect::<Vec<_>>()[y].to_string().as_str();
            x += 1;
            y += 1;
        }

        diagonals.push(diagonal);
    }

    for y in 1..width {
        let mut x = 0;
        let mut y = y;

        let mut diagonal = String::new();

        while x < height && y < width {
            diagonal += lines[x].chars().collect::<Vec<_>>()[y].to_string().as_str();
            x += 1;
            y += 1;
        }

        diagonals.push(diagonal);
    }

    // Diagonals from bottom/left, going right/up

    for x in 1..height {
        let mut x = x;
        let mut y = 0;

        let mut diagonal = String::new();

        loop {
            diagonal += lines[x].chars().collect::<Vec<_>>()[y].to_string().as_str();

            if x == 0 || y == width - 1 {
                break;
            }

            x -= 1;
            y += 1;
        }

        diagonals.push(diagonal);
    }

    for y in 1..(width - 1) {
        let mut x = height - 1;
        let mut y = y;

        let mut diagonal = String::new();

        loop {
            diagonal += lines[x].chars().collect::<Vec<_>>()[y].to_string().as_str();

            if x == 0 || y == width - 1 {
                break;
            }

            x -= 1;
            y += 1;
        }

        diagonals.push(diagonal);
    }

    diagonals
}

fn get_verticals(lines: &[String]) -> Vec<String> {
    let width = lines[0].len();
    let height = lines.len();

    let mut verticals: Vec<String> = Vec::new();

    for y in 0..width {
        let mut line = Vec::new();

        #[allow(clippy::needless_range_loop)]
        for x in 0..height {
            line.push(lines[x].chars().collect::<Vec<_>>()[y]);
        }

        verticals.push(line.iter().collect());
    }

    verticals
}

fn main() {
    let lines = get_lines();
    let diagonals = get_diagonals(&lines);
    let verticals = get_verticals(&lines);

    let xmas_re = Regex::new("XMAS").unwrap();
    let samx_re = Regex::new("SAMX").unwrap();

    let count: usize = lines
        .iter()
        .chain(diagonals.iter())
        .chain(verticals.iter())
        .map(|l| xmas_re.find_iter(l).count() + samx_re.find_iter(l).count())
        .sum();

    println!("count: {count}");

    // Part 2
    let char_arr = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mas_re = Regex::new("MAS").unwrap();
    let sam_re = Regex::new("SAM").unwrap();

    let mut mas_count = 0;

    let width = char_arr[0].len();
    let height = char_arr.len();

    for x in 1..height - 1 {
        for y in 1..width - 1 {
            let str1: String = [
                char_arr[x - 1][y - 1],
                char_arr[x][y],
                char_arr[x + 1][y + 1],
            ]
            .into_iter()
            .collect();

            let str2: String = [
                char_arr[x + 1][y - 1],
                char_arr[x][y],
                char_arr[x - 1][y + 1],
            ]
            .into_iter()
            .collect();

            let match1 = mas_re.find(&str1).is_some() || sam_re.find(&str1).is_some();
            let match2 = mas_re.find(&str2).is_some() || sam_re.find(&str2).is_some();

            if match1 && match2 {
                mas_count += 1;
            }
        }
    }

    println!("x-mas count: {mas_count}");
}
