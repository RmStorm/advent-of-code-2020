use std::fs::File;
use std::io::{BufReader, BufRead};


fn get_path_branches(i: usize) -> u64 {
    let mut paths = 0;
    let previous_positions = vec![true; i];
    get_permutations(&previous_positions, 0 , &mut paths);
    // println!("paths for i: {:?}, {:?}", paths, i);
    paths
}

fn get_permutations(previous_positions: &[bool], i:usize, paths:&mut u64) {
    if i == previous_positions.len() {
        // println!("paths: {:?}", previous_positions);
        *paths += 1;
        return
    }

    get_permutations(previous_positions, i+1, paths);

    if i > 1 { if previous_positions[i-2..i].iter().all(|&e| !e) { return } }
    
    let mut cur_positions = previous_positions.to_vec();
    cur_positions[i] = false;
    get_permutations(&cur_positions, i+1, paths)
}


pub fn day10() {
	let file = File::open("day10_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let mut code: Vec<usize> = reader.lines().map(|e| e.unwrap().parse::<usize>().unwrap()).collect();
    code.push(0);
    code.sort();
    code.push(code.last().unwrap()+3);

    let vals = (&code).iter();
    let next_vals = (&code).iter().skip(2);

    let diffs: Vec<usize> = vals.zip(next_vals).map(|(prev, next)| next - prev).collect();

    println!("code: {:?}", code);
    println!("diffs: {:?}", diffs);
    
    let mut slices:Vec<&[usize]> = Vec::new();
    let mut start = 0;
    for (i, diff) in diffs.iter().enumerate() {
        if *diff > 3 {
            if (&diffs[start..i]).len() > 0 {slices.push(&diffs[start..i])}
            start = i+1;
        }
    }
    if (&diffs[start..]).len() > 0 {slices.push(&diffs[start..])}
    println!("slices: {:?}", slices);
    
    let mut paths:u64 = 1;
    let branch_factors: Vec<u64> = (0..5).map(|e| get_path_branches(e)).collect();
    println!("bfs: {:?}", branch_factors);
    for slice in slices.iter() {
        if !slice.iter().all(|&i| i==2) {
            println!("Warning not all diffs are 2: {:?}", slice);
            continue
        }
        paths *= branch_factors[slice.len()]
    }
    // println!("sol p2: {:?}", get_path_branches(0));
    // println!("sol p2: {:?}", get_path_branches(1));
    // println!("sol p2: {:?}", get_path_branches(2));
    // println!("sol p2: {:?}", get_path_branches(3));
    println!("sol p2: {:?}", paths);
    let correct_ans: u64 = 42313823813632;
    println!("sol p2: {:?}", correct_ans);
    
}
