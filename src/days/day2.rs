use std::collections::HashMap;
use std::fs;

pub fn run() {
    let move_values: HashMap<&str, i32> = HashMap::from([
        ("X", 1), // Rock
        ("Y", 2), // Paper
        ("Z", 3), // Scissors
        /* Opponent moves */
        ("A", 1), // Rock
        ("B", 2), // Paper
        ("C", 3), // Scissors
    ]);
    let file_data: String = fs::read_to_string("src/days/input_files/day2.txt").unwrap();
    let moves: Vec<&str> = file_data.split('\n').into_iter().collect();
    let mut total_points = 0;
    for game_move in moves {
        if let [opponent_move, my_move] =
            game_move.split_whitespace().take(2).collect::<Vec<&str>>()[..]
        {
            total_points += calculate_points(opponent_move, my_move, &move_values);
        }
    }
    println!("{}", total_points);
}

fn calculate_points(
    opponent_choice: &str,
    my_strategy: &str,
    move_values: &HashMap<&str, i32>,
) -> i32 {
    let my_choice = get_move(opponent_choice, my_strategy);
    let choice_points = move_values.get(my_choice).copied().unwrap();
    if opponent_choice == "A" {
        choice_points
            + match my_choice {
                "A" => 3,
                "B" => 6,
                "C" => 0,
                _ => panic!("Wrong Choice"),
            }
    } else if opponent_choice == "B" {
        choice_points
            + match my_choice {
                "A" => 0,
                "B" => 3,
                "C" => 6,
                _ => panic!("Wrong Choice"),
            }
    } else {
        choice_points
            + match my_choice {
                "A" => 6,
                "B" => 0,
                "C" => 3,
                _ => panic!("Wrong Choice"),
            }
    }
}

/*
     X = LOSS
     Y = DRAW
     Z = WIN
*/
fn get_move<'a>(opponent_move: &'a str, strategy: &'a str) -> &'a str {
    if opponent_move == "A" {
        match strategy {
            "X" => "C",
            "Y" => "A",
            "Z" => "B",
            _ => panic!("Wrong Choice"),
        }
    } else if opponent_move == "B" {
        match strategy {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => panic!("Wrong Choice"),
        }
    } else {
        match strategy {
            "X" => "B",
            "Y" => "C",
            "Z" => "A",
            _ => panic!("Wrong Choice"),
        }
    }
}
