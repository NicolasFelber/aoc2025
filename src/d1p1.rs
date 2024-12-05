use std::fs;
fn main() {
    let mut a = vec![0; 1000];
    let mut b = vec![0; 1000];
    for (i, line) in fs::read_to_string("input_data/day1")
        .unwrap()
        .lines()
        .enumerate()
    {
        //println! {"{}", line};
        let mut split_line = line.split("   ");
        a[i] = split_line.next().unwrap_or("0").parse().unwrap();
        b[i] = split_line.next().unwrap_or("0").parse().unwrap();
    }

    a.sort();
    b.sort();

    let res: i64 = a
        .iter()
        .zip(b.iter())
        .map(|i| ((i.0 - i.1) as i64).abs())
        .sum();
    println!("{}", res);
}
