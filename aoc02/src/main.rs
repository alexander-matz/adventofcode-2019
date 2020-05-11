use std::io::{self, Read};
use std::env;

fn run_intcode(mut program: Vec<i32>) -> i32 {
    let mut ip = 0;
    loop {
        match program[ip] {
            1 => {
                let op1 = program[ip+1] as usize;
                let op2 = program[ip+2] as usize;
                let dst = program[ip+3] as usize;
                println!("add {}({}) {}({}) -> {}", op1, program[op1], op2, program[op2], dst);
                program[dst] = program[op1] + program[op2];
                ip += 4;
            },
            2 => {
                let op1 = program[ip+1] as usize;
                let op2 = program[ip+2] as usize;
                let dst = program[ip+3] as usize;
                println!("mul {}({}) {}({}) -> {}", op1, program[op1], op2, program[op2], dst);
                program[dst] = program[op1] * program[op2];
                ip += 4;
            },
            99 => {
                println!("halt");
                break;
            },
            x => {
                panic!("invalid operand {}", x)
            }
        }
    }
    program[0]
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer).unwrap();
    let mut program: Vec<i32> = buffer.split(",").into_iter().map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        program[1] = 12;
        program[2] = 2;
        let output = run_intcode(program);
        println!("value at program[0]: {}", output);
    } else if args.len() == 2 {
        let goal: i32 = args[1].parse().unwrap();
        let mut found = false;
        for j in 0..100 {
            for k in 0..100 {
                let mut copy = program.to_vec();
                copy[1] = j;
                copy[2] = k;
                let output = run_intcode(copy);
                if output == goal {
                    println!("{} reached with {} and {}, anser: {}", goal, j, k, 100 * j + k);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            println!("solution not found :(")
        }
    } else {
        panic!("usage: aoc2 [code]");
    }
}
