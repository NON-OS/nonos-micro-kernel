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

// The real boot-image footer parser, over untrusted image bytes. Pure: the
// footer layout, the algorithm enums, and the region-extraction logic pull in
// no hardware or UEFI services.
#[path = "../../../src/image_format/footer.rs"]
pub mod footer;

#[path = "../../../src/image_format/types.rs"]
pub mod types;

#[allow(clippy::op_ref)]
#[path = "../../../src/image_format/parse/mod.rs"]
pub mod parse;
