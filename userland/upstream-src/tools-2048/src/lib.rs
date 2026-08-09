#![cfg_attr(not(test), no_std)]
//! **tools-2048** is a library that provides the core logic of the popular game 2048, along with a basic AI to play the game.
//! Arbitrary board sizes are supported with the minimum being 4x4.
//!
//! Example usage:
//! ```rust
//! use tools_2048::{Game, GameMove, GameState, GameResult};
//!
//! // create a new game with a 4x4 board
//! let mut game: Game<4> = Game::new().unwrap();
//!
//! // make a move
//! game.make_move(GameMove::Left);
//! game.make_move(GameMove::Right);
//! game.make_move(GameMove::Up);
//! game.make_move(GameMove::Down);
//!
//! assert_eq!(game.state(), GameState::InProgress);  // the game should still be in progress
//! assert_eq!(game.result(), GameResult::Pending);  // the result shouldn't be decided yet
//! ```

pub mod core;
pub mod error;

#[doc(inline)]
pub use core::*;

#[doc(inline)]
pub use error::*;
