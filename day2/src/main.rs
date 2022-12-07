use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum State {
    Win,
    Draw,
    Lose
}

#[derive(Debug)]
struct Game {
    opponent: Action,
    response: Action,
    state : State
}

impl Action {

    fn parse(text: &str) -> Action {
        match text {
            "A" | "X" => Action::Rock,
            "B" | "Y" => Action::Paper,
            "C" | "Z" => Action::Scissors,
            e => panic!("unknown action {}", e)
        }
    }

    fn score(&self) -> u32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3
        }
    }
}

impl State {

    fn parse(text: &str) -> State {
        match text {
            "X" => State::Lose,
            "Y" => State::Draw,
            "Z" => State::Win,
            e => panic!("unknown action {}", e)
        }
    }

    fn to_action(&self, action: &Action) -> Action {
        match self {
            State::Draw => action.clone(),
            State::Win => match action {
                Action::Rock => Action::Paper,
                Action::Paper => Action::Scissors,
                Action::Scissors => Action::Rock
            },
            State::Lose => match action {
                Action::Rock => Action::Scissors,
                Action::Paper => Action::Rock,
                Action::Scissors => Action::Paper
            }
        }
    }

    fn score(&self) -> u32 {
        match self {
            State::Win => 6,
            State::Draw => 3,
            State::Lose => 0
        }
    }
}

impl Game {

    fn score(&self) -> u32 {
        if self.is_draw() {
            3 + self.response.score()
        } else if self.is_win() {
            6 + self.response.score()
        } else {
            self.response.score()
        }
    }

    fn state_score(&self) -> u32 {
        self.state.score() + self.state.to_action(&self.opponent).score()
    }

    fn is_win(&self) -> bool {
        match self.response {
            Action::Rock => self.opponent != Action::Paper,
            Action::Paper => self.opponent != Action::Scissors,
            Action::Scissors => self.opponent != Action::Rock
        }
    }

    fn is_draw(&self) -> bool {
        self.response == self.opponent
    }
}




fn parse() -> Vec<Game> {
    let path = env::args().nth(1).expect("missing file path");
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);

    let games: Vec<Game> = buffer.lines()
        .map(|l| l.expect("cannot read line"))
        .map(|l| {
            let mut parts = l.split(" ");
            let opponent = parts.next().expect("no opponent on line");
            let response = parts.next().expect("no response on line");
            Game {
                opponent: Action::parse(opponent),
                response: Action::parse(response),
                state: State::parse(response),
            }
        })
        .collect();

    return games
}

fn main() {
    let games = parse();
    let score: u32 = games.iter().map(|game| {
        game.score()
    })
    .sum();

    let state_score: u32 = games.iter().map(|game| {
        game.state_score()
    })
    .sum();

    println!("{:?}", games);
    println!("score : {}", score);
    println!("state score : {}", state_score);
}
