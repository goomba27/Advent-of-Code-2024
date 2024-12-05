use std::{env::args, fs};

fn main() {
    // part1();
    part2();
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

/*
The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

    7 6 4 2 1: Safe without removing any level.
    1 2 7 8 9: Unsafe regardless of which level is removed.
    9 7 6 2 1: Unsafe regardless of which level is removed.
    1 3 2 4 5: Safe by removing the second level, 3.
    8 6 4 4 1: Safe by removing the third level, 4.
    1 3 6 7 9: Safe without removing any level.

Thanks to the Problem Dampener, 4 reports are actually safe!

*/

fn part2() {
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

        match is_safe(&report) {
            (true, _) => safe_reports += 1,
            (false, idx) => {
                //do other stuff
            }
        }
    }
    println!("Safe Reports: {}", safe_reports);
    //    println!("{:#?} is Safe: {} ", report, is_safe(&report));
}

fn is_safe(report: &[i32]) -> (bool, usize) {
    let dif = report[0] - report[1];
    let mut counter = 0usize;
    for level in 0..report.len() - 1 {
        match dif {
            //decrasing
            1..=3 => {
                let temp = report[level] - report[level + 1];
                if !(1..=3).contains(&temp) {
                    return (false, level);
                }
            }
            //increasing
            -3..=-1 => {
                let temp = report[level] - report[level + 1];
                if !(-3..=-1).contains(&temp) {
                    return (false, level);
                }
            }
            _ => return (false, level),
        }
        counter += 1;
    }
    if counter != report.len() - 1 {
        return (false, 0);
    }

    (true, 0)
}
