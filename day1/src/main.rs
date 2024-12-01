use std::{
    collections::HashMap,
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
    let lines = get_lines();

    let (mut list_a, mut list_b, map_b) = lines.into_iter().fold(
        (Vec::new(), Vec::new(), HashMap::<i32, i32>::new()),
        |(mut a, mut b, mut map_b), line| {
            let nums = line
                .split("   ")
                .map(|s| s.parse::<i32>().expect("unable to parse num"))
                .collect::<Vec<_>>();

            a.push(nums[0]);
            b.push(nums[1]);
            map_b
                .entry(nums[1])
                .and_modify(|count| *count += 1)
                .or_insert(1);

            (a, b, map_b)
        },
    );

    list_a.sort();
    list_b.sort();

    let total_distance = list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("total distance: {total_distance}");

    let similarity_score = list_a
        .iter()
        .map(|id| *map_b.get(id).unwrap_or(&0) * *id)
        .sum::<i32>();

    println!("similarity score: {similarity_score}");
}
