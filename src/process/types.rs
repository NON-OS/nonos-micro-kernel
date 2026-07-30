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

use crate::arch::paging::descriptor::flags;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use super::core::ProcessControlBlock;

#[derive(Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub(crate) pcb: Option<Arc<ProcessControlBlock>>,
}

impl Process {
    #[inline]
    pub fn new(pid: u32, name: String, pcb: Option<Arc<ProcessControlBlock>>) -> Self {
        Self { pid, name, pcb }
    }

    #[inline]
    pub fn pid(&self) -> u32 {
        self.pid
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn serialize_state(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.name.len());
        out.extend_from_slice(&self.pid.to_le_bytes());
        out.extend_from_slice(self.name.as_bytes());
        out
    }

    pub fn command_line(&self) -> Option<String> {
        self.pcb.as_ref().and_then(|pcb| {
            let argv = pcb.argv.lock();
            if argv.is_empty() {
                None
            } else {
                Some(argv.join(" "))
            }
        })
    }

    pub fn environment_variables(&self) -> Option<Vec<(String, String)>> {
        self.pcb.as_ref().and_then(|pcb| {
            let envp = pcb.envp.lock();
            if envp.is_empty() {
                return None;
            }
            let mut v = Vec::with_capacity(envp.len());
            for e in envp.iter() {
                if let Some(eq) = e.find('=') {
                    v.push((String::from(&e[..eq]), String::from(&e[eq + 1..])));
                } else {
                    v.push((e.clone(), String::new()));
                }
            }
            Some(v)
        })
    }

    pub fn is_authorized_executable_region(&self, address: u64) -> bool {
        self.pcb.as_ref().map_or(false, |pcb| {
            let mem = pcb.memory.lock();
            if address >= mem.code_start.as_u64() && address < mem.code_end.as_u64() {
                return true;
            }
            for vma in &mem.vmas {
                if address >= vma.start.as_u64()
                    && address < vma.end.as_u64()
                    && vma.flags & flags::PRESENT != 0
                {
                    return true;
                }
            }
            false
        })
    }

    pub fn state(&self) -> super::core::ProcessState {
        self.pcb.as_ref().map_or(super::core::ProcessState::Ready, |pcb| pcb.state.lock().clone())
    }

    pub fn priority(&self) -> Option<super::core::types::Priority> {
        self.pcb.as_ref().map(|pcb| pcb.priority.lock().clone())
    }

    pub fn resident_memory_kb(&self) -> u64 {
        self.pcb.as_ref().map_or(0, |pcb| {
            let mem = pcb.memory.lock();
            mem.resident_pages.load(core::sync::atomic::Ordering::Relaxed) * 4
        })
    }
}
