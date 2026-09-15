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

//! The one file that is not the shipping driver: the register accessor,
//! with the same shape as capsule_driver_i2c_pci/src/regs.rs and every access
//! forwarded to the core the test attached on this thread.

use std::cell::RefCell;

use nonos_i2cmodel::Designware;

thread_local! {
    static CORE: RefCell<Option<Designware>> = const { RefCell::new(None) };
}

/// Detaches the core when dropped, so a later test on the same thread finds
/// no controller until it attaches one.
pub struct Attached;

impl Drop for Attached {
    fn drop(&mut self) {
        CORE.with(|c| *c.borrow_mut() = None);
    }
}

pub fn attach(core: Designware) -> Attached {
    CORE.with(|c| *c.borrow_mut() = Some(core));
    Attached
}

/// Look at the attached core. Panics when none is, which is a test bug.
pub fn with<R>(f: impl FnOnce(&mut Designware) -> R) -> R {
    CORE.with(|c| f(c.borrow_mut().as_mut().expect("no core attached on this thread")))
}

#[derive(Clone, Copy)]
pub struct Regs {
    _base: u64,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { _base: base }
    }

    pub fn read32(&self, offset: u64) -> u32 {
        with(|core| core.read32(offset))
    }

    pub fn write32(&self, offset: u64, value: u32) {
        with(|core| core.write32(offset, value))
    }
}
