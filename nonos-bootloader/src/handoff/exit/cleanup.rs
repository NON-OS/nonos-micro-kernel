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

use core::sync::atomic::{compiler_fence, Ordering};

pub fn secure_cleanup_before_jump() {
    wipe_crypto_state();
    wipe_zk_state();
    wipe_signing_keys();
    wipe_entropy_pools();
    compiler_fence(Ordering::SeqCst);
}

fn wipe_crypto_state() {
    crate::crypto::keystore_v2::wipe_all_keys();
}

fn wipe_zk_state() {
    crate::zk::transcript::wipe_transcript();
}

fn wipe_signing_keys() {
    crate::crypto::sig::wipe_signing_state();
}

fn wipe_entropy_pools() {
    crate::entropy::wipe_entropy_state();
}
