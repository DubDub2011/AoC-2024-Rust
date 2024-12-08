use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");
    println!("File contents: {}", contents);

    let contents: Vec<&str> = contents.split('\n').collect();
    let mut inputs:Vec<Input> = Vec::new();
    for line in contents {
        if line == "" {
            continue
        }

        let parts: Vec<&str> = line.split("   ").collect();
        inputs.push(Input(parts[0].parse().unwrap(), parts[1].parse().unwrap()));
    }

    for inp in inputs {
        println!("Input: {}, {}", inp.0, inp.1)
    }

}


struct Input(i32, i32);