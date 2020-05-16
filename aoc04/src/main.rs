use std::env;

fn check_range(pass: i32, range: &(i32, i32)) -> bool {
    pass >= range.0 && pass <= range.1
}

pub fn check_adjacent(mut pass: i32) -> bool {
    let mut last = pass % 10;
    pass = pass / 10;
    while pass > 0 {
        if pass % 10 == last {
            return true;
        }
        last = pass % 10;
        pass = pass / 10;
    };
    false
}

pub fn check_adjacent_length(mut pass: i32, length: i32) -> bool {
    let mut last = pass % 10;
    pass = pass / 10;
    let mut curr_length = 1;
    while pass > 0 {
        if pass % 10 == last {
            curr_length += 1;
        } else {
            if curr_length == length {
                return true;
            }
            curr_length = 1;
        }
        last = pass % 10;
        pass = pass / 10;
    };
    curr_length == length
}

pub fn check_increase_from_left(mut pass: i32) -> bool {
    let mut last = pass % 10;
    pass = pass / 10;
    while pass > 0 {
        if pass % 10 > last {
            return false;
        }
        last = pass % 10;
        pass = pass / 10;
    };
    true
}

fn check_password(pass: i32, range: &(i32, i32)) -> bool {
    check_range(pass, range) && check_adjacent(pass) && check_increase_from_left(pass)
}

fn check_password2(pass: i32, range: &(i32, i32)) -> bool {
    check_range(pass, range) && check_adjacent_length(pass, 2) && check_increase_from_left(pass)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let range: (i32, i32) = match args.len() {
        3 => (args[1].parse::<i32>().unwrap(), args[2].parse::<i32>().unwrap()),
        _ => panic!("usage: aoc04 lower upper")
    };
    println!("checking passwords in range [{} , {})", range.0, range.1 + 1);
    let count = (range.0 .. range.1+1).fold(0i32, |count, pass| if check_password(pass, &range) { count+1 } else { count });
    println!("found {} viable (part1)", count);
    let count2 = (range.0 .. range.1+1).fold(0i32, |count, pass| if check_password2(pass, &range) { count+1 } else { count });
    println!("found {} viable (part2)", count2);
}

#[test]
fn range() {
    assert!(check_range(5, &(1, 9)));
    assert!(!check_range(12, &(1, 9)));
}

#[test]
fn adjacent() {
    assert!(check_adjacent(12344));
    assert!(check_adjacent(11234));
    assert!(!check_adjacent(1234));
}

#[test]
fn adjacent_length() {
    assert!(check_adjacent_length(112233, 2));
    assert!(!check_adjacent_length(123444, 2));
    assert!(check_adjacent_length(111122, 2));
}

#[test]
fn decrease_from_left() {
    assert!(check_increase_from_left(1234));
    assert!(check_increase_from_left(11234));
    assert!(!check_increase_from_left(11211));
}