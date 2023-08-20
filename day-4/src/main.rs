use std::fs;

use itertools::Itertools;
use std::ops::RangeInclusive;

// solution from fasterthanli.me
trait InclusiveRangeExt {
    // part 1
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    // part 2
    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {
    // solution from fasterthanli.me
    // part 1
    let redundant = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have a start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have a pair of ranges")
        })
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count();
    dbg!(redundant);

    //my solution
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut count_incl = 0;
    let mut count_overlaps = 0;
    for l in file.lines() {
        let v = l.split(",").collect::<Vec<&str>>();
        // println!("{:?}", v);
        let left_min = v[0]
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .min()
            .unwrap();
        let left_max = v[0]
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .max()
            .unwrap();
        let right_min = v[1]
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .min()
            .unwrap();
        let right_max = v[1]
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .max()
            .unwrap();
        // part 1
        if (left_min <= right_min && left_max >= right_max)
            || (right_min <= left_min && right_max >= left_max)
        {
            count_incl += 1;
        }
        // part 2
        if (left_min <= right_min && left_max >= right_min)
            || (right_min <= left_min && right_max >= left_min)
        {
            count_overlaps += 1;
        }
    }
    println!("{count_incl}");
    println!("{count_overlaps}");
}
