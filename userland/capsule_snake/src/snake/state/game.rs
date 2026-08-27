// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;

use super::difficulty::Difficulty;
use super::mode::Mode;
use super::options::Options;
use super::phase::{Dir, Phase};
use super::run::RunRecord;
use super::screen::Screen;

pub struct Game {
    pub screen: Screen,
    pub phase: Phase,
    pub mode: Mode,
    pub diff: Difficulty,
    pub opts: Options,
    pub body: Vec<(i16, i16)>,
    pub walls: Vec<(i16, i16)>,
    pub dir: Dir,
    pub pending: Dir,
    pub food: (i16, i16),
    pub power: Option<(i16, i16)>,
    pub score: u32,
    pub lives: u8,
    pub level: usize,
    pub streak: u32,
    pub longest: u16,
    pub base_ms: i64,
    pub interval_ms: i64,
    pub elapsed: i64,
    pub deadline: i64,
    pub slow_until: i64,
    pub last_ms: i64,
    pub runs: Vec<RunRecord>,
    pub awards: Vec<u16>,
    pub rng: u64,
}
