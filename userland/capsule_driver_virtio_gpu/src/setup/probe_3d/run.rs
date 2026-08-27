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
use super::backing;
use super::ids::{PROBE_RESOURCE_ID, RENDER_CTX_ID};
use super::log::{log, log_caps, log_err};
use super::render::{clear_target, create_target};
use super::teardown::release;
use super::verify::readback;
use crate::device::cmd::{ctx_attach_resource, ctx_create, get_capset_info};
use crate::device::ControlQueue;
use crate::state::FenceCounter;

pub fn probe(
    q: &ControlQueue,
    fences: &FenceCounter,
    virgl: bool,
    device_id: u64,
    claim_epoch: u64,
) -> bool {
    if !virgl {
        log(b"[GPU-3D] virgl not offered (2d scanout only)");
        return false;
    }
    let caps = match get_capset_info(q, 0) {
        Ok(c) => c,
        Err(e) => {
            log_err(b"[GPU-3D] capset query failed: ", e);
            return false;
        }
    };
    if let Err(e) = ctx_create(q, RENDER_CTX_ID, b"nonos-render") {
        log_err(b"[GPU-3D] context create failed: ", e);
        return false;
    }
    match render(q, fences, device_id, claim_epoch) {
        Ok(()) => {
            log_caps(caps.capset_id, caps.max_version, caps.max_size);
            true
        }
        Err(e) => {
            log_err(b"[GPU-3D] render probe failed: ", e);
            false
        }
    }
}

fn render(
    q: &ControlQueue,
    fences: &FenceCounter,
    device_id: u64,
    claim_epoch: u64,
) -> Result<(), &'static str> {
    create_target(q, fences)?;
    let region = backing::map(q, fences, device_id, claim_epoch)?;
    ctx_attach_resource(q, RENDER_CTX_ID, PROBE_RESOURCE_ID)?;
    clear_target(q, fences)?;
    let verdict = readback(q, fences, region.user_va);
    // Released whether or not the pixels matched: a failed probe that leaks
    // the resource makes every later run fail on the id instead.
    let released = release(q, fences);
    verdict.and(released)
}
