pub mod battleship;
pub mod client;
pub mod server;

use battleship::*;
use std::{env, io, io::prelude::*};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum BattleshipError {
    InvalidOption(String)
}

impl Display for BattleshipError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidOption(msg) => write!(f, "Error: Invalid option - {}", msg)
        }
    }
}

impl std::error::Error for BattleshipError { }

#[derive(Debug)]
enum Player {
    Server,
    Client,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Battleship!");

    let args: Vec<String> = env::args().collect();

    let user_mode_str = if args.len() > 1 {
        args[1].clone()
    } else {
        print!("Do you want to or Host your own game, or Join someone else's? [h/j] (default: h) ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        choice
    };

    let user_mode = match user_mode_str.trim().to_lowercase().as_str() {
        "h" => Player::Server,
        "j" => Player::Client,
        _ => return Err(Box::new(BattleshipError::InvalidOption("Option not recognized: '{user_mode_str}'. Choose h for host, or j for join".to_string())))
    };

    println!("Selected user mode is {:#?}.", user_mode);

    match user_mode {
        Player::Server => server::launch_game(),
        Player::Client => client::join_game(),
    }

    // let ships: [Ship; 5] = [Ship {
    //     start: Vec2i { x: 0, y: 0 },
    //     end: Vec2i { x: 0, y: 1 },
    // }; 5];

    // let board = Board::new(ships);
}
