//! Board implementation for handling game play.
use super::{Board, Cell, CellState, GameState, Pos};
use crate::lib::Dimension;
use rand::prelude::*;
use std::{collections::BTreeMap, iter};

impl Board {
  /// Create a new board with the given columns and rows.
  pub fn new(columns: Dimension, rows: Dimension) -> Self {
    let xs = 1..=*columns;
    let ys = 1..=*rows;

    // Generate a cartesian product. Similar to my approach in Haskell.
    let cells = ys
      .flat_map(|y| xs.clone().map(move |x| (x, y)))
      .map(|(x, y)| {
        (
          Pos {
            x: Dimension(x),
            y: Dimension(y),
          },
          Cell::default(),
        )
      })
      .collect::<BTreeMap<_, _>>();
    Board {
      cells,
      columns,
      rows,
      state: GameState::New,
    }
  }

  /// Randomly mine the board with a difficulty ratio. Exclude mining the provided position.
  fn mine_board(&mut self, exclude_pos: &Pos) {
    // Set the total amount of mined cells based on
    // difficulty level.
    let total_mined_cells =
      ((*self.rows as f64 * *self.columns as f64) * 0.10) as usize;

    let mut rng = thread_rng();
    let mut random_cell_iter = iter::from_fn(|| {
      let x = rng.gen_range(1..*self.columns);
      let y = rng.gen_range(1..*self.rows);
      Some(Pos {
        x: Dimension(x),
        y: Dimension(y),
      })
    })
    .filter(|pos| pos != exclude_pos);

    // Keep setting mined cells until we hit the limit.
    while self.total_mines() < total_mined_cells {
      if let Some(pos) = random_cell_iter.next() {
        if let Some(Cell {
          state: CellState::Closed { mined, .. },
          ..
        }) = self.cells.get_mut(&pos)
        {
          *mined = true;
        }
      }
    }

    // This is consumed here to avoid violating using both a mutable and non-mutable
    // shared reference.
    let adjacent_pos = self
      .cells
      .keys()
      .map(|&pos| (pos, self.total_adjacent_mines(&pos)))
      .collect::<Vec<_>>();

    for (pos, n) in adjacent_pos {
      if let Some(cell) = self.cells.get_mut(&pos) {
        cell.adjacent_mines = n;
      }
    }
  }

  /// Get the total number of mined cells.
  pub fn total_mines(&self) -> usize {
    self
      .cells
      .values()
      .filter(|&&cell| match cell {
        Cell {
          state: CellState::Closed { mined, .. },
          ..
        } => mined,
        Cell {
          state: CellState::ExposedMine,
          ..
        } => true,
        _ => false,
      })
      .count()
  }

  /// Open requested cell and expand if necessary.
  pub fn open_cell(&mut self, pos: Pos) {
    if let Some(c) = self.cells.get_mut(&pos) {
      if let CellState::Closed { mined: true, .. } = c.state {
        self.expose_mines();
        self.state = GameState::Loss
      } else if let CellState::Closed { mined: false, .. } = c.state {
        let am = c.adjacent_mines;
        c.state = CellState::Open;
        if self.is_win() {
          self.state = GameState::Win;
        } else if am == 0 {
          if self.state == GameState::New {
            // This is the first move in the game. We will mine the
            // board now and avoid mining the position being opened.
            self.mine_board(&pos);
            self.state = GameState::Active;
          }
          self.expand(&pos)
        }
      }
    }
  }

  /// Open all adjacent cells that are not mined or flagged.
  fn expand(&mut self, pos: &Pos) {
    for p in adjacent_cells(pos, *self.rows, *self.columns) {
      if let Some(Cell {
        state:
          CellState::Closed {
            mined: false,
            flagged: false,
          },
        ..
      }) = self.cells.get(&p)
      {
        self.open_cell(p)
      }
    }
  }

  /// Expose all mined cells on the board.
  fn expose_mines(&mut self) {
    for c in self.cells.values_mut() {
      if let Cell {
        state: CellState::Closed { mined: true, .. },
        ..
      } = c
      {
        c.state = CellState::ExposedMine
      }
    }
  }

  /// Flag the cell as being potentially mined.
  pub fn flag_cell(&mut self, pos: Pos) {
    if let Some(c) = self.cells.get_mut(&pos) {
      if let CellState::Closed { flagged, .. } = &mut c.state {
        *flagged = true;
      }
    }
  }

  /// Get the state of the board.
  pub fn state(&self) -> &GameState {
    &self.state
  }

  /// Get the number of board cells.
  pub fn board_size(&self) -> usize {
    self.cells.len()
  }

  /// Evaluate board to see if all non mined cells have been opened.
  fn is_win(&self) -> bool {
    let open_and_flagged_cell_count = self
      .cells
      .values()
      .filter(|&&cell| {
        matches!(
          cell,
          Cell {
            state: CellState::Open,
            ..
          } | Cell {
            state: CellState::Closed { flagged: true, .. },
            ..
          }
        )
      })
      .count();
    let total_open_for_win = self.cells.len() - self.total_mines();
    total_open_for_win == open_and_flagged_cell_count
  }

  /// Count how many adjacent positions are mined.
  fn total_adjacent_mines(&self, pos: &Pos) -> u8 {
    adjacent_cells(pos, *self.rows, *self.columns)
      .filter_map(|p| self.cells.get(&p))
      .filter(|&&c| {
        matches!(
          c,
          Cell {
            state: CellState::Closed { mined: true, .. },
            ..
          }
        )
      })
      .count() as u8
  }
}

/// Generate adjacent positions from provided position.
fn adjacent_cells(
  &Pos {
    x: Dimension(x),
    y: Dimension(y),
  }: &Pos,
  max_rows: u8,
  max_columns: u8,
) -> impl Iterator<Item = Pos> + '_ {
  let edges = |n| [n - 1, n, n + 1].into_iter();

  edges(x)
    .flat_map(move |x1| edges(y).map(move |y1| (x1, y1)))
    .filter(move |&(x1, y1)| x1 > 0 && y1 > 0 && (x1, y1) != (x, y))
    .filter(move |&(x, y)| x <= max_rows && y <= max_columns)
    .map(|(x, y)| Pos {
      x: Dimension(x),
      y: Dimension(y),
    })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_board_new() {
    let board_max = Dimension(5);
    let test_board = Board::new(board_max, board_max);
    dbg!(&test_board);
    assert_eq!(test_board.cells.len() as u8, *board_max * *board_max);
  }

  #[test]
  fn test_mined_cells() {
    let mut board = Board::new(Dimension(5), Dimension(5));
    board.mine_board(&Pos {
      x: Dimension(1),
      y: Dimension(1),
    });
    dbg!(&board);
    assert!(board.total_mines() > 0);
  }

  #[test]
  fn test_adjacent() {
    let adjacent = adjacent_cells(
      &Pos {
        x: Dimension(1),
        y: Dimension(1),
      },
      10,
      10,
    )
    .collect::<Vec<_>>();
    dbg!(&adjacent);
    assert_eq!(adjacent.len(), 3);
    let adjacent = adjacent_cells(
      &Pos {
        x: Dimension(3),
        y: Dimension(3),
      },
      10,
      10,
    )
    .collect::<Vec<_>>();
    dbg!(&adjacent);
    assert_eq!(adjacent.len(), 8);
    let adjacent = adjacent_cells(
      &Pos {
        x: Dimension(2),
        y: Dimension(1),
      },
      10,
      10,
    )
    .collect::<Vec<_>>();
    dbg!(&adjacent);
  }
}
