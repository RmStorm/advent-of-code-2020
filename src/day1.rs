use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}

pub fn day1() {
    let v = read(File::open("day1_input.txt").unwrap()).unwrap();

    for (pos1, e1) in v.iter().enumerate() {
        for (pos2, e2) in (&v[pos1..v.len()]).iter().enumerate() {
            if e1+e2 == 2020 {println!("answer for two numbers {}", e1*e2);}
            for e3 in &v[pos2..v.len()] {
                if e1+e2+e3 == 2020 {println!("answer for three numbers {}", e1*e2*e3);}
            }
        }
    }
}