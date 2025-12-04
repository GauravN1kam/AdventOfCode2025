fn is_valid_position(x: i32, y: i32) -> bool {
    if (x < 0 || x >= 139 || y < 0 || y >= 139) {
        return false;
    }
    true
}

pub fn part1() {
    let mut input = std::fs::read_to_string("Input/DayFour.txt").unwrap();
    let dirX: Vec<i32> = vec![-1, 0, 1, 1, 1, 0, -1, -1];
    let dirY: Vec<i32> = vec![-1, -1, -1, 0, 1, 1, 1, 0];

    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut base_vec: Vec<char> = vec![];
        for word in line.chars() {
            base_vec.push(word);
        }
        grid.push(base_vec);
    }
    let mut ans = 0;
    for i in 0..139 {
        for j in 0..139 {
            if grid[i][j] == '@' {
                let mut count = 0;
                for k in 0..8 {
                    let x = i as i32 + dirX[k];
                    let y = j as i32 + dirY[k];

                    if is_valid_position(x as i32, y as i32) {
                        if grid[x as usize][y as usize] == '@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

pub fn part2() {
    let mut input = std::fs::read_to_string("Input/DayFour.txt").unwrap();
    let dirX: Vec<i32> = vec![-1, 0, 1, 1, 1, 0, -1, -1];
    let dirY: Vec<i32> = vec![-1, -1, -1, 0, 1, 1, 1, 0];

    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut base_vec: Vec<char> = vec![];
        for word in line.chars() {
            base_vec.push(word);
        }
        grid.push(base_vec);
    }
    let mut ans = 0;
    let nothing: bool = false;
    while !nothing {
        for i in 0..139 {
            for j in 0..139 {
                if grid[i][j] == '@' {
                    let mut count = 0;
                    for k in 0..8 {
                        let x = i as i32 + dirX[k];
                        let y = j as i32 + dirY[k];

                        if is_valid_position(x as i32, y as i32) {
                            if grid[x as usize][y as usize] == '@' {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 {
                        ans += 1;
                        grid[i][j] = '.';
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
