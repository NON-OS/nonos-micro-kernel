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

//! Priming the device before a test and reading it afterwards.

use super::descriptor::LEN;
use super::device::HidOverI2c;
use super::record::Command;
use super::serve::framed;

impl HidOverI2c {
    /// Queue an input report, id first, for the host's next input read.
    pub fn push_input(&mut self, report: &[u8]) {
        self.inputs.push_back(framed(report));
    }

    pub fn pending_inputs(&self) -> usize {
        self.inputs.len()
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn take_commands(&mut self) -> Vec<Command> {
        std::mem::take(&mut self.commands)
    }

    /// The feature report the host last set, id first.
    pub fn feature(&self, id: u8) -> Option<&[u8]> {
        self.features.get(&id).map(Vec::as_slice)
    }

    pub fn set_feature(&mut self, id: u8, report: &[u8]) {
        self.features.insert(id, report.to_vec());
    }

    /// Reads that ran past what the addressed register held.
    pub fn over_reads(&self) -> u32 {
        self.over_reads
    }

    pub fn descriptor(&self) -> &[u8; LEN] {
        &self.descriptor
    }

    /// The same device answering its address but NACKing every data byte,
    /// which is what a part in the wrong power state does.
    pub fn deaf(mut self) -> Self {
        self.acks_data = false;
        self
    }
}
