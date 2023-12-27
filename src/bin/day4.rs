use std::{fs, string};

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
    let input: Vec<String> = read_input("inputs/day4/example.txt");
    let (numbers_drawn_string, string_boards) = input.split_at(1);
    let numbers_drawn: Vec<i32> = numbers_drawn_string[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let q: Vec<_> = string_boards.chunks(6).map(|x| x.split_at(1).1).collect();
    let p: Vec<_> = q
        .iter()
        .map(|&l| {
            l.iter()
                .map(|s| s.split(" ").map(|x| x.to_string()).collect())
                .collect::<Vec<Vec<String>>>()
        })
        .collect();
    let boards: Vec<Vec<Vec<Cell>>> = p
        .iter()
        .map(|a| {
            a.iter()
                .map(|b| {
                    b.iter()
                        .filter(|&e| *e != "".to_string())
                        .map(|e| Cell(e.parse().unwrap(), false))
                        .collect()
                })
                .collect()
        })
        .collect();

    for n in numbers_drawn {
        let boards: Vec<Vec<Vec<Cell>>> = boards
            .iter()
            .map(|board| {
                board
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|cell| {
                                if cell.0 == n {
                                    Cell(cell.0, true)
                                } else {
                                    Cell(cell.0, false)
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
    }
}

#[derive(Debug)]
struct Cell(i32, bool);
