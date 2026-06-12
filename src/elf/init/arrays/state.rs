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

use super::info::{InitArrayInfo, PreInitArrayInfo};

pub struct InitArrayRunner {
    pub(super) preinit_array: Option<PreInitArrayInfo>,
    pub(super) init_fn: Option<VirtAddr>,
    pub(super) init_array: Option<InitArrayInfo>,
}

impl InitArrayRunner {
    pub fn new() -> Self {
        Self { preinit_array: None, init_fn: None, init_array: None }
    }
    pub fn with_preinit_array(mut self, info: PreInitArrayInfo) -> Self {
        self.preinit_array = Some(info);
        self
    }
    pub fn with_init_fn(mut self, addr: VirtAddr) -> Self {
        self.init_fn = Some(addr);
        self
    }
    pub fn with_init_array(mut self, info: InitArrayInfo) -> Self {
        self.init_array = Some(info);
        self
    }
}

impl Default for InitArrayRunner {
    fn default() -> Self {
        Self::new()
    }
}
