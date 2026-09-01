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

/// A complete terminal palette. A profile carries every colour the surface
/// draws with, so switching one can never leave a foreground stranded on a
/// background it was never chosen against.
#[derive(Clone, Copy)]
pub struct Theme {
    pub bg: u32,
    pub fg: u32,
    pub accent: u32,
    pub path: u32,
    pub dim: u32,
    pub ok: u32,
    pub warn: u32,
    pub err: u32,
    pub chrome_edge: u32,
}
