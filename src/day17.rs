use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(PartialEq, Clone, Copy, Debug)]
enum Cube {
    Active,
    InActive,
    CountA(i8),
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cube::InActive => write!(f, "."),
            Cube::Active => write!(f, "#"),
            Cube::CountA(yada) => write!(f, "{:?}", yada)
        }
    }
}

#[derive(Debug)]
struct Pocket3D<'a>(&'a [Vec<Vec<Cube>>]);

impl fmt::Display for Pocket3D<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut active = 0;
        for line_index in 0..self.0[0].len() {
                for layer in self.0 {
                    for cell in layer[line_index].iter() {
                        write!(f, "{}", cell)?;
                        if let Cube::Active = cell {active+=1}
                    }
                    write!(f, "  ")?;
                }
                write!(f, "\n")?;
        }
        write!(f, "{}\n", active)?;
        Ok(())
    }
}

fn count_adjacent_3d(layout: &Vec<Vec<Vec<Cube>>>, z:usize, y:usize, x: usize) -> u8 {
    let mut total_adjacent = 0;
    for z_offset in -1..2 {
        for y_offset in -1..2 {
            for x_offset in -1..2 {
                if z_offset == 0 && y_offset == 0 && x_offset == 0 {continue}

                let cur_z = z as i8+z_offset;
                let cur_y = y as i8+y_offset;
                let cur_x = x as i8+x_offset;
        
                if (cur_z < 0) | (cur_z >= layout.len() as i8) {continue}
                if (cur_y < 0) | (cur_y >= layout[0].len() as i8) {continue}
                if (cur_x < 0) | (cur_x >= layout[0][0].len() as i8) {continue}
                if let Cube::Active = layout[cur_z as usize][cur_y as usize][cur_x as usize] {total_adjacent+=1}
            }
        }
    }
    total_adjacent
}


fn simulate_3d(core_layout:&Vec<Vec<Cube>>) {
    let mut layout = vec![vec![vec![Cube::InActive; core_layout[0].len()+12]; core_layout.len()+12]; 13];
    for i in 0..core_layout[0].len() {
        for ii in 0..core_layout.len() {
            layout[6][ii+6][i+6] = core_layout[ii][i]
        }
    }
    
    // println!("Starting arrangement:");
    // println!("{}", Pocket3D(&layout[1..]));

    for _ in 0..6 {
        let mut next_layout = vec![vec![vec![Cube::InActive; core_layout[0].len()+12]; core_layout.len()+12]; 13];
        for (z, layer) in layout.iter().enumerate() {
            for (y, line) in layer.iter().enumerate() {
                for (x, cell) in line.iter().enumerate() {
                    let adjacent = count_adjacent_3d(&layout, z, y, x);
                    // next_layout[z][y][x] = Cube::CountA(adjacent as i8);
                    if *cell == Cube::InActive && adjacent == 3 {
                        next_layout[z][y][x] = Cube::Active
                    } else if (adjacent == 2 || adjacent == 3) && *cell == Cube::Active {
                        next_layout[z][y][x] = Cube::Active
                    } else {
                        next_layout[z][y][x] = Cube::InActive
                    }
                }
            }
        }
        layout = next_layout;
        // println!("{}", Pocket3D(&layout));
    }
    let mut total_active = 0;
    for layer in layout.iter() {
        for line in layer.iter() {
            for cell in line.iter() {
                if let Cube::Active = cell {total_active+=1}
            }
        }
    }
    println!("{}", total_active);
}

fn count_adjacent_4d(layout: &Vec<Vec<Vec<Vec<Cube>>>>, w:usize, z:usize, y:usize, x: usize) -> u8 {
    let mut total_adjacent = 0;
    for w_offset in -1..2 {
        for z_offset in -1..2 {
            for y_offset in -1..2 {
                for x_offset in -1..2 {
                    if w_offset == 0 && z_offset == 0 && y_offset == 0 && x_offset == 0 {continue}

                    let cur_w = w as i8 + w_offset;
                    let cur_z = z as i8 + z_offset;
                    let cur_y = y as i8 + y_offset;
                    let cur_x = x as i8 + x_offset;
            
                    if (cur_w < 0) | (cur_w >= layout.len() as i8) {continue}
                    if (cur_z < 0) | (cur_z >= layout[0].len() as i8) {continue}
                    if (cur_y < 0) | (cur_y >= layout[0][0].len() as i8) {continue}
                    if (cur_x < 0) | (cur_x >= layout[0][0][0].len() as i8) {continue}
                    if let Cube::Active = layout[cur_w as usize][cur_z as usize][cur_y as usize][cur_x as usize] {total_adjacent+=1}
                }
            }
        }
    }
    total_adjacent
}


fn simulate_4d(core_layout:&Vec<Vec<Cube>>) {
    let mut layout = vec![vec![vec![vec![Cube::InActive; core_layout[0].len()+12]; core_layout.len()+12]; 13]; 13];
    for i in 0..core_layout.len() {
        for ii in 0..core_layout[0].len() {
            layout[6][6][i+6][ii+6] = core_layout[i][ii]
        }
    }
    
    // println!("Starting arrangement:");
    // println!("{}", Pocket3D(&layout[1..]));

    for _ in 0..6 {
        let mut next_layout = vec![vec![vec![vec![Cube::InActive; core_layout[0].len()+12]; core_layout.len()+12]; 13]; 13];
        for (w, cube) in layout.iter().enumerate() {
            for (z, layer) in cube.iter().enumerate() {
                for (y, line) in layer.iter().enumerate() {
                    for (x, cell) in line.iter().enumerate() {
                        let adjacent = count_adjacent_4d(&layout, w, z, y, x);
                        // next_layout[w][z][y][x] = Cube::CountA(adjacent as i8);
                        if *cell == Cube::InActive && adjacent == 3 {
                            next_layout[w][z][y][x] = Cube::Active
                        } else if (adjacent == 2 || adjacent == 3) && *cell == Cube::Active {
                            next_layout[w][z][y][x] = Cube::Active
                        } else {
                            next_layout[w][z][y][x] = Cube::InActive
                        }
                    }
                }
            }
        }
        layout = next_layout;
        // println!("{}", Pocket3D(&layout));
    }
    let mut total_active = 0;
    for cube in layout.iter() {
        for layer in cube.iter() {
            for line in layer.iter() {
                for cell in line.iter() {
                    if let Cube::Active = cell {total_active+=1}
                }
            }
        }
    }
    println!("{}", total_active);
}

pub fn day17() {
	let file = File::open("day17_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let core_layout: Vec<Vec<Cube>> = reader.lines().map(|e| e.unwrap().bytes().map(|e| {
        match e {
            35 => Cube::Active,
            46 => Cube::InActive,
            _ => panic!("invalid character matched"),
        }
    }).collect()).collect();
    
    simulate_3d(&core_layout);
    simulate_4d(&core_layout);
}
