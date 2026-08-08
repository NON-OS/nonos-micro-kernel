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

//! Saying what happened at each step of a message's life.
//!
//! A mixnet gives no feedback by design: a packet that is malformed, routed
//! wrongly, or refused looks exactly like one that was delivered. Nothing
//! comes back either way. That leaves the log as the only place a failure is
//! visible, so every step that can refuse says so and why.

mod emit;
mod write;

pub use emit::{say, say_num, say_text, say_two};
