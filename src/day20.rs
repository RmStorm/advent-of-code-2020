use std::fmt;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pixel {
    Active,
    InActive,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pixel::InActive => write!(f, "."),
            Pixel::Active => write!(f, "#"),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Tile {
    id: i32,
    pixels: Vec<Vec<Pixel>>
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right
}

const SIDES: [(fn(&Tile) -> Box<(dyn DoubleEndedIterator<Item = &Pixel> + '_)>, Side); 4] = [
    (Tile::top, Side::Top), 
    (Tile::bottom, Side::Bottom), 
    (Tile::left, Side::Left), 
    (Tile::right, Side::Right), 
];

#[derive(Debug)]
struct Relation {
    side1: Side,
    side2: Side,
    reversed: bool,
    id: i32
}

impl Tile {
    fn top(&self) -> Box<dyn DoubleEndedIterator<Item = &Pixel> + '_ > {
        Box::new(self.pixels[0].iter())
    }
    fn bottom(&self) -> Box<dyn DoubleEndedIterator<Item = &Pixel> + '_ > {
        Box::new(self.pixels[self.pixels.len()-1].iter())
    }
    fn left(&self) -> Box<dyn DoubleEndedIterator<Item = &Pixel> + '_ > {
        Box::new(self.pixels.iter().map(|line| &line[0]))
    }
    fn right(&self) -> Box<dyn DoubleEndedIterator<Item = &Pixel> + '_ > {
        Box::new(self.pixels.iter().map(|line| &line[line.len()-1]))
    }
    fn find_all_matches(&self, others: &HashMap<i32, Tile>) -> Vec<Relation> {
        let mut matched_sides = Vec::new();
        for (_, other_tile) in others {
            if self.id == other_tile.id { continue }
            if let Some(matched_side) = self.find_match(other_tile) {
                matched_sides.push(matched_side)
            }
        }
        matched_sides
    }
    fn find_match(&self, other_tile: &Tile) -> Option<Relation> {
        for (side1_iter, side1) in SIDES.iter() {
            for (side2_iter, side2) in SIDES.iter() {
                if side1_iter(self).zip(side2_iter(other_tile)).all(|(e1, e2)| e1 == e2) {
                    return Some(Relation{side1: *side1, side2: *side2, reversed: false, id: other_tile.id})
                }
                if side1_iter(self).zip(side2_iter(other_tile).rev()).all(|(e1, e2)| e1 == e2) {
                    return Some(Relation{side1: *side1, side2: *side2, reversed: true, id: other_tile.id})
                }
            }
        }
        None
    }
    fn get_copy(&self) -> Tile {
        Tile{id: self.id, pixels: self.pixels.iter().map(
            |pixel_line| pixel_line.iter().map(|e| *e).collect()
        ).collect()}
    }
    fn get_flipped_TB(&self) -> Tile {
        Tile{id: self.id, pixels: self.pixels.iter().rev().map(
            |pixel_line| pixel_line.iter().map(|e| *e).collect()
        ).collect()}
    }
    fn get_flipped_LR(&self) -> Tile {
        Tile{id: self.id, pixels: self.pixels.iter().map(
            |pixel_line| pixel_line.iter().rev().map(|e| *e).collect()
        ).collect()}
    }
    fn get_transposed(&self) -> Tile {
        let mut new_pixels = Vec::new();
        for _ in self.pixels[0].iter() {new_pixels.push(Vec::new())}
        for pixel_line in self.pixels.iter() {
            for (i, pixel) in pixel_line.iter().enumerate() {
                new_pixels[i].push(*pixel)
            }
        }
        Tile{id: self.id, pixels: new_pixels}
    }

    fn count_active(&self) -> i32 {
        let mut active = 0;
        for pixel_line in self.pixels.iter() {
            for pixel in pixel_line.iter() {
                if let Pixel::Active = pixel {
                    active += 1;
                }
            }
        }
        active
    }
}


impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile: {}\n", self.id)?;
        for pixel_line in &self.pixels {
                for pixel in pixel_line {
                    write!(f, "{}", pixel)?;
                }
                write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Picture {
    tiles: Vec<Vec<Tile>>
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "picture:\n")?;
        for tile_line in self.tiles.iter() {
            for i in 0..10 {
                for tile in tile_line {
                    for pixel in tile.pixels[i].iter() {
                        write!(f, "{}", pixel)?;
                    }
                    write!(f, " ")?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


fn read_tiles(file: File) -> HashMap<i32, Tile> {
    let reader = BufReader::new(file);
    let line_regex = Regex::new(r"^Tile (\d+):$").unwrap();

    let mut tiles: HashMap<i32, Tile> = HashMap::new();
    // let mut tiles= Vec::new();
    let mut new_tile = true;
    let mut cur_tile_id = 0;
    let mut cur_lines = Vec::new();

    for line in reader.lines() {
        let text = &line.unwrap();
        if new_tile {
            let captures = line_regex.captures(&text).unwrap();
            cur_tile_id = captures[1].parse::<i32>().unwrap();
            new_tile = false;
        } else if text == "" {
            // tiles.push(Tile{id: cur_tile_id, pixels: cur_lines});
            tiles.insert(cur_tile_id, Tile{id: cur_tile_id, pixels: cur_lines});
            new_tile = true;
            cur_lines = Vec::new();
        } else {
            let pixel_line: Vec<Pixel> = text.bytes().map(|e| {
                match e {
                    35 => Pixel::Active,
                    46 => Pixel::InActive,
                    _ => panic!("invalid character matched"),
                }
            }).collect();
            cur_lines.push(pixel_line);
        }
    }
    // tiles.push(Tile{id: cur_tile_id, pixels: cur_lines});
    tiles.insert(cur_tile_id, Tile{id: cur_tile_id, pixels: cur_lines});
    tiles
}

fn check_shape(shape: &Vec<Vec<char>>, picture: &Vec<Vec<Pixel>>, position:[usize; 2]) -> bool {
    for (i, shape_line) in shape.iter().enumerate() {
        for (ii, shape_entry) in shape_line.iter().enumerate() {
            if shape_entry == &'#' {
                // println!("Checking {:?}", [i+position[0], ii+position[1]]);
                if let Pixel::InActive = picture[i+position[0]][ii+position[1]] {
                    return false
                }
            }
        }
    }
    true
}

fn find_all_shapes_in_tile(shape: &Vec<Vec<char>>, picture: &Vec<Vec<Pixel>>) -> i32 {
    let mut found_shapes = 0;
    for i in 0..picture.len()-shape.len()+1 {
        for ii in 0..picture[0].len()-shape[0].len()+1 {
            // println!("{}, {}", i, ii);
            let found = check_shape(shape, picture, [i, ii]);
            if found {
                println!("{:?}", [i, ii]);
                found_shapes += 1;
            }
        }
    }
    println!("this many shapes: {}", found_shapes);

    found_shapes
}

pub fn day20() {
	let file = File::open("day20_input.txt").expect("file not found!");
    let tiles = read_tiles(file);
    let mut picture_tiles: Vec<Vec<Tile>> = Vec::new();
    for _ in 0..(tiles.len() as f64).sqrt() as usize { picture_tiles.push(Vec::new()) }
    let mut picture = Picture{tiles: picture_tiles};
    
    // let mut res = 1;
    for (_, tile) in &tiles {
        // println!("\n\nStarting inspection of \n{}\n", tile);        
        let matched_sides = tile.find_all_matches(&tiles);
        println!("{:?}", matched_sides);

        if matched_sides.len() == 2 {
            let used_borders = [matched_sides[0].side1, matched_sides[1].side1];
            if used_borders.contains(&Side::Top) {
                if used_borders.contains(&Side::Right) {
                    picture.tiles[0].push(tile.get_flipped_TB())
                } else {
                    picture.tiles[0].push(tile.get_flipped_LR().get_flipped_TB())
                }
            } else {
                if used_borders.contains(&Side::Right) {
                    picture.tiles[0].push(tile.get_copy())
                } else {
                    picture.tiles[0].push(tile.get_flipped_LR())
                }
            }
            // res *= tile.id as u128;
            // println!("Those sides are used {:?}", used_borders);
            // println!("This is a corner {:?}", tile.id);
            break
        }
    }
    // println!("This is the current pic {:?}", picture);
    // println!("This is the current pic {}", picture.tiles[0][0]);
    for i in 0..picture.tiles.len()-1 {
        for ii in 0..picture.tiles.len() {
            // println!("Starting on: {:?}", i);
            for matched_side in picture.tiles[i][ii].find_all_matches(&tiles) {
                // println!("Matched: {:?}", matched_side);
                if let Side::Right = matched_side.side1 {
                    if i == 0 {
                        match matched_side.side2 {
                            Side::Top => {
                                let mut new_tile = tiles[&matched_side.id].get_transposed();
                                if matched_side.reversed { new_tile = new_tile.get_flipped_TB() }
                                picture.tiles[i].push(new_tile)
                            },
                            Side::Bottom => {
                                let mut new_tile = tiles[&matched_side.id].get_transposed();
                                new_tile = new_tile.get_flipped_LR();
                                if matched_side.reversed { new_tile = new_tile.get_flipped_TB() }
                                picture.tiles[i].push(new_tile)
                            },
                            Side::Left => {
                                let mut new_tile = tiles[&matched_side.id].get_copy();
                                if matched_side.reversed { new_tile = new_tile.get_flipped_TB() }
                                picture.tiles[i].push(new_tile)
                            },
                            Side::Right => {
                                let mut new_tile = tiles[&matched_side.id].get_flipped_LR();
                                if matched_side.reversed { new_tile = new_tile.get_flipped_TB() }
                                picture.tiles[i].push(new_tile)
                            },
                        }
                    }
                } else if let Side::Bottom = matched_side.side1 {
                    match matched_side.side2 {
                        Side::Top => {
                            let mut new_tile = tiles[&matched_side.id].get_copy();
                            if matched_side.reversed { new_tile = new_tile.get_flipped_LR() }
                            picture.tiles[i+1].push(new_tile)
                        },
                        Side::Bottom => {
                            let mut new_tile = tiles[&matched_side.id].get_flipped_TB();
                            if matched_side.reversed { new_tile = new_tile.get_flipped_LR() }
                            picture.tiles[i+1].push(new_tile)
                        },
                        Side::Left => {
                            let mut new_tile = tiles[&matched_side.id].get_transposed();
                            if matched_side.reversed { new_tile = new_tile.get_flipped_LR() }
                            picture.tiles[i+1].push(new_tile)
                        },
                        Side::Right => {
                            let mut new_tile = tiles[&matched_side.id].get_transposed();
                            new_tile = new_tile.get_flipped_TB();
                            if matched_side.reversed { new_tile = new_tile.get_flipped_LR() }
                            picture.tiles[i+1].push(new_tile)
                        },
                    }
                }
            }
        }
    }

    let mut no_borders = Vec::new();
    let mut cur_line: Vec<Pixel> = Vec::new();
    for tile_line in picture.tiles.iter() {
        for i in 1..9 {
            for tile in tile_line {
                for (i, pixel) in tile.pixels[i].iter().enumerate() {
                    if (i != 0) & (i != 9) {
                        cur_line.push(*pixel)
                    }
                }
            }
            no_borders.push(cur_line);
            cur_line = Vec::new()
        }
    }
    let big_tile = Tile{id:0, pixels:no_borders};
    println!("This is the current pic\n{}", picture);
    println!("This is the image\n{}", big_tile);
    println!("With this many active pixels: {}", big_tile.count_active());

    let sea_monster:Vec<Vec<char>> = vec![
        "                  # ".chars().into_iter().collect(),
        "#    ##    ##    ###".chars().into_iter().collect(),
        " #  #  #  #  #  #   ".chars().into_iter().collect()
    ];
    println!("This is the sea monster");
    let mut monster_pixels = 0;
    for pixel_line in sea_monster.iter() {
        for shape_char in pixel_line {
            print!("{}", shape_char);
            if shape_char == &'#' {
                monster_pixels += 1;
            }
        }
        println!("|");
    }
    println!("It contains {} active pixels \n", monster_pixels);

    find_all_shapes_in_tile(&sea_monster, &big_tile.pixels);
    println!("Checking tb flipped");
    let new_tile = big_tile.get_flipped_TB();
    find_all_shapes_in_tile(&sea_monster, &new_tile.pixels);
    println!("Checking lr flipped");
    let new_tile = big_tile.get_flipped_LR();
    find_all_shapes_in_tile(&sea_monster, &new_tile.pixels);
    println!("Checking tb lr flipped");
    let new_tile = big_tile.get_flipped_TB().get_flipped_LR();
    find_all_shapes_in_tile(&sea_monster, &new_tile.pixels);
    println!("Checking transposed");
    let new_tile = big_tile.get_transposed();
    find_all_shapes_in_tile(&sea_monster, &new_tile.pixels);
    println!("Checking t tb flipped");
    let new_tile = big_tile.get_transposed().get_flipped_TB();
    find_all_shapes_in_tile(&sea_monster, &new_tile.pixels);
    println!("Checking t lr flipped");
    let new_tile = big_tile.get_transposed().get_flipped_LR();
    find_all_shapes_in_tile(&sea_monster, &new_tile.pixels);
    println!("Checking t tb lr flipped");
    let new_tile = big_tile.get_transposed().get_flipped_TB().get_flipped_LR();
    find_all_shapes_in_tile(&sea_monster, &new_tile.pixels);

    println!("final ans {}", 2638-15*41);
}

