/*
In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6. 
 * */
use std::fs::File;
use std::io::prelude::*;

fn proper_rules(result:char, opponent:char) -> i32{
    let mut point:i32 = 0;
    match result {
        'X' => match opponent {
            'A' => point += 3,
            'B' => point += 1,
            'C' => point += 2,
            _ => point += 0,},
        'Y' => match opponent {
            'A' => point += 3 + 1,
            'B' => point += 3 + 2,
            'C' => point += 3 + 3,
            _ => point += 0,},

        'Z' => match opponent {
            'A' => point += 6 + 2,
            'B' => point += 6 + 3,
            'C' => point += 6 + 1,
            _ => point += 0,},     
        _ => point += 0,
    };
    point
}


fn get_winner(player:char, opponent:char) -> i32 {

    let win:i32 = match player {
        'X' =>  match opponent {
            'A' => 2,
            'B' => 0,
            'C' => 1,
            _ => 5,
        },

        'Y' => match opponent {
            'A' => 1,
            'B' => 2,
            'C' => 0,
            _ => 5,
        },
        'Z' => match opponent {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => 5,
        },

        _ => 0,
            
    };
    win

}

fn get_points(hand:char, result:i32) -> i32 {
    let mut point:i32 = match hand {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,    
        _ => 0,
    };

    point += match result {
        0 => 0,
        1 => 6,
        2 => 3,
        _ => 0,
    };
    point
}

//opens file 
fn get_phrase() -> String {
    let mut file = File::open("plays.txt").expect("when");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Does not exist");
    
    content
}

pub fn run_rock() {
/* A = rock score of 1 = X
 * B = paper score of 2 = Y
 * C = Scissors score of 3 = Z
 *6 points if I win 
 * strat guide
 *A Y
 *B X
 *C Z
 * */
    let games: String = get_phrase();       
    let mut real_points:i32 = 0;
    let mut points:i32 = 0; 
    for line in games.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        let result: i32 = get_winner(line_vec[2], line_vec[0]);
        points += get_points(line_vec[2], result);
        real_points += proper_rules(line_vec[2], line_vec[0]);
    }

    println!("{}", points);
    println!("{}", real_points);
}
