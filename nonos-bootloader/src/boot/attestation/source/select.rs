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

use uefi::prelude::*;

use super::embedded::embedded_proof_source;
use super::load_sidecar::load_zk_sidecar;
use super::types::ProofSource;
use crate::zk::parse_zk_proof;

pub fn select_zk_proof_source<'a>(st: &SystemTable<Boot>, data: &'a [u8]) -> ProofSource<'a> {
    if let Some(sidecar) = load_zk_sidecar(st) {
        match parse_zk_proof(&sidecar) {
            Ok((proof, _)) if proof.is_runtime() => return ProofSource::Sidecar(sidecar),
            _ => return ProofSource::InvalidSidecar,
        }
    }
    embedded_proof_source(data)
}
