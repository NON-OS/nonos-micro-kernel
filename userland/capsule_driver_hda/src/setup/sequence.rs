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

use nonos_libc::mk_device_release;

use super::mark::{mark, mark_corb_vid, mark_path};
use super::{claim, dma, irq, mmio, pci};
use crate::audio;
use crate::constants::{PARAM_VENDOR_ID, STREAM_TAG, VERB_GET_PARAMETER};
use crate::controller::{
    compose_verb, corb, graph, layout, leave_reset, probe, stream_run, stream_setup, verb,
    ControllerInfo, StreamDescriptor, StreamRun, STREAM_OUTPUT,
};
use crate::discover::find_hda;
use crate::error::{HdaError, HdaResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;
use crate::setup::Driver;

pub fn run() -> HdaResult<Driver> {
    let dev = find_hda().ok_or(HdaError::DeviceNotFound)?;
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_bus_master(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio::map(dev.device_id, claim_epoch, dev.bar_size)?;
    let irq = irq::bind(dev, claim_epoch, &mmio)?;
    let (corb, rirb) = dma::map_verb_rings(dev.device_id, claim_epoch, &mmio, &irq)?;
    let handles = BrokerHandles::new(
        dev.device_id,
        mmio.grant_id,
        mmio.user_va,
        irq.grant_id,
        corb.grant_id,
        rirb.grant_id,
    );
    let regs = Regs::new(handles.mmio_user_va());

    leave_reset(regs)?;
    let info = ControllerInfo::read(regs);
    if info.vmaj == 0 || info.gcap == 0 {
        return Err(HdaError::UnsupportedController);
    }
    corb::init(regs, corb.device_addr, rirb.device_addr)?;
    let cad = first_codec(info.statests);
    let mut wp = 0u16;
    let cmd = compose_verb(cad, 0, VERB_GET_PARAMETER, PARAM_VENDOR_ID);
    let resp = verb::send(regs, corb.user_va, rirb.user_va, &mut wp, cmd)?;
    mark_corb_vid((resp >> 16) as u16);
    let path = graph::find_output(regs, corb.user_va, rirb.user_va, &mut wp, cad)
        .ok_or(HdaError::UnsupportedController)?;
    mark_path(cad, path.dac_nid, path.pin_nid);
    let codecs = probe(regs, info.statests);
    mark("[HDA] up\n");
    let (bdl, sample) = dma::map_stream(
        dev.device_id,
        claim_epoch,
        &mmio,
        &irq,
        &[corb.grant_id, rirb.grant_id],
    )?;
    audio::fill(sample.user_va, crate::controller::bdl::RING_BYTES as usize);
    stream_setup::configure(regs, corb.user_va, rirb.user_va, &mut wp, cad, path, STREAM_TAG)?;
    let out = output_descriptor(info)?;
    stream_run::run(
        regs,
        StreamRun {
            desc: out,
            tag: STREAM_TAG,
            bdl_va: bdl.user_va,
            bdl_dev: bdl.device_addr,
            sample_dev: sample.device_addr,
            bytes: crate::controller::bdl::RING_BYTES as u32,
        },
    );
    mark("[HDA] stream-run\n");
    Ok(Driver {
        handles,
        regs,
        codecs,
        corb,
        rirb,
        path,
        bdl,
        sample,
        stream_off: out.mmio_offset,
        stream_tag: STREAM_TAG,
        stream_gi: out.global_index,
    })
}

fn output_descriptor(info: ControllerInfo) -> HdaResult<StreamDescriptor> {
    let (descs, n) = layout(info);
    let mut i = 0usize;
    while i < n {
        if descs[i].kind == STREAM_OUTPUT {
            return Ok(descs[i]);
        }
        i += 1;
    }
    Err(HdaError::UnsupportedController)
}

fn first_codec(statests: u16) -> u8 {
    let mut a = 0u8;
    while a < 15 {
        if (statests >> a) & 1 != 0 {
            return a;
        }
        a = a.wrapping_add(1);
    }
    0
}
