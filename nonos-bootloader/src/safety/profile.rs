// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Build-profile gates. A binary carries one trust posture; these
//! make a mislabeled build fail at compile time instead of shipping.

// `standard` is the floor of every shipping profile, so this single
// gate keeps the F12 override out of standard, hardened and production.
#[cfg(all(feature = "standard", feature = "dev-mode"))]
compile_error!(
    "dev-mode compiles in the F12 verification override; not allowed in shipping profiles"
);

#[cfg(all(feature = "production", not(feature = "zk-transparent")))]
compile_error!("production requires the transparent proof backend");
