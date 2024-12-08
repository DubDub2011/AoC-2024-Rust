use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");

    println!("With text:\n{contents}")
}
