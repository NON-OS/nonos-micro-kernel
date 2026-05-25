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

extern crate alloc;

use super::{DynamicInfo, LoadedSegment};
use crate::elf::dynlink::DynLinkInfo;
use crate::elf::tls::TlsInfo;
use crate::memory::addr::VirtAddr;
use alloc::{string::String, vec::Vec};

#[derive(Debug)]
pub struct ElfImage {
    pub base_addr: VirtAddr,
    pub entry_point: VirtAddr,
    pub size: usize,
    pub memory_size: usize,
    pub segments: Vec<LoadedSegment>,
    pub dynamic_info: Option<DynamicInfo>,
    pub dynlink_info: Option<DynLinkInfo>,
    pub tls_info: Option<TlsInfo>,
    pub interpreter: Option<String>,
}

impl ElfImage {
    pub fn is_pie(&self) -> bool {
        self.interpreter.is_some() || self.dynamic_info.is_some()
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn has_dynamic_info(&self) -> bool {
        self.dynamic_info.is_some()
    }

    pub fn has_tls(&self) -> bool {
        self.tls_info.is_some()
    }

    pub fn requires_interpreter(&self) -> bool {
        self.interpreter.is_some()
    }

    pub fn memory_footprint(&self) -> usize {
        self.segments.iter().map(|segment| segment.size).sum()
    }
}
