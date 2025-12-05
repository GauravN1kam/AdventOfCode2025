use std::{cmp::max, time::Instant, u32::MIN};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn parse_input() -> Vec<String> {
    let contents = std::fs::read_to_string("Input/DayThree.txt").unwrap();
    let mut inp: Vec<String> = Vec::new();
    for line in contents.lines().filter(|l| !l.is_empty()) {
        inp.push(line.to_string());
    }
    inp
}

fn find_max_num(num: &String) -> u32 {
    let mut array: Vec<u32> = vec![];
    for i in num.chars() {
        let n = i.to_digit(10).unwrap();
        array.push(n);
    }
    let mut max = MIN;
    for i in 0..num.len() - 1 {
        let to_compare = &array[i] * 10;
        for j in i + 1..num.len() {
            let current_num = to_compare + &array[j];
            if max < current_num {
                max = current_num;
            }
        }
    }
    max
}

fn f(array: &Vec<u128>, storage: &mut Vec<u128>, position: usize) -> u128 {
    if position >= 100 {
        return 0;
    }
    if storage.len() == 12 {
        let mut sum: u128 = 0;

        let store = storage.clone();
        for i in store {
            sum = sum * 10 + i;
        }
    }

    storage.push(array[position]);
    let take = f(array, storage, position + 1);
    storage.remove(storage.len() - 1);
    let dont_take = f(array, storage, position + 1);

    max(take, dont_take)
}

fn find_max_twelve_digit_number(num: &String) -> u128 {
    let mut array: Vec<u128> = vec![];
    for i in num.chars() {
        let n = i.to_digit(10).unwrap() as u128;
        array.push(n);
    }
    let mut positions: Vec<u128> = vec![];
    let mut start_location = 0;
    for i in 0..12 {
        let mut mx: u128 = 0;
        let mut tmp: u128 = 0;
        for j in start_location..(100 - 11 + i) {
            if mx < array[j] {
                mx = array[j];
                tmp = j as u128;
            }
        }
        positions.push(tmp);
        tmp += 1;
        start_location = tmp as usize;
    }
    let mut sum: u128 = 0;
    for i in positions {
        sum = sum * 10 + array[i as usize];
    }
    sum
}

pub fn part1() {
    let input = parse_input();
    let mut ans = 0;
    for line in input {
        ans += find_max_num(&line);
    }
    println!("ans = {}", ans)
}

pub fn part2() {
    let input = parse_input();
    let mut ans: u128 = 0;
    let start = Instant::now();
    for i in &input {
        let k = find_max_twelve_digit_number(&i);
        ans += k;
    }

    let duration = start.elapsed();
    let start_parrralel = Instant::now();
    let an: u128 = input
        .par_iter()
        .map(|x| {
            return find_max_twelve_digit_number(&x);
        })
        .sum();

    let an_duration = start_parrralel.elapsed();
    println!("ans = {} , duration = {:?}", ans, duration);
    println!("an  = {} , duration = {:?}", an, an_duration);
}
