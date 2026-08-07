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

//! The type ramp and the spacing unit.
//!
//! The screens grew twenty distinct text sizes, chosen one at a time, which
//! is why nothing lined up: two labels meant to match would differ by half a
//! point and read as a mistake. Six steps on a consistent ratio cover every
//! case the wallet has, and a single spacing unit means margins are multiples
//! of one number rather than twenty hand-picked ones.

/// Micro labels, set in capitals: "TOTAL BALANCE", "LOCK TERM".
pub const LABEL: f32 = 12.1;
/// Secondary text: hints, availability, the quiet half of a row.
pub const SMALL: f32 = 13.8;
/// Default body text and control labels.
pub const BODY: f32 = 14.9;
/// Figures inside a card, and anything the reader is meant to read first.
pub const VALUE: f32 = 17.2;
/// Section headings.
pub const TITLE: f32 = 21.8;
/// The one number a screen exists to show.
pub const HERO: f32 = 32.2;

/// The spacing unit. Every gap, inset and margin is a multiple of this, so
/// vertical rhythm holds without each screen inventing its own.
pub const UNIT: u32 = 4;

/// `n` spacing units.
pub const fn space(n: u32) -> u32 {
    UNIT * n
}
