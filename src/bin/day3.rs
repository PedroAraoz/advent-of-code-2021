use std::fs;

fn read_input(path: &str) -> Vec<String> {
    let contents: Vec<String> = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    contents.iter().map(|x| x.to_string()).collect()
}

fn main() {
    part1();
}

pub fn part1() {
    let input: Vec<String> = read_input("inputs/day3/real.txt");
    let len = input[0].len();
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for i in 0..len {
        let q: Vec<String> = input
            .iter()
            .map(|x| x.chars().nth(i).unwrap().to_string())
            .collect();
        if q.iter().filter(|&x| *x == "1").count() > input.len() / 2 {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }
    let gamma_value = isize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_value = isize::from_str_radix(epsilon.as_str(), 2).unwrap();
    println!("power consumption: {}", gamma_value * epsilon_value);
}
