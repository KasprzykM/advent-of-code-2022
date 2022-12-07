use std::{collections::HashSet, hash::Hash};

pub fn run() {
    let part_a = _first_unique_window(4).unwrap();
    println!("a) {part_a}");
    let part_b = _first_unique_window(14).unwrap();
    println!("b) {part_b}");
}

fn _first_unique_window(window_size: usize) -> Option<usize> {
    let line = include_str!("input_files/day6.txt")
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    for (index, char_windows) in line.windows(window_size).enumerate() {
        if has_unique_elements(char_windows) {
            let result = index + window_size;
            return Some(result);
        }
    }
    None
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    iter.into_iter().all(move |x| unique.insert(x))
}
