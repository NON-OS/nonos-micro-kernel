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

//! A from-scratch QR code encoder (ISO/IEC 18004), byte mode, versions 1-10.
//! It produces a scannable module matrix for a wallet address or payment URI
//! with no external dependencies, `no_std` so it runs inside the capsule.

#![no_std]

extern crate alloc;

mod encode;
mod format;
mod gf256;
mod mask;
mod matrix;
mod reed_solomon;
mod version;

use matrix::Matrix;

pub use version::Ecc;

/// A finished QR code: an `n`-by-`n` grid of light/dark modules, row-major,
/// `true` where a module is dark.
pub struct QrCode {
    pub size: usize,
    pub modules: alloc::vec::Vec<bool>,
}

impl QrCode {
    /// True where the module at (x, y) is dark.
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.modules[y * self.size + x]
    }
}

/// Encode `data` at the given error-correction level, choosing the smallest
/// version that fits and the mask with the lowest penalty. Returns None when
/// the data exceeds the version-10 byte-mode capacity.
pub fn encode(data: &[u8], ecc: Ecc) -> Option<QrCode> {
    let version = encode::choose_version(data.len(), ecc)?;
    let codewords = encode::message_codewords(data, version, ecc);

    // Lay the data once, then try every mask on a fresh copy and keep the best.
    let mut base = Matrix::new(version);
    base.place_data(&codewords);

    let mut best: Option<(u32, Matrix)> = None;
    for m in 0..8u8 {
        let mut cand = clone_matrix(&base);
        mask::apply(&mut cand, m);
        format::write_format(&mut cand, ecc, m);
        format::write_version(&mut cand, version);
        let score = mask::penalty(&cand);
        if best.as_ref().map(|(s, _)| score < *s).unwrap_or(true) {
            best = Some((score, cand));
        }
    }

    let chosen = best?.1;
    Some(QrCode { size: chosen.n, modules: chosen.modules })
}

fn clone_matrix(m: &Matrix) -> Matrix {
    Matrix { n: m.n, modules: m.modules.clone(), function: m.function.clone() }
}
