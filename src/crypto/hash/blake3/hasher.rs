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

use super::chunk::ChunkState;
use super::output::{parent_output, Output, OutputReader};
use super::{
    CHUNK_LEN, DERIVE_KEY_CONTEXT, DERIVE_KEY_MATERIAL, IV, KEYED_HASH, KEY_LEN, MAX_DEPTH, OUT_LEN,
};
use crate::crypto::constant_time::compiler_fence;

pub struct Hasher {
    key_words: [u32; 8],
    chunk_state: ChunkState,
    cv_stack: [[u32; 8]; MAX_DEPTH],
    cv_stack_len: usize,
    flags: u8,
}

impl Hasher {
    pub fn new() -> Self {
        Self::new_internal(&IV, 0)
    }

    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        let key_words = key_words_from_bytes(key);
        Self::new_internal(&key_words, KEYED_HASH)
    }

    pub fn new_derive_key(context: &str) -> Self {
        let context_hasher = Self::new_internal(&IV, DERIVE_KEY_CONTEXT);
        let mut context_hasher = context_hasher;
        context_hasher.update(context.as_bytes());
        let context_key = context_hasher.finalize();
        let context_key_words = key_words_from_bytes(&context_key);

        Self::new_internal(&context_key_words, DERIVE_KEY_MATERIAL)
    }

    fn new_internal(key_words: &[u32; 8], flags: u8) -> Self {
        Self {
            key_words: *key_words,
            chunk_state: ChunkState::new(key_words, 0, flags),
            cv_stack: [[0u32; 8]; MAX_DEPTH],
            cv_stack_len: 0,
            flags,
        }
    }

    fn push_cv(&mut self, cv: [u32; 8], chunk_counter: u64) {
        let mut cv = cv;
        let mut total_chunks = chunk_counter;

        while total_chunks & 1 == 1 {
            if self.cv_stack_len == 0 {
                break;
            }
            self.cv_stack_len -= 1;
            let left = self.cv_stack[self.cv_stack_len];
            cv = parent_output(left, cv, &self.key_words, self.flags).chaining_value();
            total_chunks >>= 1;
        }

        self.cv_stack[self.cv_stack_len] = cv;
        self.cv_stack_len += 1;
    }

    pub fn update(&mut self, input: &[u8]) -> &mut Self {
        let mut offset = 0;

        while offset < input.len() {
            if self.chunk_state.len() == CHUNK_LEN {
                let cv = self.chunk_state.output().chaining_value();
                let chunk_counter = self.chunk_state.chunk_counter;
                self.push_cv(cv, chunk_counter);
                self.chunk_state = ChunkState::new(&self.key_words, chunk_counter + 1, self.flags);
            }

            let want = CHUNK_LEN - self.chunk_state.len();
            let take = core::cmp::min(want, input.len() - offset);
            self.chunk_state.update(&input[offset..offset + take]);
            offset += take;
        }

        self
    }

    fn final_output(&self) -> Output {
        let mut output = self.chunk_state.output();

        let mut parent_nodes_remaining = self.cv_stack_len;
        while parent_nodes_remaining > 0 {
            parent_nodes_remaining -= 1;
            let left = self.cv_stack[parent_nodes_remaining];
            output = parent_output(left, output.chaining_value(), &self.key_words, self.flags);
        }

        output
    }

    pub fn finalize(&self) -> [u8; OUT_LEN] {
        self.final_output().root_hash()
    }

    pub fn finalize_xof(&self) -> OutputReader {
        let output = self.final_output();
        OutputReader::new(output.cv, output.block, output.block_len, output.flags)
    }

    pub fn reset(&mut self) {
        self.chunk_state = ChunkState::new(&self.key_words, 0, self.flags);
        self.cv_stack_len = 0;
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Hasher {
    fn drop(&mut self) {
        // SAFETY: Using volatile writes to prevent the compiler from optimizing
        // away the zeroing of sensitive cryptographic key material.
        for w in &mut self.key_words {
            unsafe {
                core::ptr::write_volatile(w, 0);
            }
        }
        for cv in &mut self.cv_stack {
            for w in cv {
                unsafe {
                    core::ptr::write_volatile(w, 0);
                }
            }
        }
        compiler_fence();
    }
}

fn key_words_from_bytes(key: &[u8; KEY_LEN]) -> [u32; 8] {
    let mut words = [0u32; 8];
    for (i, chunk) in key.chunks_exact(4).enumerate() {
        words[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }
    words
}
