use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("D:\\OneDrive - De Haagse Hogeschool\\Programming stuff\\Rust\\AdventOfCode2024\\Day3\\src\\input.txt")
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part2(content: &String) {
    let regex = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let num_regex = Regex::new(r"\d+,\d+").unwrap();
    let mut results = vec![];

    for cap in regex.captures_iter(content) {
        if let Some(matched) = cap.get(0) {
            results.push(matched.as_str());
        }
    }

    let mut stop: bool = false;
    let mut num = 0;

    for res in results {
        if res == "don't()" {
            stop = true
        } else if res == "do()" {
            stop = false;
        }

        if stop == false && res != "don't()" && res != "do()" {
            for cap2 in num_regex.captures_iter(res) {
                if let Some(matched2) = cap2.get(0) {
                    let split: Vec<&str> = matched2.as_str().split(",").collect();
                    let num1: u32 = split[0]
                        .parse()
                        .unwrap_or_else(|_| panic!("Wrong: {}", split[0]));
                    let num2: u32 = split[1]
                        .parse()
                        .unwrap_or_else(|_| panic!("Wrong: {}", split[1]));

                    num += num1 * num2;
                }
            }
        }
    }
    println!("{}", num);
}
fn part1(content: &String) {
    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let num_regex = Regex::new(r"\d+,\d+").unwrap();
    let mut results = vec![];

    for cap in regex.captures_iter(content) {
        if let Some(matched) = cap.get(0) {
            for cap2 in num_regex.captures_iter(matched.as_str()) {
                if let Some(matched2) = cap2.get(0) {
                    results.push(matched2.as_str());
                }
            }
        }
    }

    let mut num = 0;

    for result in results {
        let split: Vec<&str> = result.split(",").collect();
        let num1: u32 = split[0].parse().unwrap();
        let num2: u32 = split[1].parse().unwrap();

        num += num1 * num2;
    }

    println!("{:?}", num);
}
