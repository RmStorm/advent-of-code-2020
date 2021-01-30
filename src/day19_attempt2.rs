use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::cell::Cell;

#[derive(PartialEq, Clone, Copy, Debug)]
enum RuleElement {
    Branch(usize),
    Terminator(char),
}

#[derive(PartialEq, Debug, Clone)]
struct State {
    active: Cell<bool>,
}

impl State {
    fn new() -> State {
        State {
            active: Cell::new(false),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Gate<'a> {
    warden: char,
    in_state: &'a State,
    out_state: &'a State,
}

pub fn day19() {
	let file = File::open("day19_input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let line_regex = Regex::new(r"^(\d+): (.+)$").unwrap();
    let branch_regex = Regex::new(r"(\d+)\s*(\|*)\s*").unwrap();
    let mut rules: HashMap<usize, Vec<Vec<RuleElement>>> = HashMap::new();
    let mut messages: Vec<Vec<char>> = Vec::new();
    let mut reading_rules = true;

    // let state1 = State{active:Cell::new(false), transitions:Vec::new()};
    // let state2 = State{active:Cell::new(false), transitions:vec![&state1]};
    // println!("{:?}", state2);
    // for yada in state2.transitions.iter() {
    //     yada.active.set(true);
    // }
    // println!("{:?}", state2);

    for line in reader.lines() {
        let text = &line.unwrap();
        if text == "" {reading_rules=false; continue}
        if reading_rules {
            let captures = line_regex.captures(&text).unwrap();
            // println!("captures: {:?}", captures);
            let mut rule = vec![];
            let mut branches = vec![];
            for caps in branch_regex.captures_iter(&captures[2]) {
                branches.push(RuleElement::Branch(caps[1].parse::<usize>().unwrap()));
                if &caps[2] == "|" {
                    rule.push(branches);
                    branches = vec![];
                }
            }
            if branches.len() == 0 {
                // println!("This is a terminator since the regex failed... {:?}", &captures[2].chars().nth(1));
                branches.push(RuleElement::Terminator(captures[2].chars().nth(1).unwrap()))
            }
            rule.push(branches);
            rules.insert(captures[1].parse::<usize>().unwrap(), rule);
        } else {
            // println!("{:?}", text);
            messages.push(text.chars().collect());
        }
        
    }

    let num_rules = *rules.keys().max().unwrap()+1;
    for i in 0..num_rules {
        if rules.contains_key(&i) {
            println!("{:?}: {:?}", i, rules[&i])
        }
    }

    let mut gates = Vec::new();
    let states = vec![State::new(); 1000000];
    let start_state = &states[0];
    start_state.active.set(true);
    let final_state = construct_machine(
        &mut gates,
        &rules,
        0,
        &states,
        &mut 1,
        start_state
    );

    // for gate in gates.iter() {
    //     println!("{:?}", gate);
    // }
    // println!("{:?}\n", final_state);

    let mut ans = 0;
    for mes in messages {
        println!("STARTING SIM WITH {:?}", mes);
        let result = run_state_machine(mes, &gates, start_state, final_state);
        println!("{:?}", result);
        if result {ans += 1;}
    }
    println!("{:?}", ans);
    // let message = vec!['a', 'a', 'a', 'a', 'b', 'b', 'b'];
    // println!("STARTING SIM WITH {:?}", message);
    // let result = run_state_machine(message, &gates, start_state, final_state);
    // println!("{:?}", result);
}

fn construct_machine<'a>(
    gates:&mut Vec<Gate<'a>>,
    rules: &HashMap<usize, Vec<Vec<RuleElement>>>,
    current_rule: usize,
    states:&'a Vec<State>,
    next_unused_state: &mut usize,
    rule_in_state: &'a State
) -> &'a State {
    // println!("{:?}", rules[&current_rule]);
    let rule_out_state = &(states[*next_unused_state]);
    *next_unused_state += 1;
    for rule_branch in rules[&current_rule].iter() {
        let branch_in_state= &(states[*next_unused_state]);
        *next_unused_state += 1;
        let branch_out_state= &(states[*next_unused_state]);
        *next_unused_state += 1;

        gates.push(Gate{warden:'\0', in_state: rule_in_state, out_state: branch_in_state});
        gates.push(Gate{warden:'\0', in_state: branch_out_state, out_state: rule_out_state});

        let mut in_state;
        let mut out_state = None;
        let branch_len = rule_branch.len();
        for (i, rule) in rule_branch.iter().enumerate() {
            
            in_state = &(states[*next_unused_state]);
            *next_unused_state += 1;
            if i == 0 {
                gates.push(Gate{warden:'\0', in_state: branch_in_state, out_state: in_state})
            } else {
                gates.push(Gate{warden:'\0', in_state: out_state.unwrap(), out_state: in_state});
            }


            match rule {
                RuleElement::Branch(next_rule) => {
                    if current_rule == *next_rule {
                        out_state = Some(&(states[*next_unused_state]));
                        *next_unused_state += 1;
                        gates.push(Gate{warden:'\0', in_state, out_state: out_state.unwrap()});
                        gates.push(Gate{warden:'\0', in_state: out_state.unwrap(), out_state: in_state});

                        println!("Do not rcurse!!");
                        println!("found rule: {:?}", next_rule);
                    } else {
                        out_state = Some(construct_machine(
                            gates,
                            &rules,
                            *next_rule,
                            &states,
                             next_unused_state,
                            in_state
                        ));
                    }
                },
                RuleElement::Terminator(warden) => {
                    out_state = Some(&(states[*next_unused_state]));
                    *next_unused_state += 1;
                    gates.push(Gate{warden:*warden, in_state, out_state: out_state.unwrap()});
                },
            }
            if i == branch_len-1 {
                gates.push(Gate{warden:'\0', in_state: out_state.unwrap(), out_state: branch_out_state})
            }
        }
    }
    rule_out_state
}

fn propagate_empty_strings(machine: &Vec<Gate>) {
    let mut found_all = false;
    while !found_all {
        found_all = true;
        for gate in machine.iter() {
            if gate.in_state.active.get() && gate.warden == '\0' && !gate.out_state.active.get() {
                found_all = false;
                gate.out_state.active.set(true)
            }
        }    
    }
}

fn run_state_machine(message:Vec<char>, machine: &Vec<Gate>, start_state: &State, end_state: &State) -> bool {
    propagate_empty_strings(machine);
    for el in message.iter() {
        // for gate in machine.iter() { println!("{:?}", gate) }
        // println!("{:?}\n", el);

        if end_state.active.get() {end_state.active.set(false)};
        let mut old_active_states = Vec::new();
        let mut new_active_states = Vec::new();
        for gate in machine.iter() {
            if gate.in_state.active.get() {
                old_active_states.push(gate.in_state);
                if gate.warden == *el {
                    new_active_states.push(gate.out_state);
                }
            }
        }
        
        for state in old_active_states.iter() { state.active.set(false) }
        for state in new_active_states.iter() { state.active.set(true) }
        
        propagate_empty_strings(machine);

    }
    
    // for gate in machine.iter() { println!("{:?}", gate) }
    // println!("");
    // println!("{:?}", end_state);
    let res = end_state.active.get();
    // Reset state machine
    for gate in machine.iter() {
        gate.in_state.active.set(false);
        gate.out_state.active.set(false);
    }
    start_state.active.set(true);

    res
}