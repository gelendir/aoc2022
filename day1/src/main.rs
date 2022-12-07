use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn parse(path: String) -> Vec<Vec<u32>> {
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);

    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for line in buffer.lines() {
        let l = line.expect("cannot read line");
        match l.as_str() {
            "" => {
                elves.push(elf);
                elf = Vec::new();
            },
            number => {
                let weight: u32 = number.parse().expect("cannot parse number");
                elf.push(weight);
            }
        };
    }
    elves.push(elf);
    return elves;
}

fn main() {
    let path = env::args().nth(1).expect("missing file path");
    let elves = parse(path);

    let mut weights: Vec<u32> = elves.iter().map(|group| {
        group.iter().sum()
    })
    .collect();
    weights.sort();

    let max = weights.last().expect("cannot find max");

    let last_3 = weights.len() - 3;
    let top_3: u32 = weights[last_3..].iter().sum();

    println!("heaviest elf: {}", max);
    println!("top 3 heaviest elves: {}", top_3);
}
