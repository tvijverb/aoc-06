// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};


pub struct Game {
    pub time: u64,
    pub distance_record: u64,
}

impl Game {
    pub fn new(time: u64, distance_record: u64) -> Game {
        Game {
            time,
            distance_record,
        }
    }

    pub fn ways_to_beat_record(&self) -> u64 {
        let mut ways = 0;
        for press_duration in 1..self.time {
            let distance = press_duration * (self.time - press_duration);
            if distance > self.distance_record {
                ways += 1;
            }
        }
        ways
    }
}

fn parse_game_time(line: &str) -> Vec<u64> {
    static GAMETIME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    // get character positions for each part number in the line
    let game_times = GAMETIME.find_iter(line);

    game_times.map(|m| u64::from_str(m.as_str()).unwrap()).collect()
}

fn parse_game_distance(line: &str) -> Vec<u64> {
    static GAMEDISTANCE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    // get character positions for each part number in the line
    let game_distances = GAMEDISTANCE.find_iter(line);

    game_distances.map(|m| u64::from_str(m.as_str()).unwrap()).collect()
}


fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let file2 = File::open("input2.txt")?;
    let reader = BufReader::new(file);
    let reader2 = BufReader::new(file2);
    let mut game_times = Vec::new();
    let mut game_distances = Vec::new();
    let mut games = Vec::new();

    let mut game_times2 = Vec::new();
    let mut game_distances2 = Vec::new();
    let mut games2 = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if idx == 0 {
            game_times = parse_game_time(&line);
        } else if idx == 1 {
            game_distances = parse_game_distance(&line);
        }
    }

    for (idx, line) in reader2.lines().enumerate() {
        let line = line?;
        if idx == 0 {
            game_times2 = parse_game_time(&line);
        } else if idx == 1 {
            game_distances2 = parse_game_distance(&line);
        }
    }

    for (game_time, game_distance) in game_times.iter().zip(game_distances.iter()) {
        games.push(Game::new(*game_time, *game_distance));
    }


    for (game_time, game_distance) in game_times2.iter().zip(game_distances2.iter()) {
        games2.push(Game::new(*game_time, *game_distance));
    }

    // multiply the number of ways to beat the record for each game
    let mul = games.iter().fold(1, |acc, game| acc * game.ways_to_beat_record());
    println!("{}", mul);

    let mul2 = games2.iter().fold(1, |acc, game| acc * game.ways_to_beat_record());

    println!("{}", mul2);

    Ok(())
}