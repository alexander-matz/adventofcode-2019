use std::io::{self, Read};

fn run_intcode(mem: &mut Vec<i32>, mut input: Vec<i32>) -> Vec<i32> {
    input.reverse();
    let mut output: Vec<i32> = Vec::new();
    fn opmode_to_string(mode: i32) -> &'static str {
        match mode {
            0 => "LD",
            1 => "IM",
            _ => "??"
        }
    }
    fn read_op(mem: &Vec<i32>, op: i32, mode: i32) -> i32 {
        match mode {
            0 => mem[op as usize],
            1 => op,
            _ => panic!("invalid operand mode")
        }
    };
    let mut ip = 0;
    loop {
        let opcode = mem[ip] % 100;
        let opmode = mem[ip] / 100;
        match opcode {
            1 => {
                let opmode1 = opmode % 10;
                let opmode2 = (opmode / 10) % 10;
                let opmode3 = 0;
                let op1 = read_op(&mem, mem[ip+1], opmode1);
                let op2 = read_op(&mem, mem[ip+2], opmode2);
                let op3 = read_op(&mem, mem[ip+3], opmode3);
                println!("{} {} {} {}", mem[ip], mem[ip+1], mem[ip+2], mem[ip+3]);
                println!("ADD {}{} {}{} -> {}{}", opmode_to_string(opmode1), op1, opmode_to_string(opmode2), op2, opmode_to_string(opmode3), op3);
                mem[op3 as usize] = op1 + op2;
                ip += 4;
            },
            2 => {
                let opmode1 = opmode % 10;
                let opmode2 = (opmode / 10) % 10;
                let opmode3 = 0;
                let op1 = read_op(&mem, mem[ip+1], opmode1);
                let op2 = read_op(&mem, mem[ip+2], opmode2);
                let op3 = read_op(&mem, mem[ip+3], opmode3);
                println!("{} {} {} {}", mem[ip], mem[ip+1], mem[ip+2], mem[ip+3]);
                println!("MUL {} {} {} {} -> {} {}", opmode_to_string(opmode1), op1, opmode_to_string(opmode2), op2, opmode_to_string(opmode3), op3);
                mem[op3 as usize] = op1 * op2;
                ip += 4;
            },
            3 => {
                let opmode1 = 0;
                let op1 = read_op(mem, mem[ip+1], opmode1);
                println!("{} {}", mem[ip], mem[ip+1]);
                println!("IN  -> {} {}", opmode_to_string(opmode1), op1);
                match input.pop() {
                    Some(val) => mem[op1 as usize] = val,
                    None => panic!("ran out of input data")
                }
                ip += 2;
            },
            4 => {
                let opmode1 = opmode %10;
                let op1 = read_op(mem, mem[ip+1], opmode1);
                println!("{} {}", mem[ip], mem[ip+1]);
                println!("OUT {}{}", opmode_to_string(opmode1), op1);
                output.push(op1);
                ip += 2;
            }
            99 => {
                println!("halt");
                break;
            },
            other => {
                panic!("invalid operand {}", other)
            }
        }
    }
    output
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer).unwrap();
    let mut program: Vec<i32> = buffer.split(",").into_iter().map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let output = run_intcode(&mut program, vec![1]);
    println!("output: {:?}", output);
}

#[test]
fn test_immediate() {
    let mut mem: Vec<i32> = vec![1101, 2, 3, 5, 99, 0];
    run_intcode(&mut mem, vec![]);
    assert_eq!(mem[0], 5);

    mem = vec![1102, 2, 3, 5, 99, 0];
    run_intcode(&mut mem, vec![]);
    assert_eq!(mem[0], 6);
}

#[test]
fn test_input() {
    let mut mem: Vec<i32> = vec![3, 3, 99, 0];
    run_intcode(&mut mem, vec![42]);
    assert_eq!(mem[0], 42);
}

#[test]
fn test_output() {
    let mut mem: Vec<i32> = vec![4, 3, 99, 42];
    let output = run_intcode(&mut mem, vec![]);
    assert_eq!(output, vec![42]);
}