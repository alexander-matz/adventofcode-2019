use std::io::{self, BufRead};

fn calc_fuel(mass: i32) -> i32 {
    return mass / 3 - 2;
}

fn calc_fuel_inc(mut mass: i32) -> i32 {
    let mut fuel = 0;
    loop {
        let delta = mass / 3 - 2;
        if delta < 0 { break; }
        fuel = fuel + delta;
        mass = delta;
    }
    return fuel;
}

fn main() {
    let stdin = io::stdin();
    let mut fuel: i32 = 0;
    let mut fuel2: i32 = 0;
    for line in stdin.lock().lines() {
        let module_mass = line.unwrap().parse::<i32>().unwrap();
        fuel += calc_fuel(module_mass);
        fuel2 += calc_fuel_inc(module_mass);
    }
    println!("fuel required: {}", fuel);
    println!("fuel2 required: {}", fuel2);
}
