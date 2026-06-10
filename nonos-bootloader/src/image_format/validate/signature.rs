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

use super::error::ImageValidationError;
use crate::image_format::types::SignatureAlgorithm;

pub fn validate_signature_size(
    sig_bytes: &[u8],
    algorithm: SignatureAlgorithm,
) -> Result<(), ImageValidationError> {
    let expected_size = algorithm.signature_size();

    if sig_bytes.len() != expected_size {
        return Err(ImageValidationError::SignatureSizeMismatch);
    }

    if sig_bytes.iter().all(|&b| b == 0) {
        return Err(ImageValidationError::SignatureAllZeros);
    }

    Ok(())
}
