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

use super::api_state::{
    BUILD_TIMESTAMP, INIT_DONE, KEYSTORE, KEY_FINGERPRINT, NONOS_KEY_ID, NONOS_PUBLIC_KEY,
};
use core::sync::atomic::Ordering;

pub fn is_initialized() -> bool {
    INIT_DONE.load(Ordering::SeqCst)
}
pub fn key_count() -> usize {
    KEYSTORE.lock().count
}
pub fn get_minimum_version() -> u32 {
    KEYSTORE.lock().minimum_version
}
pub fn get_nonos_key() -> &'static [u8; 32] {
    &NONOS_PUBLIC_KEY
}
pub fn get_nonos_key_id() -> &'static [u8; 32] {
    &NONOS_KEY_ID
}
pub fn get_key_fingerprint() -> &'static str {
    KEY_FINGERPRINT
}
pub fn get_build_timestamp() -> u64 {
    BUILD_TIMESTAMP
}
