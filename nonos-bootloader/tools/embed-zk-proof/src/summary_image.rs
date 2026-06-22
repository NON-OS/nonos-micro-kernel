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

use embed_zk_proof::{Args, AttestedImage, FOOTER_SIZE};

pub fn print_image_summary(args: &Args, image: &AttestedImage) {
    println!("=== Output ===");
    println!("Written: {} ({} bytes)", args.output.display(), image.data.len());
    println!("\nBreakdown:");
    println!("  Kernel:      {} bytes", image.kernel_size);
    println!("  Signature:   {} bytes", image.signature_size);
    println!("  ZK block:    {} bytes", image.proof_size);
    println!("  Footer:      {} bytes", FOOTER_SIZE);
    println!("  Total:       {} bytes", image.data.len());
}
