use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead as _, BufReader},
};

fn get_lines() -> impl Iterator<Item = String> {
    let file = File::open("../input.txt").expect("no input file found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("unable to read line"))
}

#[derive(Clone, Copy)]
#[allow(unused)]
enum Part {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn new_i32(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn in_bounds(&self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

fn calculate_antinode_points(antennas: &[Point], part: Part) -> HashSet<Point> {
    // Iterate through all possible pairs of points,
    // calculate the antinode location

    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let antenna_i = &antennas[i];
            let antenna_j = &antennas[j];

            let dx = antenna_i.x - antenna_j.x;
            let dy = antenna_i.y - antenna_j.y;

            match part {
                Part::One => {
                    let antinode_1 = Point::new_i32(antenna_i.x + dx, antenna_i.y + dy);
                    let antinode_2 =
                        Point::new_i32(antenna_i.x + (dx * -2), antenna_i.y + (dy * -2));

                    antinodes.insert(antinode_1);
                    antinodes.insert(antinode_2);
                }
                Part::Two => {
                    for i in 0..=55 {
                        let antinode =
                            Point::new_i32(antenna_i.x + (dx * i), antenna_i.y + (dy * i));
                        antinodes.insert(antinode);

                        let antinode =
                            Point::new_i32(antenna_i.x + (dx * -i), antenna_i.y + (dy * -i));
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    antinodes
}

fn main() {
    let lines: Vec<String> = get_lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let part_2 = Part::Two;

    let antenna_map: HashMap<char, Vec<Point>> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x, y), c))
        })
        .filter(|(_, c)| *c != '.')
        .fold(HashMap::new(), |mut map, (point, c)| {
            map.entry(c)
                .and_modify(|points| points.push(point))
                .or_insert(vec![point]);
            map
        });

    let antinode_points = antenna_map
        .values()
        .flat_map(|points| calculate_antinode_points(points, part_2))
        .filter(|point| point.in_bounds(width as i32, height as i32))
        .collect::<HashSet<Point>>();

    println!("unique antinode positions: {}", antinode_points.len());
}
