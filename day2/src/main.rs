use std::{env::args, fs};

fn main() {
    part1();
}

/*
The levels are either all increasing or all decreasing.
Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:
    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
*/

fn part1() {
    let input = fs::read_to_string("src/input.txt");

    let mut safe_reports = 0u32;
    for line in input.unwrap().lines() {
        if line.trim() == "" {
            continue;
        }

        let report: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.trim().parse::<i32>().unwrap())
            .collect();

        //assuming is not gonna be empty
        let dif = report[0] - report[1];
        let mut counter = 0usize;
        for level in 0..report.len() - 1 {
            match dif {
                //decrasing
                1..=3 => {
                    let temp = report[level] - report[level + 1];
                    if !(1..=3).contains(&temp) {
                        break;
                    }
                }
                //increasing
                -3..=-1 => {
                    let temp = report[level] - report[level + 1];
                    if !(-3..=-1).contains(&temp) {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
            counter += 1;
        }
        if counter == report.len() - 1 {
            safe_reports += 1;
        }
    }
    println!("Number of Safe Reports: {}", safe_reports);
}
