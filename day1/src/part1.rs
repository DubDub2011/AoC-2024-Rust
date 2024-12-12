pub fn diff_check(left: &Vec<i32>, right: &Vec<i32>) -> i32 {   
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