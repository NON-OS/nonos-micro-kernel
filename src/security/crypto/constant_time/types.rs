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
pub enum CtVerifyResult {
    Equal,
    NotEqual,
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
pub enum TimingMode {
    Quick,
    Statistical,
}

#[derive(Debug)]
pub struct SelfCheckResult {
    pub passed: bool,
    pub checks_run: u32,
    pub checks_passed: u32,
    pub failure_description: Option<&'static str>,
}
