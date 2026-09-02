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
//!
//! Chrome (header, tab strip, footer, input bar, command blocks) is not stored
//! but derived from the active profile background via paint::shade::elevate,
//! so every profile including the translucent ones stays complete.

pub mod profiles;
pub mod types;
