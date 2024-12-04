use std::fs;
use std::ptr::null_mut;

fn main() {
    let contents: String = fs::read_to_string("D:\\OneDrive - De Haagse Hogeschool\\Programming stuff\\Rust\\AdventOfCode2024\\day4\\src\\input.txt")
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents)
}

fn part1(content: &String) {
    let chars = generate_char_array(content);
    let mut total = 0;

    for i in 0..chars.len() {
        for j in 0..chars.len() {
            if chars[i][j] == 'X' {
                total += search(i, j, &chars);
            }
        }
    }
    println!("{}", total);
}

fn generate_char_array(content: &String) -> Vec<Vec<char>> {
    content.lines().map(|line| line.chars().collect()).collect()
}

fn part2(content: &String) {
    let chars = generate_char_array(content);
    let mut total = 0;

    for i in 0..chars.len() {
        for j in 0..chars.len() {
            if chars[i][j] == 'A' {
                total += search2(i, j, &chars);
            }
        }
    }
    println!("{}", total);
}

fn search(mut i: usize, mut j: usize, chars: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut x: i32 = i as i32;
    let mut y: i32 = j as i32;

    for DIRECTION in DIRECTIONS {
        let mut found = String::new();
        found.push('X');

        for k in 0..3 {
            x += DIRECTION.0;
            y += DIRECTION.1;

            if x < 0 || y < 0 || x as usize >= chars.len() || y as usize >= chars[0].len() {
                break;
            }

            found.push(chars[x as usize][y as usize]);
        }

        if found == "XMAS" {
            total += 1;
        }
        x = i as i32;
        y = j as i32;
    }
    total
}

fn search2(mut i: usize, mut j: usize, chars: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    const DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut x: i32 = i as i32;
    let mut y: i32 = j as i32;

    let mut found = String::new();

    for DIRECTION in DIRECTIONS {
        x += DIRECTION.0;
        y += DIRECTION.1;

        if x < 0 || y < 0 || x as usize >= chars.len() || y as usize >= chars[0].len() {
            break;
        }

        found.push(chars[x as usize][y as usize]);

        if found == "SMSM" || found == "MMSS" || found == "SSMM" || found == "MSMS" {
            total += 1;
        }
        x = i as i32;
        y = j as i32;
    }

    total
}
