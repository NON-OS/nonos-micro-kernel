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

//! The machine state: a fixed register file over the field, and the bounds-checked
//! reads and writes the run and step code go through. Keeping the register access
//! next to the state that owns it is what lets the register array stay private.

use nonos_stark::field::Fp;

use super::ProveError;
use crate::isa::REGS;

/// The machine: a fixed register file over the field. Every value it holds is an
/// `Fp`, the same scalar the STARK commits, so a run and its proof share one field.
pub struct Vm {
    regs: [Fp; REGS],
}

impl Default for Vm {
    fn default() -> Vm {
        Vm::new()
    }
}

impl Vm {
    /// A fresh machine with every register zeroed.
    pub fn new() -> Vm {
        Vm { regs: [Fp::ZERO; REGS] }
    }

    /// Read a register, bounds-checked. An out-of-range index is a typed error
    /// rather than a panic, so a malformed program cannot crash the executor.
    pub(super) fn rget(&self, idx: u8) -> Result<Fp, ProveError> {
        self.regs.get(idx as usize).copied().ok_or(ProveError::BadRegister(idx))
    }

    /// Write a register, bounds-checked.
    pub(super) fn wset(&mut self, idx: u8, v: Fp) -> Result<(), ProveError> {
        match self.regs.get_mut(idx as usize) {
            Some(slot) => {
                *slot = v;
                Ok(())
            }
            None => Err(ProveError::BadRegister(idx)),
        }
    }
}
