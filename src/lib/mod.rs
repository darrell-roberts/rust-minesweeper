//! Game types and trait implementations.
mod game;

use clap::Parser;
use std::{
  cmp::Ordering,
  collections::BTreeMap,
  fmt::{Display, Formatter},
  num::ParseIntError,
  ops::Deref,
  str::FromStr,
};
use thiserror::Error;

/// Command line arguments.
#[derive(Parser)]
pub struct ProgramArgs {
  #[clap(short, help = "Number of rows", action, default_value_t = Dimension(10))]
  pub rows: Dimension,
  #[clap(short, help = "Number of columns", action, default_value_t = Dimension(10))]
  pub columns: Dimension,
}

/// Newtype wrapper for non-zero cartesian coordinate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Dimension(u8);

impl Display for Dimension {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Deref for Dimension {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// A non-zero unsigned 8 bit integer.
#[derive(Error, Debug)]
pub enum DimensionError {
  #[error("Dimension can not be 0")]
  Zero,
  #[error("x y must be 1..255")]
  Int(#[from] ParseIntError),
}

impl TryFrom<u8> for Dimension {
  type Error = DimensionError;
  fn try_from(value: u8) -> Result<Self, Self::Error> {
    if value > 0 {
      Ok(Self(value))
    } else {
      Err(DimensionError::Zero)
    }
  }
}

impl FromStr for Dimension {
  type Err = DimensionError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let val = s.parse::<u8>()?;
    if val == 0 {
      Err(DimensionError::Zero)
    } else {
      Ok(Dimension(val))
    }
  }
}

/// Board cell.
#[derive(Debug, Copy, Clone, Default)]
struct Cell {
  state: CellState,
  adjacent_mines: u8,
}

/// Cell position on the board.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pos {
  pub x: Dimension,
  pub y: Dimension,
}

impl PartialOrd for Pos {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.y.cmp(&other.y).then(self.x.cmp(&other.x)))
  }
}

impl Ord for Pos {
  fn cmp(&self, other: &Self) -> Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
  }
}

/// State of the cell.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CellState {
  Open,
  Closed { flagged: bool, mined: bool },
  ExposedMine,
}

impl Default for CellState {
  fn default() -> Self {
    CellState::Closed {
      flagged: false,
      mined: false,
    }
  }
}

impl Display for Cell {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let num = self.adjacent_mines.to_string();
    write!(
      f,
      "{}",
      match self.state {
        CellState::Open =>
          if self.adjacent_mines > 0 {
            num.chars().take(1).next().expect("1 - 8 value")
          } else {
            ' '
          },
        CellState::Closed { flagged, .. } =>
          if flagged {
            'F'
          } else {
            '.'
          },
        CellState::ExposedMine => 'X',
      }
    )
  }
}

/// State of the game.
#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
  New,
  Active,
  Loss,
  Win,
}

/// Game board.
#[derive(Debug)]
pub struct Board {
  cells: BTreeMap<Pos, Cell>,
  columns: Dimension,
  rows: Dimension,
  state: GameState,
}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(
      f,
      "board: {}, mines: {}",
      self.board_size(),
      self.total_mines()
    )?;
    write!(f, "   ")?;
    for c in 1..=*self.columns {
      write!(f, "{c:<3}")?;
    }
    for (pos, cell) in self.cells.iter() {
      if *pos.x == 1 {
        write!(f, "\n{:<2} {cell}  ", *pos.y)?;
      } else {
        write!(f, "{cell:<3}  ")?;
      }
    }
    writeln!(f)
  }
}
