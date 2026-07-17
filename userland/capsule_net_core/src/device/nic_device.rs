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

use smoltcp::phy::{Device, DeviceCapabilities};
use smoltcp::time::Instant;

use crate::device::types::{NicDevice, NicRxToken, NicTxToken};
use crate::device::{capabilities, receive, transmit};

impl Device for NicDevice {
    type RxToken<'a>
        = NicRxToken
    where
        Self: 'a;
    type TxToken<'a>
        = NicTxToken
    where
        Self: 'a;

    fn receive(&mut self, _now: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        receive::receive(self.port)
    }

    fn transmit(&mut self, _now: Instant) -> Option<Self::TxToken<'_>> {
        transmit::transmit(self.port)
    }

    fn capabilities(&self) -> DeviceCapabilities {
        capabilities::capabilities()
    }
}
