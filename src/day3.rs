use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


pub fn day3() {
    let lines = lines_from_file("day3_input.txt");
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut res: u64 = 1;
    for slope in slopes {
        let mut slope_res = 0;
        let mut x = 0;
        let width = lines[0].len();
    
        for line in (&lines[slope.1..]).iter().step_by(slope.1) {
            x += slope.0;
            if line.as_bytes()[x % width] == 35 {slope_res+=1}
        }
        res = res * slope_res;
        println!{"{:?}, {:?}", slope_res, res};
    }
    println!{"{:?}", res}
}
