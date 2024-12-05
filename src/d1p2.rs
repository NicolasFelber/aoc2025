use std::collections::HashMap;
use std::fs;

fn main() {
    let mut a = HashMap::new();
    let mut b = HashMap::new();
    for (i, line) in fs::read_to_string("input_data/day1")
        .unwrap()
        .lines()
        .enumerate()
    {
        //println! {"{}", line};
        let mut split_line = line.split("   ");
        let item1: i64 = split_line.next().unwrap_or("0").parse().unwrap();
        let item2: i64 = split_line.next().unwrap_or("0").parse().unwrap();
        a.entry(item1).and_modify(|e| *e += 1).or_insert(1);
        b.entry(item2).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut res = 0;

    for (key, val) in a.iter() {
        res += match b.get(key) {
            Some(i) => key * val * i,
            None => 0,
        };
    }

    println!("{}", res);
}
