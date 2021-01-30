use std::fs::File;
use std::io::{BufReader, BufRead};


fn convert_to_num(captured_number: &str) -> (i32, i32) {
	let mut row = String::with_capacity(7);
	let mut col = String::with_capacity(3);

	for i in captured_number.chars() {
		match i {
			'F' => row.push('0'),
			'B' => row.push('1'),
			'L' => col.push('0'),
			'R' => col.push('1'),
			_ => (),
		}
	}
	(i32::from_str_radix(&row, 2).unwrap(), i32::from_str_radix(&col, 2).unwrap())
}

pub fn day5() {
	let file = File::open("day5_input.txt").expect("file not found!");
	let reader = BufReader::new(file);

	let mut places:Vec<bool> = vec![true; 128*8];
	// let mut seat_ids:Vec<i32> = Vec::with_capacity(970);
	for line in reader.lines() {
		let seat_coord = convert_to_num(&line.unwrap());
		// seat_ids.push(seat_coord.0 * 8 + seat_coord.1);
		places[(seat_coord.0 * 8 + seat_coord.1) as usize] = false
	}

	let mut found_first = false;
	for (index, seat) in places.iter().enumerate() {
		if !found_first && !seat { found_first=true }
		if found_first && *seat { println!("final seat: {:?}", index); break }
	}
}