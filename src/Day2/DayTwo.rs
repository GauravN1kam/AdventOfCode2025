fn parse_input() -> Vec<Vec<i128>> {
    let contents = std::fs::read_to_string("Input/DayTwo.txt").unwrap();
    let mut input: Vec<Vec<i128>> = Vec::new();
    for line in contents.lines().filter(|f| !f.is_empty()) {
        let inp = line.split(",");
        for i in inp {
            let vals: Vec<&str> = i.split("-").collect();
            let low: i128 = vals[0].parse().unwrap();
            let high: i128 = vals[1].parse().unwrap();
            input.push(vec![low, high]);
        }
    }
    input
}

fn is_valid(num: &i128) -> bool {
    let num_str = num.to_string();
    let size_of_num = num_str.len();

    let check_range = size_of_num / 2;
    for i in 1..=size_of_num / 2 {
        if size_of_num % i == 0 {
            let mut valid = true;
            for j in 0..size_of_num - i {
                if num_str.chars().nth(j) != num_str.chars().nth(j + i) {
                    valid = false;
                    break;
                }
                // if &num_str[j..j + i] != &num_str[j + 1 + i..j + 1 + 2 * i] {
                //     valid = false;
                //     break;
                // }
            }
            if valid {
                return true;
            }
        }
    }

    false
}

pub fn part1() {
    let contents = std::fs::read_to_string("Input/DayTwo.txt").unwrap();

    let mut ans: i128 = 0;
    for line in contents.lines().filter(|f| !f.is_empty()) {
        let inp = line.split(",");
        for i in inp {
            let vals: Vec<&str> = i.split("-").collect();
            let mut low: i128 = vals[0].parse().unwrap();
            let high: i128 = vals[1].parse().unwrap();
            while low <= high {
                let mut ten: i128 = 10;
                let numberofDigits = low.checked_ilog10().unwrap_or(0);
                let tmp: i128 = ten.pow((numberofDigits + 1) / 2);

                let first_half = low / tmp;
                let second_half = low % tmp;

                if first_half == second_half {
                    ans += low;
                }
                low += 1;
            }
        }
    }
    println!("{}", ans);
}

pub fn part2() {
    let mut ans: i128 = 0;
    let input = parse_input();
    for i in &input {
        println!("low = {}, high = {}", i[0], i[1]);
        let mut low = i[0];
        let high = i[1];

        while low < high {
            if is_valid(&low) {
                ans += low;
                println!("{}", low);
            }
            low += 1;
        }
    }
    println!("ans = {:?}", ans);
}
