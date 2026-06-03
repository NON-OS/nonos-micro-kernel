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

pub(crate) const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

pub(crate) const N_MK_IPC_SEND: i64 = tag4(b"MISD");
pub(crate) const N_MK_IPC_RECV: i64 = tag4(b"MIRC");
pub(crate) const N_MK_IPC_CALL: i64 = tag4(b"MICL");
pub(crate) const N_MK_IPC_RECV_FROM: i64 = tag4(b"MIRF");
pub(crate) const N_MK_IPC_REPLY: i64 = tag4(b"MIRY");
pub(crate) const N_MK_IPC_SEND_TO_PID: i64 = tag4(b"MISP");
pub(crate) const N_MK_SERVICE_LOOKUP: i64 = tag4(b"MSVL");
pub(crate) const N_MK_SERVICE_REGISTER: i64 = tag4(b"MSVR");
pub(crate) const N_MK_MMAP: i64 = tag4(b"MMAP");
pub(crate) const N_MK_EXIT: i64 = tag4(b"MEXT");
pub(crate) const N_MK_YIELD: i64 = tag4(b"MYLD");
pub(crate) const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");
pub(crate) const N_MK_TIME_RTC: i64 = tag4(b"MTRT");
pub(crate) const N_ADMIN_REBOOT: i64 = tag4(b"ARBT");
pub(crate) const N_ADMIN_SHUTDOWN: i64 = tag4(b"ASDN");
pub(crate) const N_ADMIN_POLICY_PUSH: i64 = tag4(b"APPS");
pub(crate) const N_CRYPTO_RANDOM: i64 = tag4(b"CRND");
pub(crate) const N_CRYPTO_HASH: i64 = tag4(b"CHSH");
pub(crate) const N_CRYPTO_ENCRYPT: i64 = tag4(b"CENC");
pub(crate) const N_CRYPTO_DECRYPT: i64 = tag4(b"CDEC");
pub(crate) const N_CRYPTO_ED25519_VERIFY: i64 = tag4(b"CEDV");
pub(crate) const N_CRYPTO_X25519_PUBLIC: i64 = tag4(b"CXPK");
pub(crate) const N_CRYPTO_X25519_SHARED: i64 = tag4(b"CXSH");
pub(crate) const N_CRYPTO_HMAC_SHA256: i64 = tag4(b"CHMC");
pub(crate) const N_CRYPTO_HKDF_SHA256: i64 = tag4(b"CHKF");
pub(crate) const N_GFX_DISPLAY_DIMENSIONS: i64 = tag4(b"GDIM");
pub(crate) const N_GFX_SURFACE_CREATE: i64 = tag4(b"GSCR");
pub(crate) const N_GFX_SURFACE_DESTROY: i64 = tag4(b"GSDS");
pub(crate) const N_GFX_SURFACE_MAP: i64 = tag4(b"GSMP");
pub(crate) const N_GFX_SURFACE_PRESENT_FULL: i64 = tag4(b"GPRF");
pub(crate) const N_GFX_SURFACE_PRESENT_RECT: i64 = tag4(b"GPRR");
pub(crate) const N_GFX_DISPLAY_LIST: i64 = tag4(b"GDLS");
pub(crate) const N_GFX_CURSOR_PRESENT: i64 = tag4(b"GCUR");
pub(crate) const N_MK_DEVICE_LIST: i64 = tag4(b"MDLS");
pub(crate) const N_MK_DEVICE_CLAIM: i64 = tag4(b"MDCL");
pub(crate) const N_MK_DEVICE_RELEASE: i64 = tag4(b"MDRL");
pub(crate) const N_MK_MMIO_MAP: i64 = tag4(b"MMMP");
pub(crate) const N_MK_MMIO_UNMAP: i64 = tag4(b"MMUM");
pub(crate) const N_MK_IRQ_BIND: i64 = tag4(b"MIRB");
pub(crate) const N_MK_IRQ_UNBIND: i64 = tag4(b"MIRU");
pub(crate) const N_MK_IRQ_ACK: i64 = tag4(b"MIRA");
pub(crate) const N_MK_IRQ_POLL: i64 = tag4(b"MIRP");
pub(crate) const N_MK_IRQ_WAIT: i64 = tag4(b"MIRW");
pub(crate) const N_MK_DMA_MAP: i64 = tag4(b"MDMM");
pub(crate) const N_MK_DMA_UNMAP: i64 = tag4(b"MDMU");
pub(crate) const N_MK_PIO_GRANT: i64 = tag4(b"MPGT");
pub(crate) const N_MK_PIO_READ: i64 = tag4(b"MPRD");
pub(crate) const N_MK_PIO_WRITE: i64 = tag4(b"MPWR");
pub(crate) const N_MK_PIO_RELEASE: i64 = tag4(b"MPRL");
pub(crate) const N_MK_DEBUG: i64 = tag4(b"MDBG");
pub(crate) const N_MK_PCI_CONFIG_READ: i64 = tag4(b"MPCR");
pub(crate) const N_MK_PCI_CONFIG_WRITE: i64 = tag4(b"MPCW");
pub(crate) const N_MK_SURFACE_REGISTER: i64 = tag4(b"MSRG");
pub(crate) const N_MK_SURFACE_SHARE: i64 = tag4(b"MSSH");
pub(crate) const N_MK_SURFACE_ATTACH: i64 = tag4(b"MSAT");
pub(crate) const N_MK_SURFACE_RELEASE: i64 = tag4(b"MSRL");
pub(crate) const N_MK_SURFACE_PRESENT: i64 = tag4(b"MSPR");
pub(crate) const N_MK_DISPLAY_VSYNC_WAIT: i64 = tag4(b"MDVW");
pub(crate) const N_MK_INPUT_EVENT_POST: i64 = tag4(b"MIEP");
pub(crate) const N_MK_INPUT_EVENT_DRAIN: i64 = tag4(b"MIED");
pub(crate) const N_MK_INPUT_EVENT_WAIT: i64 = tag4(b"MIEW");
