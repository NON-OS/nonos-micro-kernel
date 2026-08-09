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

use qrcode::{Color, QrCode};

pub const PAYLOAD: &str = "https://nonos.systems";

pub struct Matrix {
    pub width: usize,
    pub dark: Vec<bool>,
}

impl Matrix {
    pub fn is_dark(&self, x: usize, y: usize) -> bool {
        self.dark.get(y * self.width + x).copied().unwrap_or(false)
    }
}

pub fn encode() -> Option<Matrix> {
    let code = QrCode::new(PAYLOAD.as_bytes()).ok()?;
    let width = code.width();
    let dark = code.to_colors().into_iter().map(|c| c == Color::Dark).collect();
    Some(Matrix { width, dark })
}
