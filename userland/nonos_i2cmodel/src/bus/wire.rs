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

//! The bus itself: which target is addressed, and the trace.

use super::target::{Dir, Target};
use super::trace::BusEvent;

#[derive(Default)]
pub struct Bus {
    targets: Vec<Box<dyn Target>>,
    addressed: Option<usize>,
    trace: Vec<BusEvent>,
}

impl Bus {
    pub fn with(mut self, target: impl Target + 'static) -> Self {
        self.targets.push(Box::new(target));
        self
    }

    pub fn trace(&self) -> &[BusEvent] {
        &self.trace
    }

    /// A START (or repeated START) to `addr`. True when a target answered.
    pub fn start(&mut self, addr: u8, dir: Dir, repeated: bool) -> bool {
        self.addressed = self.targets.iter().position(|t| t.address() == addr);
        let acked = self.addressed.is_some();
        if let Some(i) = self.addressed {
            self.targets[i].start(dir, repeated);
        }
        self.trace.push(match repeated {
            true => BusEvent::Restart { addr, dir, acked },
            false => BusEvent::Start { addr, dir, acked },
        });
        acked
    }

    pub fn write(&mut self, byte: u8) -> bool {
        let acked = self.addressed.is_some_and(|i| self.targets[i].write(byte));
        self.trace.push(BusEvent::Write { byte, acked });
        acked
    }

    /// A byte from the addressed target; all ones when nothing is addressed,
    /// which is what a released SDA line reads as.
    pub fn read(&mut self, last: bool) -> u8 {
        let byte = self.addressed.map_or(0xFF, |i| self.targets[i].read(last));
        self.trace.push(BusEvent::Read { byte, last });
        byte
    }

    pub fn stop(&mut self) {
        if let Some(i) = self.addressed.take() {
            self.targets[i].stop();
        }
        self.trace.push(BusEvent::Stop);
    }
}
