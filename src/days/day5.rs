use std::collections::VecDeque;

pub fn run() {
    part_a();
    part_b();
}

fn part_a() {
    let mut stacks_data = include_str!("input_files/day5_stacks.txt")
        .lines()
        .peekable();
    let moves = include_str!("input_files/day5_moves.txt").lines();

    let stacks_count = (stacks_data.peek().unwrap().len() / 4) + 1;
    let mut stacks: Vec<Vec<&str>> = vec![vec![]; stacks_count];
    for line in stacks_data {
        for (stack_index, (line_index, letter)) in line.chars().enumerate().step_by(4).enumerate() {
            if letter == '[' {
                stacks[stack_index].push(&line[line_index + 1..line_index + 2]);
            }
        }
    }
    stacks.iter_mut().for_each(|stack| stack.reverse());
    for stack_move in moves {
        let v = stack_move
            .split_whitespace()
            .skip(1)
            .take(5)
            .collect::<Vec<&str>>();
        if let [stack_moves_count, _, from, _, to] = &v[..] {
            let from = from.parse::<usize>().unwrap() - 1;
            let to = to.parse::<usize>().unwrap() - 1;
            for _ in 0..stack_moves_count.parse::<u8>().unwrap() {
                let value = stacks[from].pop().unwrap();
                stacks[to].push(value);
            }
        }
    }

    let result = stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect::<Vec<&str>>()
        .join("");
    println!("a) {result}");
}

fn part_b() {
    let mut stacks_data = include_str!("input_files/day5_stacks.txt")
        .lines()
        .peekable();
    let moves = include_str!("input_files/day5_moves.txt").lines();

    let stacks_count = (stacks_data.peek().unwrap().len() / 4) + 1;
    let mut stacks: Vec<VecDeque<&str>> = vec![VecDeque::new(); stacks_count];
    for line in stacks_data {
        for (stack_index, (line_index, letter)) in line.chars().enumerate().step_by(4).enumerate() {
            if letter == '[' {
                stacks[stack_index].push_front(&line[line_index + 1..line_index + 2]);
            }
        }
    }
    for stack_move in moves {
        let v = stack_move
            .split_whitespace()
            .skip(1)
            .take(5)
            .collect::<Vec<&str>>();
        if let [stack_moves_count, _, from, _, to] = &v[..] {
            let from = from.parse::<usize>().unwrap() - 1;
            let to = to.parse::<usize>().unwrap() - 1;
            let mut items_to_move = vec![];
            for _ in 0..stack_moves_count.parse::<u8>().unwrap() {
                items_to_move.push(stacks[from].pop_back().unwrap());
            }
            items_to_move
                .iter()
                .rev()
                .for_each(|item| stacks[to].push_back(item));
        }
    }

    let result = stacks
        .iter_mut()
        .map(|stack| stack.pop_back().unwrap())
        .collect::<Vec<&str>>()
        .join("");
    println!("b) {result}");
}
