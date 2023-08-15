use std::{fs, collections::HashSet};
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();

//     let input = "vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw";
    let mut sum = 0;

    // part 1
    (file.lines()).into_iter().for_each(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        for ch in left.chars() {
            if right.contains([ch]) {
                let num =  match ch.is_uppercase() {
                    true => ch as u32 - 38,
                    false => ch as u32 - 96,
                };
                sum += num;
                break;
            }
        }
    });

    //part 2
    let mut iter = file.lines().collect::<Vec<&str>>();
    for chunk in iter.chunks_mut(3) {
        let r = chunk.iter().map(|c| c.chars().collect::<HashSet<char>>()).collect::<Vec<HashSet<char>>>();
        let intersect = r[0].intersection(&r[1]).collect::<Vec<_>>();
        for ch in intersect {
            if r[2].contains(ch) {
                let num =  match ch.is_uppercase() {
                    true => *ch as u32 - 38,
                    false => *ch as u32 - 96,
                };
                sum += num;
                break;
                
            }
        }
    }
    println!("{}", sum);
}
