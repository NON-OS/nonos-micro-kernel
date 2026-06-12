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

use crate::firmware::types::{FirmwareEntry, FirmwareHandoff, FirmwareType, MAX_FIRMWARE_ENTRIES};

static INTEL_AX200: &[u8] = include_bytes!("../../firmware/intel/iwlwifi-cc-a0-77.ucode");
static INTEL_AX210: &[u8] = include_bytes!("../../firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");
static INTEL_8265: &[u8] = include_bytes!("../../firmware/intel/iwlwifi-8265-36.ucode");
static INTEL_9260: &[u8] = include_bytes!("../../firmware/intel/iwlwifi-9260-th-b0-jf-b0-46.ucode");
static INTEL_7265: &[u8] = include_bytes!("../../firmware/intel/iwlwifi-7265D-29.ucode");
static RTL8821C: &[u8] = include_bytes!("../../firmware/realtek/rtw8821c_fw.bin");
static RTL8822B: &[u8] = include_bytes!("../../firmware/realtek/rtw8822b_fw.bin");
static RTL8822C: &[u8] = include_bytes!("../../firmware/realtek/rtw8822c_fw.bin");
static RTL8723D: &[u8] = include_bytes!("../../firmware/realtek/rtw8723d_fw.bin");
static RTL8851B: &[u8] = include_bytes!("../../firmware/realtek/rtw8851b_fw.bin");
static RTL8852A: &[u8] = include_bytes!("../../firmware/realtek/rtw8852a_fw.bin");
static RTL8852B: &[u8] = include_bytes!("../../firmware/realtek/rtw8852b_fw.bin");
static RTL8852C: &[u8] = include_bytes!("../../firmware/realtek/rtw8852c_fw.bin");

const FIRMWARE_TABLE: &[(FirmwareType, &[u8])] = &[
    (FirmwareType::IntelAx200, INTEL_AX200),
    (FirmwareType::IntelAx210, INTEL_AX210),
    (FirmwareType::Intel8265, INTEL_8265),
    (FirmwareType::Intel9260, INTEL_9260),
    (FirmwareType::Intel7265, INTEL_7265),
    (FirmwareType::Rtl8821c, RTL8821C),
    (FirmwareType::Rtl8822b, RTL8822B),
    (FirmwareType::Rtl8822c, RTL8822C),
    (FirmwareType::Rtl8723d, RTL8723D),
    (FirmwareType::Rtl8851b, RTL8851B),
    (FirmwareType::Rtl8852a, RTL8852A),
    (FirmwareType::Rtl8852b, RTL8852B),
    (FirmwareType::Rtl8852c, RTL8852C),
];

pub fn get_firmware_handoff() -> FirmwareHandoff {
    let mut handoff = FirmwareHandoff::new();
    for (fw_type, data) in FIRMWARE_TABLE {
        if handoff.count < MAX_FIRMWARE_ENTRIES {
            handoff.entries[handoff.count] = FirmwareEntry {
                fw_type: *fw_type,
                ptr: data.as_ptr() as u64,
                size: data.len() as u32,
                reserved: 0,
            };
            handoff.count += 1;
        }
    }
    handoff
}

pub fn get_firmware(fw_type: FirmwareType) -> Option<&'static [u8]> {
    for (t, data) in FIRMWARE_TABLE {
        if *t == fw_type {
            return Some(*data);
        }
    }
    None
}
pub fn has_embedded_firmware() -> bool {
    true
}
pub fn firmware_count() -> usize {
    FIRMWARE_TABLE.len()
}
