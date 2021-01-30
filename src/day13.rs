use regex::Regex;

use std::fs::File;
use std::io::{BufReader, BufRead};



pub fn day13() {
	let file = File::open("day13_input.txt").expect("file not found!");
    let mut reader = BufReader::new(file);
    
    let mut start_time = String::new();
    reader.read_line(&mut start_time);
    let start_time = start_time.trim().parse::<i32>().unwrap();

    println!("dists {:?}", start_time);
    
    let numbers: Vec<[i32;2]> = reader.split(b',').enumerate()
        .map(|(i, e)| {
            match std::str::from_utf8(&e.unwrap()).unwrap().parse::<i32>() { // uhm... I'm not sure what to say.
                Ok(number) => Ok([number, i as i32]),
                Err(error) => Err(error),
            }
        })
        .filter_map(Result::ok)
        .collect();
    // Set minimum waiting time to the maximum possible value
    let max_bus = &numbers.iter().max().unwrap();
    let yada:u128 = 100000000000000;
    println!("Result part 1: {:?}", yada);
    let mut cur_min_waiting_time = *(&numbers.iter().map(|a| a[0]).max().unwrap());
    let mut cur_bus = 0;

    for [bus, _delay] in &numbers {
        if bus-start_time%bus < cur_min_waiting_time {
            cur_min_waiting_time = bus-start_time%bus;
            cur_bus = *bus;
        }
    }
    println!("Result part 1: {:?}", cur_min_waiting_time * cur_bus);

    let t = 1068781;
    println!("max_bus: {:?}, ", max_bus);
    
    let mut found = false;
    let mut i:u128 = 1;
    let mut t:u128 = 0;
    let mb0 = max_bus[0] as u128;
    let mb1 = max_bus[1] as u128;

    // This works but it takes like 19 hours.... It's literally retarded
    while !found {
        t = mb0*i+mb0-mb1;
        found = true;
        for [bus, delay] in &numbers {
            if (*delay as u128 +t)% (*bus as u128) != 0 {
                found = false;
                break
            }
        }
        i += 1;
    }
    println!("{:?}", t)
}
