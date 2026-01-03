use std::vec;

pub fn read_input() -> Vec<Vec<i8>> {
    let mut inp: Vec<Vec<i8>> = vec![];

    let vec: Vec<String> = std::fs::read_to_string("Input/DaySeven.txt")
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();

    let col = vec[0].len();
    let row = vec.len();

    for _ in 0..row {
        let mut k: Vec<i8> = vec![];
        for _ in 0..col {
            k.push(-1);
        }
        inp.push(k);
    }

    for i in 0..row {
        for j in 0..col {
            let ch = vec[i].as_bytes()[j];
            match ch {
                b'S' => inp[i][j] = 2,
                b'^' => inp[i][j] = 1,
                b'.' => inp[i][j] = 0,
                _ => (),
            }
        }
    }

    inp
}

fn is_valid_pos(array: &Vec<Vec<i8>>, posx: usize, posy: usize) -> bool {
    if posx < array.len() && posx >= 0 && posy < array[0].len() && posy >= 0 {
        return true;
    }
    false
}

pub fn part_1() {
    let mut input = read_input();

    let col = input[0].len();
    let row = input.len();
    input[1][col / 2] = 9;

    let mut ans = 0;
    for i in 2..row {
        for j in 0..col {
            if input[i][j] == 1 {
                if is_valid_pos(&input, i - 1, j) && input[i - 1][j] == 9 {
                    ans += 1;
                    if is_valid_pos(&input, i, j - 1) && input[i][j - 1] == 0 {
                        input[i][j - 1] = 9;
                    }
                    if is_valid_pos(&input, i, j + 1) && input[i][j + 1] == 0 {
                        input[i][j + 1] = 9;
                    }
                }
            } else if input[i][j] == 0 {
                if is_valid_pos(&input, i - 1, j) && input[i - 1][j] == 9 {
                    input[i][j] = 9;
                }
            }
        }
    }
    println!("{ans}");
}

fn search(array: &Vec<Vec<i8>>, row: usize, col: usize) -> u128 {
    if col >= array[0].len() || col < 0 {
        return 0;
    }
    if row >= array.len() {
        return 1;
    }

    let mut L: u128 = 0;
    let mut M: u128 = 0;
    let mut R: u128 = 0;
    if col > 0 && array[row][col] == 9 {
        L = search(array, row + 1, col - 1);
    }
    if array[row][col] == 9 {
        M = search(array, row + 1, col);
    }
    if array[row][col] == 9 {
        R = search(array, row + 1, col + 1);
    }
    L + M + R
}

pub fn part_2() {
    let mut input = read_input();

    let col = input[0].len();
    let row = input.len();

    input[1][col / 2] = 9;
    for i in 2..row {
        for j in 0..col {
            if input[i][j] == 1 {
                if is_valid_pos(&input, i - 1, j) && input[i - 1][j] == 9 {
                    if is_valid_pos(&input, i, j - 1) && input[i][j - 1] == 0 {
                        input[i][j - 1] = 9;
                    }
                    if is_valid_pos(&input, i, j + 1) && input[i][j + 1] == 0 {
                        input[i][j + 1] = 9;
                    }
                }
            } else if input[i][j] == 0 {
                if is_valid_pos(&input, i - 1, j) && input[i - 1][j] == 9 {
                    input[i][j] = 9;
                }
            }
        }
    }
    let ans = search(&input, 1, col / 2);

    println!("{ans}");
}
