use std::collections::HashMap;

pub fn similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
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
