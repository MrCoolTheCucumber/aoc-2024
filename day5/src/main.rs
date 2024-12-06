use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead as _, BufReader},
};

fn get_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("no input file found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("unable to read line"))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Rule {
    before: i32,
    after: i32,
}

impl Rule {
    pub fn rule_is_valid_in_update(&self, update: &Update) -> bool {
        match (
            update.idx_map.get(&self.before),
            update.idx_map.get(&self.after),
        ) {
            (Some(before_idx), Some(after_idx)) => before_idx < after_idx,
            _ => true,
        }
    }
}

#[derive(Clone)]
struct Update {
    seq: Vec<i32>,
    idx_map: HashMap<i32, usize>,
}

impl Update {
    pub fn get_middle_page_number(&self) -> i32 {
        let middle_idx = (self.seq.len() / 2) + 1;
        self.seq[middle_idx - 1]
    }

    pub fn fix_update(&self, rules: &[Rule]) -> Update {
        let mut fixed_update = self.clone();

        while !is_update_correctly_ordered(rules, &fixed_update) {
            for rule in rules {
                if let (Some(before_idx), Some(after_idx)) = (
                    fixed_update.idx_map.get(&rule.before).cloned(),
                    fixed_update.idx_map.get(&rule.after).cloned(),
                ) {
                    if before_idx < after_idx {
                        continue;
                    }

                    fixed_update.idx_map.remove(&rule.before);
                    fixed_update.idx_map.remove(&rule.after);

                    fixed_update.idx_map.insert(rule.before, after_idx);
                    fixed_update.idx_map.insert(rule.after, before_idx);

                    fixed_update.seq.swap(before_idx, after_idx);

                    break;
                }
            }
        }

        fixed_update
    }
}

fn main() {
    let lines = get_lines();

    let (rules, updates) = parse_lines(&lines);

    let middle_page_sum = updates
        .iter()
        .filter(|update| is_update_correctly_ordered(&rules, update))
        .map(|update| update.get_middle_page_number())
        .sum::<i32>();

    println!("Middle page num sum: {middle_page_sum}");

    let middle_page_bad_update_sum = updates
        .iter()
        .filter(|update| !is_update_correctly_ordered(&rules, update))
        .map(|update| update.fix_update(&rules).get_middle_page_number())
        .sum::<i32>();

    println!("Middle page num sum of bad updates fixed: {middle_page_bad_update_sum}");
}

fn parse_lines(lines: &[String]) -> (Vec<Rule>, Vec<Update>) {
    let mut rules = Vec::new();
    let mut seqs = Vec::new();
    let mut parse_seqs = false;

    for line in lines {
        match line.as_str() {
            "" => parse_seqs = true,

            _ if !parse_seqs => {
                let raw_rule = line
                    .split("|")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                rules.push(Rule {
                    before: raw_rule[0],
                    after: raw_rule[1],
                });
            }

            _ if parse_seqs => {
                let seq = line
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                let idx_map = seq
                    .iter()
                    .enumerate()
                    .fold(HashMap::new(), |mut map, (idx, n)| {
                        map.insert(*n, idx);
                        map
                    });

                seqs.push(Update { seq, idx_map });
            }

            _ => unreachable!(),
        }
    }

    (rules, seqs)
}

fn is_update_correctly_ordered(rules: &[Rule], update: &Update) -> bool {
    rules
        .iter()
        .all(|rule| rule.rule_is_valid_in_update(update))
}
