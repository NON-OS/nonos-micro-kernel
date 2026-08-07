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
use super::tag::tag4;

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
pub(crate) const N_MK_STORE_WRITE: i64 = tag4(b"MSWR");
pub(crate) const N_MK_PCI_CONFIG_READ: i64 = tag4(b"MPCR");
pub(crate) const N_MK_PCI_CONFIG_WRITE: i64 = tag4(b"MPCW");
