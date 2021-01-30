use regex::Regex;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufReader, BufRead};


fn add_permutations_to_mem(masked_adr: &mut [u8], i:usize, memory: &mut HashMap<i64, i64>, val:i64) {
    if i == masked_adr.len() {
        let masked_adr_string = masked_adr.iter().map(|e| *e as char).collect::<String>();
        memory.insert(i64::from_str_radix(&masked_adr_string, 2).unwrap(), val);
        return
    }
    if masked_adr[i] as char == 'X' {
        masked_adr[i] = '0' as u8;
        add_permutations_to_mem(masked_adr, i+1, memory, val);
        
        masked_adr[i] = '1' as u8;
        add_permutations_to_mem(masked_adr, i+1, memory, val);

        masked_adr[i] = 'X' as u8;
    } else {
        add_permutations_to_mem(masked_adr, i+1, memory, val);
    }
}


fn address_decoding(captures:regex::Captures, mask: &Vec<u8>, memory: &mut HashMap<i64, i64>) {
    let adr = (&captures[1]).parse::<i64>().unwrap();
    let val = (&captures[2]).parse::<i64>().unwrap();
    
    let mut masked_adr_string: Vec<u8> = format!("{:036b}", adr)  // must be same length as mask..
        .bytes().enumerate().map(|(i, bit)| {
            // This index 'i' is only correct if binary_number is
            // formatted with the exact same amount of leading zeros as the mask
            let mask_bit = mask[i];
            return if mask_bit as char == '0' { bit } else { mask_bit }
        }).collect();
    add_permutations_to_mem(&mut masked_adr_string, 0, memory, val)
}


fn value_decoding(captures:regex::Captures, mask: &Vec<u8>, memory: &mut HashMap<i64, i64>) {
    let adr = (&captures[1]).parse::<i64>().unwrap();
    let val = (&captures[2]).parse::<i64>().unwrap();
    
    let masked_number_string = format!("{:036b}", val)
        .bytes().enumerate().map(|(i, bit)| {
            // This index is only correct if binary_number is
            // formatted with the exact same amount of leading zeros as the mask
            let mask_bit = mask[i];
            return if mask_bit as char == 'X' { bit } else { mask_bit } as char
        }).collect::<String>();
    memory.insert(adr, i64::from_str_radix(&masked_number_string, 2).unwrap());
}


pub fn day14() {
	let file = File::open("day14_input.txt").expect("file not found!");
    let mut reader = BufReader::new(file);
    let key_value_pair_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut memory: HashMap<i64, i64> = HashMap::new(); // Instantiate empty memory dict

    let mut mask:Vec<u8> = ['X' as u8; 36].to_vec(); // Instantiate bit mask with all X's
    for line in reader.lines() {
        let text = &line.unwrap();
        println!("{:?}", key_value_pair_re.captures(text));

        match key_value_pair_re.captures(text) {
            Some(captures) => value_decoding(captures, &mask, &mut memory),
            None => mask = text.split(" = ").nth(1).unwrap().bytes().collect(), // if not a mem match overwrite mask
        }
    }
    let mut answer_part_1 = 0;
    for (key, value) in &memory {
        answer_part_1 += value;
    }
    println!("memory: {:?}", memory);
    println!("result part 1: {:?}", answer_part_1);
}
