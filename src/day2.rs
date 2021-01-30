use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use regex::Regex;

fn read<R: Read>(io: R) -> Result<(i64, i64), Error> {
    let br = BufReader::new(io);
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(\w+)$").unwrap();

    let mut rule1_correct = 0;
    let mut rule2_correct = 0;
    for line in br.lines() {
        let line_content = line?;
        let cap = re.captures(&line_content).unwrap();
        let limits = (&cap[1].parse::<i32>().unwrap(), &cap[2].parse::<i32>().unwrap());
        let char_rule = cap[3].as_bytes()[0];
        let password = cap[4].as_bytes();

        let mut char_count = 0;
        for password_char in password {
            if char_rule == *password_char { char_count += 1 }
        }
        if char_count >= *limits.0 && char_count <= *limits.1 { rule1_correct += 1 }

        let mut password_correct = 0;
        if password[(*limits.0 - 1) as usize] == char_rule { password_correct ^= 1 }
        if password[(*limits.1 - 1) as usize] == char_rule { password_correct ^= 1 }
        rule2_correct += password_correct
    }
    Ok((rule1_correct, rule2_correct))
}

pub fn day2() {
    let correct_passwords = read(File::open("day2_input.txt").unwrap()).unwrap();
    println!("{:?}", correct_passwords);
}
