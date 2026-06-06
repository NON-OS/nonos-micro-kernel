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
use crate::constants::{TRB_IDT, TRB_TYPE_SETUP_STAGE, TRT_IN_DATA, TRT_NO_DATA, TRT_OUT_DATA};
use crate::trb::Trb;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetupDir {
    NoData,
    HostToDevice,
    DeviceToHost,
}
pub fn setup_stage(
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
    dir: SetupDir,
    cycle: bool,
) -> Trb {
    let mut trb = Trb::zero();
    trb.d0 = (bm_request_type as u32) | ((b_request as u32) << 8) | ((w_value as u32) << 16);
    trb.d1 = (w_index as u32) | ((w_length as u32) << 16);
    trb.d2 = 8 | trt_bits(dir);
    trb.d3 = TRB_IDT;
    trb.set_type(TRB_TYPE_SETUP_STAGE);
    trb.set_cycle(cycle);
    trb
}
fn trt_bits(dir: SetupDir) -> u32 {
    match dir {
        SetupDir::NoData => TRT_NO_DATA,
        SetupDir::HostToDevice => TRT_OUT_DATA,
        SetupDir::DeviceToHost => TRT_IN_DATA,
    }
}
