use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;


fn wrap_up_line(group_answers: &mut Vec<HashSet<u8>>) -> usize {
    let mut group_collate = group_answers.pop().unwrap();
    for set in group_answers.drain(..) {
        // Change intersection to union to solve part1
        group_collate = group_collate.intersection(&set).map(|v| *v).collect();
    }
    group_collate.len()
}


pub fn day6() {
	let file = File::open("day6_input.txt").expect("file not found!");
    let reader = BufReader::new(file);

    let mut group_answers: Vec<HashSet<u8>> = Vec::new();
    let mut final_answer: usize = 0;

	for line in reader.lines() {
        let text = &line.unwrap();
		if text == "" { 
            final_answer += wrap_up_line(&mut group_answers)
		} else {
            group_answers.push(text.bytes().collect());    
        }
    }
    final_answer += wrap_up_line(&mut group_answers);
	println!("final_answer: {:?}", final_answer);
}
