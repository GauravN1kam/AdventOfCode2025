fn parse_input() -> (String, String) {
    let contents = std::fs::read_to_string("Input/DayFive.txt").unwrap();
    let vec: Vec<&str> = contents.split("\n\n").collect();

    (vec[0].into(), vec[1].into())
}
pub fn part1() {
    let (rnge, inp) = parse_input();
    let mut inp_range: Vec<(i128, i128)> = vec![];

    for i in rnge.lines().filter(|&a| !a.is_empty()) {
        let tmp: Vec<&str> = i.split("-").collect();
        let a: i128 = tmp[0].parse().unwrap();
        let b: i128 = tmp[1].parse().unwrap();
        inp_range.push((a, b));
    }
    inp_range.sort_by_key(|a| a.0);

    let mut ans = 0;
    for i in inp.lines().filter(|a| !a.is_empty()) {
        let check: i128 = i.parse().unwrap();

        for k in &inp_range {
            if check >= k.0 && check <= k.1 {
                ans += 1;
                break;
            }
        }
    }
    println!("{ans}");
}

pub fn part2() {
    let (input, _) = parse_input();
    let mut size = 0;
    let mut inp_range: Vec<(i128, i128)> = vec![];
    for i in input.lines().filter(|&a| !a.is_empty()) {
        let tmp: Vec<&str> = i.split("-").collect();
        let a: i128 = tmp[0].parse().unwrap();
        let b: i128 = tmp[1].parse().unwrap();
        inp_range.push((a, b));
        size += 1;
    }
    let mut ans: i128 = inp_range[0].1 - inp_range[0].0 + 1;
    inp_range.sort_by_key(|a| a.0);
    // let mut min_tuple: (i128, i128) = inp_range[0];
    // for i in &inp_range {
    //     if i.0 <= min_tuple.1 && i.1 <= min_tuple.1 {
    //         ans += 0;
    //     } else if i.0 <= min_tuple.1 && min_tuple.1 <= i.1 {
    //         ans += (i.1 - min_tuple.1) + 1;
    //         min_tuple.1 = i.1;
    //     } else if min_tuple.0 <= i.0 && min_tuple.1 <= i.1 {
    //         ans += (i.1 - i.0) + 1;
    //         min_tuple = (i.0, i.1);
    //     }
    // }
    let mut merged_list: Vec<(i128, i128)> = vec![];

    for (start, end) in inp_range {
        if let Some((last_start, last_end)) = merged_list.last_mut() {
            if start <= *last_end + 1 {
                if end > *last_end {
                    *last_end = end;
                }
            } else {
                merged_list.push((start, end));
            }
        } else {
            merged_list.push((start, end));
        }
    }

    ans = merged_list.iter().map(|(start, end)| end - start + 1).sum();
    println!("{}", size);
    println!("ans = {ans}");
}
