use std::fs;

pub fn run() {
    let mut elfs: Vec<i32> = fs::read_to_string("src/days/input_files/day1.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|line| line.split('\n').map(|x| x.parse::<i32>().unwrap()).sum())
        .collect();
    elfs.sort_by(|a, b| b.cmp(a));
    println!("a) {:?}", elfs[0]);
    println!("b) {:?}", &elfs[0..3].iter().sum::<i32>());
}
