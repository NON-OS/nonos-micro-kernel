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

//! Diagnostics for a rejected flag, phrased the way the shell reports them.

use alloc::vec::Vec;

use super::spec::Spec;

pub(super) fn unknown(spec: &Spec, flag: u8) -> Vec<u8> {
    let mut msg = Vec::new();
    msg.extend_from_slice(spec.name);
    msg.extend_from_slice(b": unknown flag -");
    msg.push(flag);
    msg
}

pub(super) fn missing(spec: &Spec, arg: &[u8]) -> Vec<u8> {
    let mut msg = Vec::new();
    msg.extend_from_slice(spec.name);
    msg.extend_from_slice(b": missing value for ");
    msg.extend_from_slice(arg);
    msg
}
