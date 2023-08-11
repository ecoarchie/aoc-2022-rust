use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<i32> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|n| n.parse::<i32>().unwrap_or_default()) // make each slice into a string
        .collect() // gather them together into a vector
}

// fn read_lines_iter(input: &str) -> String {
//     let result = input
//         .split("\n\n")
//         .map(|load| {
//             println!("Load is {}", load);
//             load.lines()
//                 .map(|item| item.parse::<u64>().unwrap())
//                 .sum::<u64>()
//         })
//         .max()
//         .unwrap();
//     result.to_string()
// }

fn main() {
    let vec = read_lines("./input");
    let mut max = 0;
    let mut sum = 0;

    // PART 1
    // for num in &vec {
    //     if *num > 0 {
    //         sum += num;
    //     } else {
    //         if sum > max {
    //             max = sum;
    //         }
    //         sum = 0;
    //     }
    // }
    // println!("{}", max);

    // PART 2
    let mut sums = Vec::new();
    for num in &vec {
        if *num > 0 {
            sum += num;
        } else {
            if sum > max {
                max = sum;
            }
            sums.push(sum);
            sum = 0;
        }
    }
    sums.sort_by(|a, b| b.cmp(a));
    let result: i32 = sums.iter().take(3).sum();
    println!("{:?}", result);
}
