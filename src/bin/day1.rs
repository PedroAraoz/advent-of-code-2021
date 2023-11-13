use std::{fs, i32::MAX};

pub fn read_input(path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    contents
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    part1();
    part2();
}

pub fn part2() {
    let input: Vec<i32> = read_input("inputs/day1/real.txt");
    let sum: Vec<i32> = input.windows(3).map( |x| x.iter().sum()).collect();

    let mut count = 0;
    let mut last: i32 = MAX;
    for i in sum {
        if i > last {
            count += 1;
        }
        last = i;
    }
    println!("{count}");
}

pub fn part1() {
    let input: Vec<i32> = read_input("inputs/day1/real.txt");
    let mut count = 0;
    let mut last: i32 = MAX;
    for i in input {
        if i > last {
            count += 1;
        }
        last = i;
    }
    println!("{count}");
}
