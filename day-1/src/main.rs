use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<i32> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|n| n.parse::<i32>().unwrap_or_default())  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let vec = read_lines("./input");
    let mut max = 0;
    let mut sum = 0;
    for num in vec {
        if num > 0 {
            sum += num;
        } else {
            if sum > max {
                max = sum;
            }
            sum = 0;
        }
    }
    println!("{}", max);
}