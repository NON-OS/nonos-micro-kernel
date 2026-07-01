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

use alloc::vec::Vec;

use super::parse::parse;
use super::rule::Rule;

const UA: &str = "head{display:none}script{display:none}style{display:none}title{display:none}a{color:#4c9aff}b{font-weight:bold}strong{font-weight:bold}h1{font-weight:bold}h2{font-weight:bold}h3{font-weight:bold}h4{font-weight:bold}h5{font-weight:bold}h6{font-weight:bold}";

pub fn ua_rules() -> Vec<Rule> {
    parse(UA)
}
