use std::fs;
fn main() {
    let a = fs::read_to_string("input_data/day4").unwrap();
    let res = solution(a);
    print!("{}", res);
}

fn solution(input: String) -> u64 {
    let input_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut res = 0;

    for i in 0..input_vec.len() {
        for j in 0..input_vec[0].len() {
            if input_vec[i][j] == 'X' {
                res += check_x(i, j, &input_vec);
            }
        }
    }

    return res;
}

fn check_x(i: usize, j: usize, vector: &Vec<Vec<char>>) -> u64 {
    if i > 3 {
        if 
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        assert_eq!(solution(a), 18);
    }
}
