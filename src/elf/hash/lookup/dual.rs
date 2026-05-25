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

use super::super::{gnu::GnuHashTable, sysv::SysvHashTable};

pub struct DualHashLookup {
    gnu: Option<GnuHashTable>,
    sysv: Option<SysvHashTable>,
}

impl DualHashLookup {
    pub fn new() -> Self { Self { gnu: None, sysv: None } }
    pub fn with_gnu(mut self, table: GnuHashTable) -> Self { self.gnu = Some(table); self }
    pub fn with_sysv(mut self, table: SysvHashTable) -> Self { self.sysv = Some(table); self }

    pub fn lookup(&self, name: &str) -> Option<usize> {
        if let Some(gnu) = &self.gnu {
            if let Some(idx) = gnu.lookup(name) {
                return Some(idx);
            }
        }
        self.sysv.as_ref().and_then(|table| table.lookup(name))
    }

    pub fn has_gnu(&self) -> bool { self.gnu.is_some() }
    pub fn has_sysv(&self) -> bool { self.sysv.is_some() }
}

impl Default for DualHashLookup {
    fn default() -> Self { Self::new() }
}
