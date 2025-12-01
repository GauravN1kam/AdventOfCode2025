
pub fn part1() {
    let content = std::fs::read_to_string("Input/DayOneA.txt").unwrap();
    let mut ans = 0;
    let mut val = 50;
    for line in content.lines().filter(|f| !f.is_empty()) {
        let (dir, num) = line.split_at(1);
        let mut tmp: i32 = num.parse().unwrap();
        if dir == "L" {
            tmp = -tmp;
        }

        val = (val + tmp).rem_euclid(100);
        if val == 0 {
            ans += 1;
        }

        println!("Val =  {} : Temp = {}", val, tmp);
    }
    println!("Final  = {}", ans);
}

pub fn part2() {
    let content = std::fs::read_to_string("Input/DayOneA.txt").unwrap();
    let mut ans = 0;
    let mut val = 50;
    for line in content.lines().filter(|f| !f.is_empty()) {
        let (dir, num) = line.split_at(1);
        let mut tmp: i32 = num.parse().unwrap();
        if dir == "L" {
            tmp = -tmp;
        }
        if tmp >= 0 {
            ans += (val + tmp) / 100;
        } else {
            let reversed = (100 - val) % 100;
            ans += (reversed - tmp) / 100;
        }
        val = (val + tmp).rem_euclid(100);
    }
    println!("{}", ans);
}
