mod part1;
mod part2;

use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");

    let contents: Vec<&str> = contents.split('\n').collect();
    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();

    for line in contents {
        if line == "" {
            continue
        }

        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }

    let left = &left;
    let right = &right;

    println!("Part 1: {}", part1::diff_check(left, right));
    println!("Part 2: {}", part2::similarity(left, right));
}




