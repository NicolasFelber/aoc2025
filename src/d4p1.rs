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
    let mut count = 0;
    if i >= 3 {
        if vector[i - 1][j] == 'M' && vector[i - 2][j] == 'A' && vector[i - 3][j] == 'S' {
            count += 1;
            println!["{} {} up", i, j];
        }
        if j >= 3
            && vector[i - 1][j - 1] == 'M'
            && vector[i - 2][j - 2] == 'A'
            && vector[i - 3][j - 3] == 'S'
        {
            count += 1;
            println!["{} {} up left", i, j];
        }
        if j < vector[0].len() - 3
            && vector[i - 1][j + 1] == 'M'
            && vector[i - 2][j + 2] == 'A'
            && vector[i - 3][j + 3] == 'S'
        {
            count += 1;
            println!["{} {} up right", i, j];
        }
    }
    if i < vector.len() - 3 {
        if vector[i + 1][j] == 'M' && vector[i + 2][j] == 'A' && vector[i + 3][j] == 'S' {
            count += 1;
            println!["{} {} down", i, j];
        }
        if j >= 3
            && vector[i + 1][j - 1] == 'M'
            && vector[i + 2][j - 2] == 'A'
            && vector[i + 3][j - 3] == 'S'
        {
            count += 1;
            println!["{} {} down left", i, j];
        }
        if j < vector[i].len() - 3
            && vector[i + 1][j + 1] == 'M'
            && vector[i + 2][j + 2] == 'A'
            && vector[i + 3][j + 3] == 'S'
        {
            count += 1;
            println!["{} {} down right", i, j];
        }
    }
    if j >= 3 && vector[i][j - 1] == 'M' && vector[i][j - 2] == 'A' && vector[i][j - 3] == 'S' {
        count += 1;
        println!["{} {} left", i, j];
    }
    if j < vector[i].len() - 3
        && vector[i][j + 1] == 'M'
        && vector[i][j + 2] == 'A'
        && vector[i][j + 3] == 'S'
    {
        count += 1;
        println!["{} {} right", i, j];
    }
    return count;
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
