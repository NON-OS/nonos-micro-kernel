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

use super::{compose_verb, compose_verb_long, verb, OutputPath};
use crate::constants::{
    AMP_OUT_UNMUTE, PIN_OUT_ENABLE, POWER_D0, STREAM_FMT_48K16S, VERB_SET_AMP_GAIN_MUTE,
    VERB_SET_CHANNEL_STREAMID, VERB_SET_PIN_WIDGET_CONTROL, VERB_SET_POWER_STATE,
    VERB_SET_STREAM_FORMAT,
};
use crate::error::HdaResult;
use crate::regs::Regs;

const AMP_GAIN: u16 = 0x7f;

fn send(regs: Regs, cv: u64, rv: u64, wp: &mut u16, cmd: u32) -> HdaResult<()> {
    verb::send(regs, cv, rv, wp, cmd)?;
    Ok(())
}

pub(crate) fn configure(
    regs: Regs,
    cv: u64,
    rv: u64,
    wp: &mut u16,
    cad: u8,
    path: OutputPath,
    tag: u8,
) -> HdaResult<()> {
    let power = |nid: u8| compose_verb(cad, nid, VERB_SET_POWER_STATE, POWER_D0 as u16);
    send(regs, cv, rv, wp, power(path.afg_nid))?;
    send(regs, cv, rv, wp, power(path.dac_nid))?;
    send(regs, cv, rv, wp, power(path.pin_nid))?;
    let pinctl = compose_verb(cad, path.pin_nid, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16);
    send(regs, cv, rv, wp, pinctl)?;
    let amp = compose_verb_long(cad, path.dac_nid, VERB_SET_AMP_GAIN_MUTE as u16, AMP_OUT_UNMUTE | AMP_GAIN);
    send(regs, cv, rv, wp, amp)?;
    let fmt = compose_verb_long(cad, path.dac_nid, VERB_SET_STREAM_FORMAT as u16, STREAM_FMT_48K16S);
    send(regs, cv, rv, wp, fmt)?;
    let sid = compose_verb(cad, path.dac_nid, VERB_SET_CHANNEL_STREAMID, (tag as u16) << 4);
    send(regs, cv, rv, wp, sid)
}
