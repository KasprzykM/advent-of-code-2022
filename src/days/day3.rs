use std::collections::HashMap;
use std::fs;
pub fn run() {
    let file_data: String = fs::read_to_string("src/days/input_files/day3.txt").unwrap();
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let priorities: HashMap<char, usize> = alphabet
        .char_indices()
        .map(|(index, character)| (character, index + 1))
        .collect::<HashMap<char, usize>>();
    let mut sum = 0;
    for rucksack in file_data.split('\n') {
        let left = &rucksack[0..rucksack.len() / 2];
        let right = &rucksack[rucksack.len() / 2..rucksack.len()];
        let mut already_found = vec![];
        for item in left.chars() {
            if right.contains(item) && !already_found.contains(&item) {
                already_found.push(item);
                let value = priorities.get(&item).unwrap();
                sum += value;
            }
        }
    }
    println!("{}", sum);
}
