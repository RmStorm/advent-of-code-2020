use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;


fn check_valid(number: &i64, components: &[i64]) -> bool {
    // This function checks if there are 2 numbers in 'components' that add up to the requested number
    for (i, x) in components.iter().enumerate() {
        for y in &components[i+1..] {
            if x + y == *number {
                return true
            }
        }
    }
    false
}

fn find_target_range<'a>(weakness_target: i64, code: &'a [i64]) -> &'a [i64] {
    // This function tries to find a contigous range of numbers that add up to 'weakness_target'
    for (i, x) in code.iter().enumerate() {
        let mut cur_sum = *x;
        for (ii, y) in (&code[i+1..]).iter().enumerate() {
            cur_sum += y;
            if cur_sum > weakness_target { break }
            if cur_sum == weakness_target {
                return &code[i..i+ii+2];
            }
        }
    }
    panic!("No matching range found.");
}

pub fn day9() {
	let file = File::open("day9_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let code: Vec<i64> = reader.lines().map(|e| e.unwrap().parse::<i64>().unwrap()).collect();
    let mut weakness_target = 0;

    let start = 25;

    for (i, number) in (&code[start..]).iter().enumerate() {
        let valid = check_valid(number, &code[i..i+start]);
        if !valid {
            weakness_target = *number;
            break
        }
    }
    println!("first invalid: {:?}", weakness_target);

    let yada = find_target_range(weakness_target, &code);
    println!("solution: {:?}", yada.iter().min().unwrap() + yada.iter().max().unwrap());
}
