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

pub mod mac;
pub mod rx;
pub mod tx;

use alloc::vec::Vec;
use smoltcp::phy::{Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;

pub struct NicDevice { pub port: u32 }
pub struct NicRxToken(Vec<u8>);
pub struct NicTxToken { pub port: u32 }

impl smoltcp::phy::RxToken for NicRxToken {
    fn consume<R, F>(self, f: F) -> R
    where F: FnOnce(&mut [u8]) -> R {
        let mut frame = self.0;
        f(&mut frame)
    }
}

impl smoltcp::phy::TxToken for NicTxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where F: FnOnce(&mut [u8]) -> R {
        let mut buf = alloc::vec![0u8; len];
        let r = f(&mut buf);
        tx::send_frame(self.port, &buf);
        r
    }
}

impl Device for NicDevice {
    type RxToken<'a> = NicRxToken where Self: 'a;
    type TxToken<'a> = NicTxToken where Self: 'a;

    fn receive(&mut self, _now: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let frame = rx::poll_frame(self.port)?;
        Some((NicRxToken(frame), NicTxToken { port: self.port }))
    }

    fn transmit(&mut self, _now: Instant) -> Option<Self::TxToken<'_>> {
        Some(NicTxToken { port: self.port })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut c = DeviceCapabilities::default();
        c.max_transmission_unit = 1514;
        c.medium = Medium::Ethernet;
        c
    }
}

pub fn mac(port: u32) -> [u8; 6] {
    mac::read_mac(port).unwrap_or([0u8; 6])
}
