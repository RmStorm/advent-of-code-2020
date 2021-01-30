use regex::Regex;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufReader, BufRead};


fn follow_steps(actions:&Vec<(String, i32)>) {
    let wind_directions: HashMap<i32, &str> = [(0, "N"), (90, "E"), (180, "S"), (270, "W")].iter().cloned().collect();
    let mut distances = vec![0, 0, 0, 0];
    let mut cur_dir_degrees = 90;

	for (action, value) in actions.iter() {
        let mut cur_dir = &action[..];

        match &action[..] {
            "F" => {cur_dir = wind_directions[&cur_dir_degrees]},
            "L" => {cur_dir_degrees = ((cur_dir_degrees-value)%360+360)%360},
            "R" => {cur_dir_degrees = ((cur_dir_degrees+value)%360+360)%360},
            _ => ()
        }
        match &cur_dir[..] {
            "N" => {distances[0] += value},
            "E" => {distances[1] += value},
            "S" => {distances[2] += value},
            "W" => {distances[3] += value},
            _ => ()
        }
    }
    println!("dists {:?}", (distances[0]-distances[2]).abs()+(distances[1]-distances[3]).abs());
}


fn follow_waypoint(actions:&Vec<(String, i32)>) {
    let mut waypoint:Vec<i32> = [1, 10, 0, 0].to_vec();
    let mut distances = vec![0, 0, 0, 0];

	for (action, value) in actions.iter() {
        let mut cur_dir = &action[..];

        match &action[..] {
            "F" => {
                distances = distances.iter().enumerate()
                .map(|(i, n)| n + waypoint[i]*value).collect();
            },
            "L" => {waypoint = (0..4).map(|i| waypoint[(((i+value/90)%4+4)%4) as usize]).collect()},
            "R" => {waypoint = (0..4).map(|i| waypoint[(((i-value/90)%4+4)%4) as usize]).collect()},
            _ => ()
        }
        match &cur_dir[..] {
            "N" => {waypoint[0] += value},
            "E" => {waypoint[1] += value},
            "S" => {waypoint[2] += value},
            "W" => {waypoint[3] += value},
            _ => ()
        }
    }
    println!("dists {:?}", (distances[0]-distances[2]).abs()+(distances[1]-distances[3]).abs());
}


pub fn day12() {
	let file = File::open("day12_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let key_value_pair_re = Regex::new(r"([^\d])(.+)").unwrap();
    let mut actions = Vec::new();

    for line in reader.lines() {
        let text = &line.unwrap();
        let caps = key_value_pair_re.captures(text).unwrap();
        actions.push(((&caps[1]).to_string(), (&caps[2]).parse::<i32>().unwrap()))
    }
    // println!("dists {:?}", actions);
    follow_steps(&actions);
    follow_waypoint(&actions);
}
