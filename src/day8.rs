use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;


pub fn day8() {
	let file = File::open("day8_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let commands: Vec<(String, i32)> = reader.lines().map(|e| {
        let line = e.unwrap();
        let mut line2 = line.split(" ");
        (line2.next().unwrap().to_string(), line2.next().unwrap().parse::<i32>().unwrap())
    }).collect();
    let num_commands = commands.len();

    for change_op in 0..num_commands {
        if &commands[change_op as usize].0 == "acc" { continue; }
        let mut acc = 0;
        let mut i = 0;
        let mut command_order: HashSet<i32> = HashSet::new();
        
        while command_order.insert(i) {
            if i >= num_commands as i32 {println!("Ran to completion! {:?}", acc); break}
            let (command, val) = &commands[i as usize];
            if command == "acc" {
                acc += val; i += 1;
            } else if (change_op as i32 == i && command == "nop") || (change_op as i32 != i && command == "jmp") {
                i += val
            } else {
                i += 1
            }
        }
    }
}
