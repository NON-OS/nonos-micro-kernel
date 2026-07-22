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

use super::{compose_verb, verb};
use crate::constants::{
    FUNCTION_GROUP_AUDIO, PARAM_AUDIO_WIDGET_CAP, PARAM_FUNCTION_GROUP_TYPE, PARAM_SUBNODE_COUNT,
    VERB_GET_PARAMETER, WIDGET_TYPE_DAC, WIDGET_TYPE_PIN,
};
use crate::regs::Regs;

const WALK_CAP: u8 = 64;

#[derive(Clone, Copy)]
pub(crate) struct OutputPath {
    pub(crate) dac_nid: u8,
    pub(crate) pin_nid: u8,
}

fn get_param(regs: Regs, cv: u64, rv: u64, wp: &mut u16, cad: u8, nid: u8, param: u16) -> Option<u32> {
    verb::send(regs, cv, rv, wp, compose_verb(cad, nid, VERB_GET_PARAMETER, param)).ok()
}

fn subnodes(regs: Regs, cv: u64, rv: u64, wp: &mut u16, cad: u8, nid: u8) -> Option<(u8, u8)> {
    let r = get_param(regs, cv, rv, wp, cad, nid, PARAM_SUBNODE_COUNT)?;
    Some((((r >> 16) & 0xff) as u8, (r & 0xff) as u8))
}

fn find_afg(regs: Regs, cv: u64, rv: u64, wp: &mut u16, cad: u8, start: u8, count: u8) -> Option<u8> {
    let mut i = 0u8;
    while i < count && i < WALK_CAP {
        let nid = start.wrapping_add(i);
        let t = get_param(regs, cv, rv, wp, cad, nid, PARAM_FUNCTION_GROUP_TYPE)?;
        if t & 0xff == FUNCTION_GROUP_AUDIO {
            return Some(nid);
        }
        i = i.wrapping_add(1);
    }
    None
}

pub(crate) fn find_output(regs: Regs, cv: u64, rv: u64, wp: &mut u16, cad: u8) -> Option<OutputPath> {
    let (fg_start, fg_count) = subnodes(regs, cv, rv, wp, cad, 0)?;
    let afg = find_afg(regs, cv, rv, wp, cad, fg_start, fg_count)?;
    let (w_start, w_count) = subnodes(regs, cv, rv, wp, cad, afg)?;
    let (mut dac, mut pin) = (None, None);
    let mut i = 0u8;
    while i < w_count && i < WALK_CAP {
        let nid = w_start.wrapping_add(i);
        let cap = get_param(regs, cv, rv, wp, cad, nid, PARAM_AUDIO_WIDGET_CAP)?;
        let ty = (cap >> 20) & 0xf;
        if ty == WIDGET_TYPE_DAC && dac.is_none() {
            dac = Some(nid);
        }
        if ty == WIDGET_TYPE_PIN && pin.is_none() {
            pin = Some(nid);
        }
        i = i.wrapping_add(1);
    }
    Some(OutputPath { dac_nid: dac?, pin_nid: pin? })
}
