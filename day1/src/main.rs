use std::fs;
use std::collections::HashMap;

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

    println!("Part 1: {}", diff_check(left, right));
    println!("Part 2: {}", similarity(left, right));
}

fn diff_check(left: &Vec<i32>, right: &Vec<i32>) -> i32 {   
    let mut left = left.clone();
    let mut right = right.clone();
    left.sort();
    right.sort();

    let mut total = 0;
    for idx in 0..left.len() {
        total += (left[idx] - right[idx]).abs();
    }

    total
}

fn similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut index = HashMap::new();

    for idx in 0..right.len() {
        let similarity_value = right[idx];
        match index.get(&similarity_value) {
            Some(new_value) => { index.insert(similarity_value, new_value + 1); }
            None => { index.insert(similarity_value, 1);}
        }
    }

    let mut sum = 0;
    for idx in 0..left.len() {
        let val = left[idx];
        match index.get(&val) {
            Some(count) => { sum = sum + count * val;}
            None => ()
        }
    }

    sum
}

