use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;


fn check_range(captured_number: &str, min: i32, max: i32) -> i32 {
	if let Ok(i) = captured_number.parse::<i32>() {
		if i >= min && i <= max {
			return 1
		}
	}
	0
}

pub fn day4() {
	let file = File::open("day4_input.txt").expect("file not found!");
	let reader = BufReader::new(file);
	let key_value_pair_re = Regex::new(r"(\w+):([^\s]+)").unwrap();
	let hgt_regex = Regex::new(r"(\d+)([incm]+)").unwrap();  // Check for a number followed by in or cm
	let hcl_regex = Regex::new(r"(#[[0-9][a-f]]{6})").unwrap();
    let valid_ecl: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().collect();
    let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();

	let mut cur_count = 0;
	let mut good_passwords = 0;

	for line in reader.lines() {
		let text = &line.unwrap();
		if text == "" { 
			if cur_count == 7 {good_passwords += 1}
			cur_count = 0;
		}
		// println!("whole line: {:?}, good sofar {:?}", &text, good_passwords);
		for caps in key_value_pair_re.captures_iter(text) {
			if &caps[1] == "byr" { cur_count += check_range(&caps[2], 1920, 2002); }
			if &caps[1] == "iyr" { cur_count += check_range(&caps[2], 2010, 2020); }
			if &caps[1] == "eyr" { cur_count += check_range(&caps[2], 2020, 2030); }
			if &caps[1] == "hgt" {
				if let Some(hgt_caps) = hgt_regex.captures(&caps[2]) {
					if &hgt_caps[2] == "cm" {
						cur_count += check_range(&hgt_caps[1], 150, 193);
					} else if &hgt_caps[2] == "in" {
						cur_count += check_range(&hgt_caps[1], 59, 76);
					}
				}
			}
			if &caps[1] == "hcl" && hcl_regex.is_match(&caps[2]) { cur_count += 1; }
			if &caps[1] == "ecl" && valid_ecl.contains(&caps[2]) { cur_count += 1; }
			if &caps[1] == "pid" && pid_regex.is_match(&caps[2]) { cur_count += 1; }
			// println!("caps: {:?}, cur_count {:?}", &caps, cur_count);
		}
	}
	if cur_count == 7 {good_passwords += 1}
	println!("final: {:?}", good_passwords);
}