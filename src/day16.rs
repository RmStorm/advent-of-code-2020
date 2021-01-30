use regex::Regex;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn check_number(ranges:&Vec<Vec<i32>>, num: &i32) -> bool {
    for range in ranges {
        if range.contains(num) {return true}
    }
    false
}

pub fn day16() {
    let mut ranges:Vec<Vec<Vec<i32>>> = Vec::new();
    let lines = lines_from_file("day16_input.txt");
	let range_regex = Regex::new(r"([^\d]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut line_iter = lines.into_iter();
    let mut rule_names:Vec<String> = Vec::new();
    while let Some(line) = line_iter.next() {
        if line == "" {break}
        let captures = range_regex.captures(&line).unwrap();
        rule_names.push(captures[1].to_string());
        let nums:Vec<i32> = (2..6).map(|i| captures[i].parse::<i32>().unwrap()).collect();
        ranges.push([(nums[0]..(nums[1]+1)).collect(), (nums[2]..(nums[3]+1)).collect()].to_vec());
    }

    line_iter.next();  // skip "your ticket:" line
    let mut ticket = Vec::new();
    while let Some(line) = line_iter.next() {
        if line == "" {break}
        ticket = Vec::from_iter(line.split(",").map(|e| e.parse::<i32>().unwrap()));
    }

    let mut sol_part_1 = 0;
    let mut valid_lines = Vec::new();

    line_iter.next();  // skip "nearby tickets:" line
    while let Some(line) = line_iter.next() {
        let ticket_numbers = Vec::from_iter(line.split(",").map(|e| e.parse::<i32>().unwrap()));
        let mut line_valid = true;
        for num in ticket_numbers.iter() {
            if !ranges.iter().any(|e| check_number(e, &num)) {
                sol_part_1 += num;
                line_valid = false;
            }
        }
        if line_valid {valid_lines.push(ticket_numbers)};
    }
    println!("sol_part_1 {:?}", sol_part_1);

    let mut options:Vec<Vec<usize>> = vec![(0..rule_names.len()).collect(); rule_names.len()];
    // First find out which options are valid for which numbers by filtering out the invalid options
    for line in valid_lines {
        for (i, num) in line.iter().enumerate() {
            options[i].retain(|range_index| check_number(&ranges[*range_index], &num));
        }
    }

    // If a number has only one valid rule, assign it and remove the rule as a possibility from the other numbers
    let mut answers: HashMap<&str, usize> = HashMap::new();

    for _ in 0..options.len() {  // This is the maximum number the inner filter loop might have to be repeated to complete..
        for i in 0..options.len() {
            if options[i].len() == 1 { // Number i has only valid rule. Assign it!
                let matched_range_index = options[i][0];
                answers.insert(&rule_names[matched_range_index], i);
                for ii in 0..options.len() {
                    options[ii].retain(|range_index| *range_index != matched_range_index);
                }
            }
        }
    }
    
    let mut sol_part_2:u64 = 1;
    for (key, value) in answers.iter() {
        if key.contains("departure") { sol_part_2 *= ticket[*value as usize] as u64 }
    }
    println!("sol_part_2 {:?}", sol_part_2);
}
