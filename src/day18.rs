use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Clone, Copy, Debug)]
enum TermOp {
    Op(char),
    Term(i64),
}

fn do_op(term1:i64, term2:i64, op: char) -> i64 {
    if op == '*' {
        term1 * term2
    } else {
        term1 + term2
    }
}

fn find_closing_paren(sub_question: &[TermOp]) -> usize {
    let mut parenthesis_depth = 0;
    for (i, termop) in sub_question[0..].iter().enumerate() {
        if i == 0 {continue}
        match termop {
            TermOp::Op(op) => {
                if *op == '(' {parenthesis_depth += 1}
                else if *op == ')' { if parenthesis_depth==0 { return i } else { parenthesis_depth -= 1; }}
                else {continue}
            },
            TermOp::Term(_) => continue
        }
    }
    panic!("We should have recursed and not reached here...")
}

fn solve(sub_question: &[TermOp]) -> i64 {
    // println!("sub_question: {:?}", sub_question);

    match sub_question[0] {
        TermOp::Term(subterm0) => {
            if sub_question.len() == 1 {
                return subterm0
            }
            let current_op: char = match sub_question[1] {
                TermOp::Term(_) => panic!("There should be an operator here.."),
                TermOp::Op(op) => op
            };
            if current_op == '*' {
                return subterm0 * solve(&sub_question[2..])
            }
            return match sub_question[2] {
                TermOp::Term(subterm2) => {
                    let mut first_new_term = vec![TermOp::Term( do_op(subterm0, subterm2, current_op))];
                    first_new_term.extend(&sub_question[3..]);
                    solve(&first_new_term)
                },
                TermOp::Op(_) => {
                    let i = find_closing_paren(&sub_question[2..]);
                    let mut subterm = vec![TermOp::Term(do_op(subterm0, solve(&sub_question[2..i+3]), current_op))];
                    subterm.extend(&sub_question[i+3..]);
                    return solve(&subterm)
                }
            }
        },
        TermOp::Op(op) => {
            assert!(op == '(', "Expected opening parenthesis.. Something went wrong.");
            let i = find_closing_paren(sub_question);
            let mut subterm = vec![TermOp::Term(solve(&sub_question[1..i]))];
            subterm.extend(&sub_question[i+1..]);
            return solve(&subterm)
        },
    }
}

pub fn day18() {
	let file = File::open("day18_input.txt").expect("file not found!");
    let reader = BufReader::new(file);

    let mut ans = 0;
    for line in reader.lines() {
        let question:Vec<TermOp> = line.unwrap().chars().map(|symbol| 
            
            if symbol == ' ' {Err("skipping space")} else {Ok(
                match symbol.to_digit(10) {
                    Some(number) => TermOp::Term(number as i64),
                    None => TermOp::Op(symbol),
                }
            )}
                
        ).filter_map(Result::ok).collect();
        // println!("question: {:?}", question);
        ans += solve(&question);
        println!("ans: {:?}", ans);

    }
}
