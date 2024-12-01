#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

fn main() {
    //part1();
    // part2();
}

fn part1() {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    let input = File::open("src/input.txt");
    let reader = io::BufReader::new(input.unwrap()).lines();

    for line in reader.flatten() {
        //not really needed but my text editor is adding an extra \n at the end
        if line.trim() == "" {
            continue;
        }

        let line_inputs: Vec<&str> = line.split_whitespace().collect();
        let l = line_inputs[0].trim().parse::<u32>().unwrap();
        let r = line_inputs[1].trim().parse::<u32>().unwrap();

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let mut sum: u32 = 0;

    for n in 0..left.len() {
        sum += left[n].abs_diff(right[n]);
    }

    println!("Total Sum: {}", sum);
}

fn part2() {
    let input = File::open("src/input.txt");
    let reader = io::BufReader::new(input.unwrap()).lines();

    let mut left_hash: HashMap<u32, u32> = HashMap::new();
    let mut right_hash: HashMap<u32, u32> = HashMap::new();

    for line in reader.flatten() {
        //not really needed but my text editor is adding an extra \n at the end
        if line.trim() == "" {
            continue;
        }

        let line_inputs: Vec<&str> = line.split_whitespace().collect();
        let left_key = line_inputs[0].trim().parse::<u32>().unwrap();
        let right_key = line_inputs[1].trim().parse::<u32>().unwrap();

        //ended up using HashMap for both if i keep track of how many are on the left i can just
        //multiple it and save some time
        if let Some(value) = left_hash.get_mut(&left_key) {
            *value += 1;
        } else {
            left_hash.insert(left_key, 1);
        }

        if let Some(value) = right_hash.get_mut(&right_key) {
            *value += 1;
        } else {
            right_hash.insert(right_key, 1);
        }
    }

    let mut sum: u32 = 0;

    for (k, v) in left_hash.iter() {
        if let Some(x) = right_hash.get(k) {
            sum += k * v * x;
        } else {
            sum += 0
        }
    }

    println!("Total Sum: {}", sum);
}
