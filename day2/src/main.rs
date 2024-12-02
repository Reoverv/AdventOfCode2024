use std::fs;
use std::fs::FileType;

fn main() {
    let contents = fs::read_to_string("D:\\OneDrive - De Haagse Hogeschool\\Programming stuff\\Rust\\AdventOfCode2024\\Day2\\src\\input.txt")
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(content: &String) {
    let mut total = 0;

    for line in content.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        if check_part1(&split) {
            total += 1
        }
    }

    println!("{}", total);
}

fn part2(content: &String) {
    let mut total = 0;

    for line in content.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        //println!("{} - {:?}", check_part2(&split), split);
        if check_part2(&split) {
            total += 1
        }
    }

    println!("{}", total);
}

fn check_part2(split: &Vec<&str>) -> bool {
    let mut fault = 0;
    if check_part1(split){
        fault += 1
    };
    
    for i in 0..split.len() {
        let mut clone = split.clone();
        clone.remove(i);

        if check_part1(&clone) && fault == 0 {
            fault += 1;
            break
        };
    }
    


    fault == 1
}

fn check_part1(split: &Vec<&str>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    let mut difference: i32 = 0;

    let nums: Vec<i32> = split.iter().map(|s| s.parse().expect("error")).collect();

    for i in 0..split.len() - 1 {
        difference += (nums[i] - nums[i + 1]).abs();

        if nums[i] <= nums[i + 1] {
            decreasing = false;
        } else if nums[i] >= nums[i + 1] {
            increasing = false;
        }

        if nums[i] == nums[i + 1] {
            increasing = false;
            decreasing = false;
        }

        if difference > 3 {
            increasing = false;
            decreasing = false;
        }
        difference = 0;
    }

    increasing || decreasing
}
