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

//! The Realtek MAC power-sequence engine: the command shape and the executor
//! that walks a transition table. The 8821CE power tables themselves are
//! vendor data to be ported verbatim from rtw88 (GPL, attributed); this is the
//! engine they run on, proven against a modeled device.

pub mod command;
mod run;
mod tables;

pub use run::run_pwr_seq;
pub use tables::CARD_ENABLE;
