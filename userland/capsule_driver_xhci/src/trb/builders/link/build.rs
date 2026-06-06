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
use super::state::LinkTrbBuilderState;
use crate::trb::base::Trb;
pub struct LinkTrbBuilder {
    inner: LinkTrbBuilderState,
}
impl LinkTrbBuilder {
    pub fn new() -> Self {
        Self { inner: LinkTrbBuilderState::new() }
    }
    pub fn target(self, phys_addr: u64) -> Self {
        Self { inner: self.inner.target(phys_addr) }
    }
    pub fn toggle_cycle(self, toggle: bool) -> Self {
        Self { inner: self.inner.toggle_cycle(toggle) }
    }
    pub fn cycle(self, cycle: bool) -> Self {
        Self { inner: self.inner.cycle(cycle) }
    }
    pub fn build(self) -> Trb {
        self.inner.trb
    }
}
