use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Clone, Copy, Debug)]
enum WaitingPlace {
    Floor,
    Empty,
    Taken,
}


impl fmt::Display for WaitingPlace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            WaitingPlace::Floor => '.',
            WaitingPlace::Empty => 'L',
            WaitingPlace::Taken => '#',
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
struct WPvec<'a>(&'a Vec<WaitingPlace>);

impl fmt::Display for WPvec<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in self.0 {
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}


const DIRECTIONS:[[i8; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];

fn count_los(layout: &Vec<Vec<WaitingPlace>>, row:usize, col: usize, adjacent: bool) -> u8 {
    let mut total_taken = 0;
    for dir in DIRECTIONS.iter() {
        let mut cur_row = row as i8;
        let mut cur_col = col as i8;
        while true {
            cur_row += dir[0];
            if (cur_row < 0) | (cur_row as usize >= layout.len()) {break}
            cur_col += dir[1];
            if (cur_col < 0) | (cur_col as usize >= layout[0].len()) {break}
            if let WaitingPlace::Taken = layout[cur_row as usize][cur_col as usize] {total_taken+=1; break}
            if let WaitingPlace::Empty = layout[cur_row as usize][cur_col as usize] {break}
            if adjacent {break}
        }
    }
    
    // println!("{:?}:{:?} {:?}:{:?}", row_start, row_end, col_start, col_end);
    // println!("row: {:?}, col: {:?}", row, col);
    // println!("place: {:?}, adjacent taken: {:?}\n", layout[row][col], total_taken);
    total_taken
}


pub fn day11() {
	let file = File::open("day11_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let mut layout: Vec<Vec<WaitingPlace>> = reader.lines().map(|e| e.unwrap().bytes().map(|e| {
        match e {
            76 => WaitingPlace::Empty,
            35 => WaitingPlace::Taken,
            46 => WaitingPlace::Floor,
            _ => panic!("invalid character matched"),
        }
    }).collect()).collect();

    println!("Starting arrangement:");
    for l in layout.iter() {
        println!("{}", WPvec(&l));
    }

    let mut counter = 0;
    let stop_value = 200;
    while counter < stop_value {
        // println!("\n");
        // for l in layout.iter() {
        //     println!("{}", WPvec(&l));
        // }
        let mut next_layout = vec![vec![WaitingPlace::Floor; layout[0].len()]; layout.len()];
        for (row, places) in layout.iter().enumerate() {
            for (column, place) in places.iter().enumerate() {
                let adjacent = count_los(&layout, row, column, false);
                if adjacent == 0 && *place == WaitingPlace::Empty {
                    next_layout[row][column] = WaitingPlace::Taken
                } else if adjacent >= 5 && *place == WaitingPlace::Taken {
                    next_layout[row][column] = WaitingPlace::Empty
                } else {
                    next_layout[row][column] = layout[row][column]
                }
            }
        }
        if layout == next_layout {
            break
        }
        layout = next_layout;
        counter += 1;
    }
    if counter == stop_value {panic!("Seats did not stabilize increase stop_value or fix bug!")}

    let occupied_chairs:usize = layout.iter().map(|v| v.iter().filter(|&n| WaitingPlace::Taken == *n).count()).sum();
    println!("\niterations: {}\nfinal count {}", counter, occupied_chairs);
}
