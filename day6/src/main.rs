use std::{
    collections::HashSet,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn next_pos(&self, cur_x: i32, cur_y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (cur_x - 1, cur_y),
            Direction::Down => (cur_x + 1, cur_y),
            Direction::Left => (cur_x, cur_y - 1),
            Direction::Right => (cur_x, cur_y + 1),
        }
    }
}

fn main() {
    let lines = get_lines()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let (mut cur_x, mut cur_y) = find_current_pos(width, height, &lines);
    let mut current_direction = Direction::Up;
    let mut unique_positions = HashSet::new();

    while cur_x >= 0 && cur_x < height && cur_y >= 0 && cur_y < width {
        unique_positions.insert((cur_x, cur_y));

        let (next_x, next_y) = current_direction.next_pos(cur_x, cur_y);

        if in_bounds(next_x, next_y, width, height)
            && lines[next_x as usize][next_y as usize] == '#'
        {
            current_direction = current_direction.turn_right();

            continue;
        }

        cur_x = next_x;
        cur_y = next_y;
    }

    println!("num distinct pos: {}", unique_positions.len());

    // brute force loops
    let mut count = 0;

    for x in 0..height {
        for y in 0..width {
            if lines[x as usize][y as usize] == '^' {
                continue;
            }

            let mut lines = lines.clone();
            lines[x as usize][y as usize] = '#';

            if causes_loop(&lines, width, height) {
                count += 1;
            }
        }
    }

    println!("count: {count}");
}

fn find_current_pos(width: i32, height: i32, lines: &[Vec<char>]) -> (i32, i32) {
    for x in 0..height {
        for y in 0..width {
            if lines[x as usize][y as usize] == '^' {
                return (x, y);
            }
        }
    }

    unreachable!()
}

fn in_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}

fn causes_loop(lines: &[Vec<char>], width: i32, height: i32) -> bool {
    let (mut cur_x, mut cur_y) = find_current_pos(width, height, lines);
    let mut current_direction = Direction::Up;
    let mut unique_positions = HashSet::new();

    while cur_x >= 0 && cur_x < height && cur_y >= 0 && cur_y < width {
        unique_positions.insert((cur_x, cur_y, current_direction));

        let (next_x, next_y) = current_direction.next_pos(cur_x, cur_y);

        if in_bounds(next_x, next_y, width, height)
            && lines[next_x as usize][next_y as usize] == '#'
        {
            current_direction = current_direction.turn_right();
            continue;
        }

        if unique_positions.contains(&(next_x, next_y, current_direction)) {
            return true;
        }

        cur_x = next_x;
        cur_y = next_y;
    }

    false
}
