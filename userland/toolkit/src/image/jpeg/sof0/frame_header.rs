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
use super::component::Component;
use super::constants::MAX_COMPS;

#[derive(Clone, Copy)]
pub struct FrameHeader {
    pub precision: u8,
    pub width: u16,
    pub height: u16,
    pub num_comps: u8,
    pub comps: [Component; MAX_COMPS],
    pub h_max: u8,
    pub v_max: u8,
}
