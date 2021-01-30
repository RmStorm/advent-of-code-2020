use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


#[derive(PartialEq, Clone, Copy, Debug)]
enum TreeNode {
    Branch(usize),
    Terminator(char),
}

pub fn day19() {
	let file = File::open("day19_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let line_regex = Regex::new(r"^(\d+): (.+)$").unwrap();
    let branch_regex = Regex::new(r"(\d+)\s*(\|*)\s*").unwrap();
    let mut rules: HashMap<usize, Vec<Vec<TreeNode>>> = HashMap::new();
    let mut messages: Vec<Vec<char>> = Vec::new();
    let mut reading_rules = true;

    for line in reader.lines() {
        let text = &line.unwrap();
        if text == "" {reading_rules=false; continue}
        if reading_rules {
            let captures = line_regex.captures(&text).unwrap();
            // println!("captures: {:?}", captures);
            let mut rule = vec![];
            let mut subrule = vec![];
            for caps in branch_regex.captures_iter(&captures[2]) {
                subrule.push(TreeNode::Branch(caps[1].parse::<usize>().unwrap()));
                if &caps[2] == "|" {
                    rule.push(subrule);
                    subrule = vec![];
                }
            }
            if subrule.len() == 0 {
                // println!("This is a terminator since the regex failed... {:?}", &captures[2].chars().nth(1));
                subrule.push(TreeNode::Terminator(captures[2].chars().nth(1).unwrap()))
            }
            rule.push(subrule);
            rules.insert(captures[1].parse::<usize>().unwrap(), rule);
        } else {
            // println!("{:?}", text);
            messages.push(text.chars().collect());
        }
        
    }
    // println!("{:?}", rules);
    let num_rules = *rules.keys().max().unwrap()+1;
    let mut rules_vector: Vec<&Vec<Vec<TreeNode>>> = Vec::with_capacity(num_rules);
    for i in 0..num_rules {rules_vector.push(&rules[&i])}
    for yada in rules_vector.iter() {
        println!("{:?}", yada);
    }

    let mut matched_messages = 0;
    for mes in messages.iter() {
        println!("{:?}", mes);
        let mut check_index = 0;
        let yada = check_valid(&mes, &rules, &mut check_index, &0, 0);
        if check_index == mes.len()-1 {
            println!("{:?}", yada);
            matched_messages += 1;
        } else {
            println!("FAAALSE");
        }
    }
    println!("{:?}", matched_messages)
}

fn check_valid(message: &Vec<char>, rules: &HashMap<usize, Vec<Vec<TreeNode>>>, check_index: &mut usize, rule_number: &usize, depth:i32) -> bool {
    // println!("{:?}  check: {:?} rule: {:?} : {:?}",depth, check_index, rule_number, rules[rule_number]);
    for rule_branch in rules[rule_number].iter() {
        let mut branch_res = true;  // instantiate as true, flip to false on broken rule
        let starting_index = *check_index;
        for (i, rule) in rule_branch.iter().enumerate() {
            if i != 0 {*check_index += 1}
            if *check_index == message.len() {return false}
            // println!("{:?} now checking rule, {:?}, enum_i {:?}", depth, rule, i);
            let res = match rule {
                TreeNode::Branch(next_rule) => check_valid(message, rules, check_index, next_rule, depth+1),
                TreeNode::Terminator(a_char) => {
                    // println!("{:?} checking {:?}={:?}", depth, a_char, message[*check_index]);
                    // println!("{:?} hitted {:?}", depth, *a_char == message[*check_index]);
                    return *a_char == message[*check_index]
                }
            };
            branch_res &= res;
            // println!("{:?} res: {:?}, branch_res: {:?}", depth, res, branch_res);
            if !res { *check_index = starting_index; break }
        }
        if branch_res {
            return branch_res;
        }
    }
    false
}