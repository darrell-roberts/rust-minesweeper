use crate::lib::{Board, DimensionError, GameState, Pos, ProgramArgs};
use clap::Parser;
use std::{
  io::{stdin, stdout, Write},
  str::FromStr,
};
use thiserror::Error;

mod lib;

/// User command.
#[derive(Debug, PartialEq, Eq)]
enum Command {
  Open(Pos),
  Flag(Pos),
  Quit,
}

/// Invalid command error.
#[derive(Debug, Error)]
enum InvalidCommand {
  #[error("Invalid command: {0}")]
  Command(String),
  #[error("Invalid dimension: {0}")]
  Dimension(#[from] DimensionError),
  #[error("IO Error: {0}")]
  IO(#[from] std::io::Error),
}

/// Parse coordinates provided by user.
fn parse_coords(x: &str, y: &str) -> Result<Pos, InvalidCommand> {
  x.parse()
    .and_then(|x| y.parse().map(|y| (x, y)))
    .map(|(x, y)| Pos { x, y })
    .map(Ok)?
}

impl FromStr for Command {
  type Err = InvalidCommand;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s.split(' ').collect::<Vec<_>>();
    match parts.as_slice() {
      ["q"] => Ok(Command::Quit),
      &["o", x, y] => parse_coords(x, y).map(Command::Open),
      &["f", x, y] => parse_coords(x, y).map(Command::Flag),
      _ => Err(InvalidCommand::Command(s.to_owned())),
    }
  }
}

/// Parse user input.
fn parse_command() -> Result<Command, InvalidCommand> {
  let mut input = String::new();
  stdin().read_line(&mut input)?;
  input.trim().parse()
}

/// Main game loop. Draws the board and takes user input
/// until win/loss or quit.
fn game_loop(mut board: Board) {
  loop {
    println!("{board}");

    let state = board.state();

    if let GameState::Loss = state {
      println!("You Lose!");
      break;
    }

    if let GameState::Win = state {
      println!("You Win!");
      break;
    }

    print!("(o, f, q): ");
    stdout().flush().unwrap();
    match parse_command() {
      Ok(Command::Quit) => break,
      Ok(Command::Open(p)) => {
        board.open_cell(p);
      }
      Ok(Command::Flag(p)) => {
        board.flag_cell(p);
      }
      Err(e) => {
        eprintln!("Invalid command: {e}");
        continue;
      }
    }
  }
}

/// Parse command line arguments and start game.
fn main() {
  let program_opts = ProgramArgs::parse();
  let board = Board::new(program_opts.columns, program_opts.rows);
  game_loop(board);
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::lib::{Dimension, DimensionError};
  #[test]
  fn test_parse() {
    let open = "o 1 1".parse::<Command>().unwrap();
    assert_eq!(
      open,
      Command::Open(Pos {
        x: Dimension::try_from(1).unwrap(),
        y: Dimension::try_from(1).unwrap()
      })
    );
    let flag = "f 1 1".parse::<Command>().unwrap();
    assert_eq!(
      flag,
      Command::Flag(Pos {
        x: Dimension::try_from(1).unwrap(),
        y: Dimension::try_from(1).unwrap()
      })
    );
    let quit = "q".parse::<Command>().unwrap();
    assert_eq!(quit, Command::Quit);
    let invalid = "abc".parse::<Command>();
    assert!(matches!(invalid, Err(InvalidCommand::Command(s)) if s == "abc"));
    let overflow = "o 260 2".parse::<Command>();
    dbg!(&overflow);
    assert!(matches!(
      overflow,
      Err(InvalidCommand::Dimension(DimensionError::Int(_)))
    ));
  }
}
