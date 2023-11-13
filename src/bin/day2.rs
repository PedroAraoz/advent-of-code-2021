use std::fs;

fn read_input(path: &str) -> Vec<Command> {
    let contents: Vec<String> = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    contents
        .iter()
        .map(|x| {
            let split: Vec<&str> = x.split(" ").collect();
            let s = split[0];
            let n: i32 = split[1].parse().unwrap();
            match (s, n) {
                ("forward", x) => Command::Forward(x),
                ("down", x) => Command::Down(x),
                ("up", x) => Command::Up(x),
                _ => Command::Nothing,
            }
        })
        .collect()
}
#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    Nothing,
}

#[derive(Debug)]
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn main() {
    part1();
    part2();
}

pub fn part1() {
    let input: Vec<Command> = read_input("inputs/day2/real.txt");
    let mut sub = Submarine::new();
    for i in input {
        match i {
            Command::Forward(x) => sub.horizontal += x,
            Command::Down(x) => sub.depth += x,
            Command::Up(x) => sub.depth -= x,
            Command::Nothing => {}
        }
    }
    println!("{:?}", sub.depth * sub.horizontal);
}

pub fn part2() {
    let input: Vec<Command> = read_input("inputs/day2/real.txt");
    let mut sub = Submarine::new();
    for i in input {
        match i {
            Command::Forward(x) => {
                sub.horizontal += x;
                sub.depth += sub.aim * x;
            },
            Command::Down(x) => sub.aim += x,
            Command::Up(x) => sub.aim -= x,
            Command::Nothing => {}
        }
    }
    println!("{:?}", sub.depth * sub.horizontal);
}
