use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");

    let contents: Vec<&str> = contents.split('\n').collect();
    let mut inputs:Vec<Input> = Vec::new();
    for line in contents {
        if line == "" {
            continue
        }

        let parts: Vec<&str> = line.split("   ").collect();
        inputs.push(Input(parts[0].parse().unwrap(), parts[1].parse().unwrap()));
    }

    println!("Part 1: {}", part1(inputs))

}

fn part1(mut inputs: Vec<Input>) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for inp in inputs {
        left.push(inp.0);
        right.push(inp.1);
    }
    
    left.sort();
    right.sort();

    let mut total = 0;
    for idx in 0..left.len() {
        total += (left[idx] - right[idx]).abs();
    }

    total
}

struct Input(i32, i32);