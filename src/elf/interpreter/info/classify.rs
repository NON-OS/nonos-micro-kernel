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

use super::{
    constants::{known_interp, NONOS_INTERP},
    state::InterpreterInfo,
};

impl InterpreterInfo {
    pub fn as_str(&self) -> &str {
        &self.path
    }
    pub fn is_nonos_ld(&self) -> bool {
        self.path == NONOS_INTERP || self.path.contains("ld-nonos")
    }
    pub fn is_glibc_ld(&self) -> bool {
        self.path.contains(known_interp::GLIBC_LD)
    }
    pub fn is_musl_ld(&self) -> bool {
        self.path.contains(known_interp::MUSL_LD)
    }

    pub fn is_known_ld(&self) -> bool {
        self.is_nonos_ld()
            || self.is_glibc_ld()
            || self.is_musl_ld()
            || self.path.contains(known_interp::GENERIC_LD)
    }

    pub fn is_foreign(&self) -> bool {
        !self.is_nonos_ld() && self.is_known_ld()
    }
    pub fn filename(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
    }
    pub fn is_absolute(&self) -> bool {
        self.path.starts_with('/')
    }
    pub fn directory(&self) -> &str {
        self.path.rfind('/').map_or("", |idx| &self.path[..idx])
    }
}
