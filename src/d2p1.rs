use std::fs;

fn main() {
    let mut counter = 0;

    for line in fs::read_to_string("input_data/day2").unwrap().lines() {
        if check_line(line.to_string()) {
            counter += 1;
        }
    }

    println!("{}", counter);
}

fn check_line(line: String) -> bool {
    let cont: Vec<Option<u64>> = line
        .split(" ")
        .map(|i| match i.parse::<u64>() {
            Ok(n) => Some(n),
            Err(e) => {
                println!("Couldn't parse the input: {}", e);
                None
            }
        })
        .collect();

    for i in 0..cont.len() {
        let mut slice = cont.to_owned();
        slice.remove(i);
        if check_line_removed(slice) {
            return true;
        }
    }

    return false;
}

fn check_line_removed(line: Vec<Option<u64>>) -> bool {
    let ascending = line[1] > line[0];
    for (i, val) in line.iter().enumerate().skip(1) {
        if ascending && val.unwrap() < line[i - 1].unwrap()
            || !ascending && val.unwrap() > line[i - 1].unwrap()
            || val.unwrap().abs_diff(line[i - 1].unwrap()) > 3
            || val.unwrap().abs_diff(line[i - 1].unwrap()) == 0
        {
            return false;
        }
    }
    return true;
}
