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

use super::memory::calculate_brk;
use crate::elf::loader::ElfImage;
use crate::elf::stack::StackLayout;
use crate::elf::tls::TlsInfo;
use crate::memory::addr::VirtAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug)]
pub struct ProcessImage {
    pub executable: ElfImage,
    pub interpreter: Option<ElfImage>,
    pub stack: StackLayout,
    pub entry_point: VirtAddr,
    pub initial_sp: VirtAddr,
    pub brk_start: VirtAddr,
    pub brk_current: VirtAddr,
    pub tls: Option<TlsInfo>,
    pub state: ProcessState,
}

impl ProcessImage {
    pub fn new(executable: ElfImage, interpreter: Option<ElfImage>, stack: StackLayout) -> Self {
        let entry_point = interpreter.as_ref().map_or(executable.entry_point, |image| image.entry_point);
        let brk_start = calculate_brk(&executable);
        Self {
            executable,
            interpreter,
            entry_point,
            initial_sp: stack.stack_pointer,
            brk_start,
            brk_current: brk_start,
            tls: None,
            state: ProcessState::Created,
            stack,
        }
    }
}
