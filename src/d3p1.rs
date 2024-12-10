use std::fs;

enum States {
    NONE,
    M,
    U,
    L,
    IntOne,
    IntTwo,
    D,
    O,
    N,
    APO,
    T,
    BL,
}

enum DODONT {
    DO,
    DONT,
}

fn main() {
    let input_string = fs::read_to_string("input_data/day3").unwrap();
    let mut int_one = Vec::new();
    let mut int_two = Vec::new();
    let mut enabled = true;
    let mut total = 0;
    let mut possible_current = DODONT::DO;

    let mut curr_state = States::NONE;

    for c in input_string.chars() {
        match curr_state {
            States::NONE => {
                if c == 'm' && enabled {
                    curr_state = States::M;
                } else if c == 'd' {
                    curr_state = States::D;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::M => {
                if c == 'u' {
                    curr_state = States::U;
                } else if c == 'd' {
                    curr_state = States::D;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::U => {
                if c == 'l' {
                    curr_state = States::L;
                } else if c == 'd' {
                    curr_state = States::D;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::L => {
                if c == '(' {
                    curr_state = States::IntOne;
                } else if c == 'd' {
                    curr_state = States::D;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::IntOne => {
                if c.is_digit(10) {
                    curr_state = States::IntOne;
                    int_one.push(c.to_digit(10).unwrap().into());
                } else if c == ',' {
                    curr_state = States::IntTwo;
                } else if c == 'd' {
                    curr_state = States::D;
                    int_one = Vec::new();
                } else {
                    curr_state = States::NONE;
                    int_one = Vec::new();
                }
            }
            States::IntTwo => {
                if c.is_digit(10) {
                    curr_state = States::IntTwo;
                    int_two.push(c.to_digit(10).unwrap().into());
                } else if c == ')' {
                    curr_state = States::NONE;
                    total += complete_mult(&int_one, &int_two);
                    int_one = Vec::new();
                    int_two = Vec::new();
                } else if c == 'd' {
                    curr_state = States::D;
                    int_one = Vec::new();
                    int_two = Vec::new();
                } else {
                    curr_state = States::NONE;
                    int_two = Vec::new();
                    int_one = Vec::new();
                }
            }
            States::D => {
                if c == 'o' {
                    curr_state = States::O;
                } else if c == 'm' && enabled {
                    curr_state = States::M;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::O => {
                if c == 'n' {
                    curr_state = States::N;
                } else if c == '(' {
                    curr_state = States::BL;
                    possible_current = DODONT::DO;
                } else if c == 'm' && enabled {
                    curr_state = States::M;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::N => {
                if c == '\'' {
                    curr_state = States::APO;
                } else if c == 'm' && enabled {
                    curr_state = States::M;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::APO => {
                if c == 't' {
                    curr_state = States::T;
                } else if c == 'm' && enabled {
                    curr_state = States::M;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::T => {
                if c == '(' {
                    curr_state = States::BL;
                    possible_current = DODONT::DONT;
                } else if c == 'm' && enabled {
                    curr_state = States::M;
                } else {
                    curr_state = States::NONE;
                }
            }
            States::BL => {
                if c == ')' {
                    match possible_current {
                        DODONT::DO => enabled = true,
                        DODONT::DONT => enabled = false,
                    }
                } else if c == 'm' && enabled {
                    curr_state = States::M;
                } else {
                    curr_state = States::NONE;
                }
            }
        }
    }
    println!("{}", total);
}

fn complete_mult(vec_one: &Vec<u64>, vec_two: &Vec<u64>) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let len_one: u32 = vec_one.len().try_into().unwrap();
    let len_two: u32 = vec_two.len().try_into().unwrap();

    for (i, e) in vec_one.iter().enumerate() {
        a += e * 10_u64.pow(len_one - 1 - (i as u32));
    }
    for (i, e) in vec_two.iter().enumerate() {
        b += e * 10_u64.pow(len_two - 1 - (i as u32));
    }

    return a * b;
}
