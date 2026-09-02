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

use alloc::string::String;
use alloc::vec::Vec;

use super::types::{File, Store, MAX_FILES};

const README: &[u8] = b"Welcome to NONOS.\n\nThis file lives in the vfs capsule.\nTry: ls, cat /docs/demo.txt, write /hello.txt hi, mkdir /tmp\nThe file manager and text editor see the same filesystem.\n";
const ABOUT: &[u8] = b"NONOS is a RAM-resident x86_64 microkernel.\nEvery app is a signed capsule running at CPL=3 behind a\ncapability boundary; this filesystem is itself a userspace\nservice reached over IPC.\n";
const DEMO: &[u8] = b"Demo loop:\n 1. terminal: write /hello.txt hello from nonos\n 2. file manager: see /hello.txt appear\n 3. editor: Ctrl-O /hello.txt, edit, Ctrl-S\n 4. terminal: cat /hello.txt shows the edit\n";

impl Store {
    pub fn seed(&mut self) {
        let _ = self.mkdir("/docs");
        // Scratch space; std's env::temp_dir() on NONOS points here.
        let _ = self.mkdir("/tmp");
        let _ = self.mkdir("/capsules");
        let _ = self.mkdir("/home/nonos/workspace");
        self.seed_file("/readme.txt", README);
        self.seed_file("/docs/about.txt", ABOUT);
        self.seed_file("/docs/demo.txt", DEMO);
        self.seed_file("/images/pepe.png", include_bytes!("../../../testimages/pepe.png"));
        self.seed_file("/images/hardware.jpg", include_bytes!("../../../testimages/hardware.jpg"));
        self.seed_file("/images/network.gif", include_bytes!("../../../testimages/network.gif"));
        self.seed_file("/images/field.png", include_bytes!("../../../testimages/field.png"));
        self.seed_capsule_store();
        self.seed_audio_store();
    }

    #[cfg(not(feature = "seed-terminal-store"))]
    fn seed_capsule_store(&mut self) {}

    #[cfg(feature = "seed-terminal-store")]
    fn seed_capsule_store(&mut self) {
        self.seed_file("/capsules/hello.elf", store::HELLO_ELF);
        self.seed_file("/capsules/hello.nonos_id_cert.bin", store::HELLO_CERT);
        self.seed_file("/capsules/hello.manifest.bin", store::HELLO_MANIFEST);
        self.seed_file("/capsules/hello.zk_trailer.bin", store::HELLO_TRAILER);
    }

    #[cfg(not(feature = "seed-audio-store"))]
    fn seed_audio_store(&mut self) {}

    #[cfg(feature = "seed-audio-store")]
    fn seed_audio_store(&mut self) {
        self.seed_file("/audio/boot_tone.wav", include_bytes!("../../testassets/boot_tone.wav"));
        self.seed_file("/audio/boot_tone.mp3", include_bytes!("../../testassets/boot_tone.mp3"));
    }

    fn seed_file(&mut self, name: &str, data: &[u8]) {
        if self.files.len() < MAX_FILES && self.find(name).is_none() {
            self.files.push(File::new(String::from(name), Vec::from(data), false));
        }
    }
}

#[cfg(feature = "seed-terminal-store")]
mod store {
    pub const HELLO_ELF: &[u8] =
        include_bytes!("../../../../../userland/capsule_hello/target/x86_64-nonos-user/release/hello");
    pub const HELLO_CERT: &[u8] =
        include_bytes!("../../../../../nonos-data/trust/capsules/hello.nonos_id_cert.bin");
    pub const HELLO_MANIFEST: &[u8] =
        include_bytes!("../../../../../nonos-data/trust/capsules/hello.manifest.bin");
    pub const HELLO_TRAILER: &[u8] =
        include_bytes!("../../../../../nonos-data/trust/capsules/hello.zk_trailer.bin");
}
