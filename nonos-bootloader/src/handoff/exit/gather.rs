// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::handoff_init::HandoffInitParams;
use crate::firmware::FirmwareHandoff;
use crate::handoff::config::{get_acpi_rsdp, get_framebuffer_info, get_smbios_entry};
use crate::handoff::prepare::{
    build_handoff_flags, detect_cpu_security_features, estimate_tsc_frequency, HandoffAllocations,
};
use crate::handoff::timing::get_uefi_time_epoch;
use crate::handoff::types::CryptoHandoff;
use crate::loader::KernelImage;
use uefi::prelude::*;
use uefi::table::boot::BootServices;

/// Collect all system info needed for kernel handoff before ExitBootServices.
pub fn gather_system_info(
    st: &SystemTable<Boot>,
    bs: &BootServices,
    kernel: &KernelImage,
    crypto: &CryptoHandoff,
    firmware: FirmwareHandoff,
    tpm_measured: bool,
    allocs: &HandoffAllocations,
    rng_seed: [u8; 32],
) -> HandoffInitParams {
    let fb_info = get_framebuffer_info(bs);
    // Paint the framebuffer the kernel will inherit onto the splash: on a
    // machine with no serial console, a photo of this line is the only way
    // to see what address and layout the firmware actually handed over.
    crate::display::log_panel::log_ok(
        alloc::format!(
            "fb 0x{:x} stride {} fmt {}",
            fb_info.ptr,
            fb_info.stride,
            fb_info.pixel_format
        )
        .as_bytes(),
    );
    let (smep, smap, umip) = detect_cpu_security_features();
    let acpi_rsdp = get_acpi_rsdp(st);
    HandoffInitParams {
        fb_info,
        acpi_rsdp,
        smbios_entry: get_smbios_entry(st),
        unix_epoch_ms: get_uefi_time_epoch(st),
        tsc_hz: estimate_tsc_frequency(bs),
        handoff_flags: build_handoff_flags(
            fb_info.ptr != 0,
            acpi_rsdp != 0,
            crypto,
            tpm_measured,
            smep,
            smap,
            umip,
        ),
        entry_point: kernel.entry_point as u64,
        cmdline_addr: allocs.cmdline_addr,
        crypto: *crypto,
        firmware,
        rng_seed,
    }
}
