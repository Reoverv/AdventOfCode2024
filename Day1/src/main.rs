use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let contents = fs::read_to_string("D:\\OneDrive - De Haagse Hogeschool\\Programming stuff\\Rust\\AdventOfCode2024\\Day1\\src\\input.txt")
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents)
}

fn part2(contents: &String) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in contents.lines() {
        let mut split = line.split("   ");

        let leftNum: u32 = split
            .next()
            .expect("Found first number")
            .parse()
            .expect("Failed to parse the first number");
        let rightNum: u32 = split
            .next()
            .expect("Found second number")
            .parse()
            .expect("Failed to parse the second number");
        left.push(leftNum);
        right.push(rightNum);
    }

    let mut total: u32 = 0;

    for i in 0..left.len() {
        let mut times = 0;
        for j in 0..right.len() {
            if left[i] == right[j] {
                times += 1;
            }
        }
        total += left[i] * times;
    }
    print!("{}", total)
}

fn part1(contents: &String) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in contents.lines() {
        let mut split = line.split("   ");

        let leftNum = split
            .next()
            .expect("Found first number")
            .parse()
            .expect("Failed to parse the first number");
        let rightNum = split
            .next()
            .expect("Found second number")
            .parse()
            .expect("Failed to parse the second number");

        left.push(leftNum);
        right.push(rightNum);
    }

    left.sort();
    right.sort();

    let mut distance = 0;

    for i in 0..left.len() {
        distance += get_distance(left[i], right[i]);
    }
    print!("{}\n", distance)
}

fn get_distance(first: u32, second: u32) -> u32 {
    if first < second {
        second - first
    } else {
        first - second
    }
}
