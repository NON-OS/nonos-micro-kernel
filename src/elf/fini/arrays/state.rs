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

use crate::memory::addr::VirtAddr;

use super::info::FiniArrayInfo;

pub struct FiniArrayRunner {
    pub(super) fini_array: Option<FiniArrayInfo>,
    pub(super) fini_fn: Option<VirtAddr>,
}

impl FiniArrayRunner {
    pub fn new() -> Self { Self { fini_array: None, fini_fn: None } }
    pub fn with_fini_array(mut self, info: FiniArrayInfo) -> Self { self.fini_array = Some(info); self }
    pub fn with_fini_fn(mut self, addr: VirtAddr) -> Self { self.fini_fn = Some(addr); self }
}

impl Default for FiniArrayRunner {
    fn default() -> Self { Self::new() }
}
