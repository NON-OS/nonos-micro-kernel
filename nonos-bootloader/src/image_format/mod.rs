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

pub mod footer;
pub mod parse;
pub mod types;
pub mod validate;

pub use footer::{ImageFooter, FOOTER_MAGIC, FOOTER_SIZE, FOOTER_VERSION};
pub use parse::{has_production_footer, parse_image_footer, ParseError, ParsedImage};
pub use types::{flags as image_flags, HashAlgorithm, ImageFormat, SignatureAlgorithm};
pub use validate::{validate_image, ImageValidationError};
