use std::collections::HashMap;
use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn day15() {
	let file = File::open("day15_input.txt").expect("file not found!");
    let mut start_sequence = String::new();
    BufReader::new(file).read_line(&mut start_sequence).unwrap();
    let start_sequence = Vec::from_iter(start_sequence.split(",").map(|e| e.parse::<usize>().unwrap()));
    println!("start_sequence {:?}", start_sequence);

    let mut sequence = Vec::with_capacity(30000001);
    let mut last_seen: HashMap<usize, usize> = HashMap::new(); // Instantiate empty memory dict

    for (i, e) in start_sequence.iter().enumerate() {
        sequence.push(*e);
        if i < (start_sequence.len()-1) {last_seen.insert(*e, i as usize);}
    }
    
    for i in start_sequence.len()..2020 {
        match last_seen.get_mut(&sequence[i-1]) {
            Some(val) => {
                sequence.push(i-1-*val);
                *val = i-1;
            },
            None => {
                last_seen.insert(sequence[i-1], i-1);
                sequence.push(0);
            },
        }
    }
    println!("{:?}", sequence.last());
}
