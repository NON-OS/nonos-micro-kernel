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

//! Price model. The capsule never moves money; it only carries the
//! shape so the future capsule_payment knows what to charge for.

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PriceKind {
    Free = 0,
    OneTime = 1,
    Subscription = 2,
    UsageMetered = 3,
}

impl PriceKind {
    pub fn from_u8(b: u8) -> Self {
        match b {
            1 => Self::OneTime,
            2 => Self::Subscription,
            3 => Self::UsageMetered,
            _ => Self::Free,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PriceModel {
    pub kind: PriceKind,
    /// Price expressed in the smallest unit of the carrying token
    /// (e.g. wei for ETH, atto-NOX for NOX). The capsule does not
    /// assume USD or fiat conversion.
    pub amount_atomic: u128,
    /// Period in seconds for `Subscription` and `UsageMetered`;
    /// 0 for `Free` and `OneTime`.
    pub period_seconds: u64,
}
