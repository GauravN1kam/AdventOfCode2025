fn parse_input() -> (Vec<Vec<i128>>, Vec<String>) {
    let content = std::fs::read_to_string("Input/DaySix.txt").unwrap();

    let mut numbers: Vec<Vec<i128>> = vec![];
    let mut size = 1;
    let mut pattern: Vec<String> = vec![];
    for i in content.lines().filter(|a| !a.is_empty()) {
        if size == 5 {
            pattern = i.split_whitespace().map(|a| a.to_string()).collect();
            println!("{:?}", pattern.len());

            break;
        }

        let k: Vec<&str> = i.split_whitespace().collect();

        let v: Vec<i128> = k.iter().map(|a| a.parse().unwrap()).collect();

        numbers.push(v);

        size += 1;
    }

    (numbers, pattern)
}

pub fn part1() {
    let (numbers, pattern) = parse_input();

    let cols = numbers[0].len();
    let rows = numbers.len();

    let mut ans = 0;
    for col in 0..cols {
        if pattern[col] == "*" {
            let mut tmp = 1;
            for row in 0..rows {
                tmp *= numbers[row][col];
            }
            ans += tmp;
        } else if pattern[col] == "+" {
            let mut tmp = 0;
            for row in 0..rows {
                tmp += numbers[row][col];
            }
            ans += tmp;
        }
    }
    println!("ans = {ans}");
}

pub fn part2() {
    let contents = std::fs::read_to_string("Input/DaySix.txt").unwrap();
    let mut input: Vec<String> = contents.lines().map(String::from).collect();

    let mut sum: u64 = 0;

    let col = input[0].len();
    let row = input.len();

    let mut col_sums: Vec<u64> = vec![];

    for i in (0..col).rev() {
        let mut temp: u64 = 0;

        for j in 0..row {
            let ch = input[j].as_bytes()[i];

            match ch {
                b' ' => (),
                b'0'..=b'9' => {
                    temp = temp * 10 + (ch - b'0') as u64;
                }
                b'+' | b'*' => {
                    col_sums.push(temp);

                    match ch {
                        b'+' => sum += col_sums.iter().sum::<u64>(),
                        b'*' => sum += col_sums.iter().product::<u64>(),
                        _ => (),
                    }
                    col_sums.clear();
                    temp = 0;
                }
                _ => (),
            }
        }

        if temp > 0 {
            col_sums.push(temp);
        }
    }
    println!("{sum}");
}
