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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
    Base,
    Timer,
    Ipi,
    Rfence,
    Hsm,
    Srst,
    Pmu,
    Dbcn,
    Susp,
    Cppc,
    Legacy(usize),
}

impl Extension {
    pub fn eid(&self) -> usize {
        match self {
            Self::Base => 0x10,
            Self::Timer => 0x54494D45,
            Self::Ipi => 0x735049,
            Self::Rfence => 0x52464E43,
            Self::Hsm => 0x48534D,
            Self::Srst => 0x53525354,
            Self::Pmu => 0x504D55,
            Self::Dbcn => 0x4442434E,
            Self::Susp => 0x53555350,
            Self::Cppc => 0x43505043,
            Self::Legacy(eid) => *eid,
        }
    }
}
