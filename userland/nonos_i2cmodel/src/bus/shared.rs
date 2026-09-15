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

//! A target the test keeps a handle on after the bus has taken it.
//!
//! The bus owns its targets, and a proof still has to ask the device what it
//! received once the driver is done with it. `Shared` puts the device behind
//! a mutex the bus and the test both hold.

use std::sync::{Arc, Mutex, MutexGuard};

use super::target::{Dir, Target};

pub struct Shared<T>(Arc<Mutex<T>>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Shared<T> {
    pub fn new(target: T) -> Self {
        Self(Arc::new(Mutex::new(target)))
    }

    /// The device, for the test to inspect or prime. A poisoned lock means a
    /// test already failed inside it, and its state is still worth reading.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl<T: Target> Target for Shared<T> {
    fn address(&self) -> u8 {
        self.lock().address()
    }
    fn start(&mut self, dir: Dir, repeated: bool) {
        self.lock().start(dir, repeated)
    }
    fn write(&mut self, byte: u8) -> bool {
        self.lock().write(byte)
    }
    fn read(&mut self, last: bool) -> u8 {
        self.lock().read(last)
    }
    fn stop(&mut self) {
        self.lock().stop()
    }
}
