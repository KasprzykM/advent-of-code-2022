use std::fs;

pub fn run() {
    part_a();
    part_b();
}

fn part_b() {
    let file_data: String = fs::read_to_string("src/days/input_files/day4.txt").unwrap();
    let mut overlapping_ranges = 0;
    for line in file_data.split('\n') {
        let (first_range, second_range) = match &line.split(',').take(2).collect::<Vec<&str>>()[..]
        {
            &[first, second, ..] => (first, second),
            _ => unreachable!(),
        };
        let (a1, b1) = get_range(first_range);
        let (a2, b2) = get_range(second_range);

        if (b1 >= a2 && b1 <= b2) || (b2 >= a1 && b2 <= b1) {
            overlapping_ranges += 1;
        }
    }
    println!("b) {}", overlapping_ranges);
}

fn part_a() {
    let file_data: String = fs::read_to_string("src/days/input_files/day4.txt").unwrap();
    let mut overlapping_ranges = 0;
    for line in file_data.split('\n') {
        let (first_range, second_range) = match &line.split(',').take(2).collect::<Vec<&str>>()[..]
        {
            &[first, second, ..] => (first, second),
            _ => unreachable!(),
        };
        let (a1, b1) = get_range(first_range);
        let (a2, b2) = get_range(second_range);

        if (a1 <= a2 && b1 >= b2) || (a2 <= a1 && b2 >= b1) {
            overlapping_ranges += 1;
        }
    }
    println!("a) {}", overlapping_ranges);
}

fn get_range(string_rep: &str) -> (i32, i32) {
    match &string_rep
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()[..]
    {
        &[first, second, ..] => (first, second),
        _ => unreachable!(),
    }
}
