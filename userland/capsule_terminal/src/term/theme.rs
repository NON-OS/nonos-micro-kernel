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

//! The terminal palette.
//!
//! Near black rather than pure black, because the chrome is drawn by lifting
//! the background a few steps and there is nowhere to lift from at zero. The
//! accent is one colour used consistently for what the system says about
//! itself, and it is kept apart from the colours that carry meaning: a reader
//! who has learned that green passed and red failed should not have to
//! relearn it because those are also the brand.

pub mod profiles;
pub mod types;

/// Body background. A faint cool cast keeps it from reading as a dead grey
/// once the compositor puts a wallpaper behind a translucent profile.
pub const BACKGROUND: u32 = 0xFF07_090B;

/// Body text. Slightly under white, because pure white on near black glares
/// at the sizes a terminal is read at for hours.
pub const FOREGROUND: u32 = 0xFFD6_DCE3;

/// The accent. One colour, used for the prompt, the cursor and the marks the
/// system makes, so the eye learns it in one place.
pub const ACCENT: u32 = 0xFF3F_D0C9;
pub const PROMPT: u32 = ACCENT;
pub const CURSOR: u32 = ACCENT;

/// Paths, a step lighter than the accent so a location reads as related to
/// the prompt without competing with it.
pub const PATH: u32 = 0xFF7F_DBCA;

/// Secondary text: hints, keybindings, timings. Legible against the
/// background and clearly subordinate to the body.
pub const DIM: u32 = 0xFF56_5E69;

// Chrome (header, tab strip, footer, input bar, command blocks) derives from
// the active `state.bg` via paint::shade::elevate, so every profile including
// the translucent ones stays complete.

/// What a command did. These carry meaning rather than identity, so they stay
/// the colours a terminal reader already knows.
pub const BLOCK_OK: u32 = 0xFF5F_D68A;
pub const BLOCK_ERR: u32 = 0xFFFF_5566;
pub const BLOCK_RUN: u32 = 0xFF56_5E69;
