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
#[derive(Debug, Clone, Copy)]
pub struct ControllerLayout {
    pub op_base: u64,
    pub doorbell_base: u64,
    pub primary_intr_base: u64,
    pub max_slots: u8,
    pub max_ports: u8,
    pub max_scratchpad: u32,
    pub context_size: u8,
}