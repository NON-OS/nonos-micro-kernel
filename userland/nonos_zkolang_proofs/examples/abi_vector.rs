// Cross-check the on-chain ABI vector against the Rust commitment. Prints the
// canonical bytes, the blake3 digest, and the four field limbs for [Halt], to
// confirm the Solidity RecursionAbi library and src/commit.rs agree byte for byte.
use nonos_zkolang::{commit, commit_limbs, serialize, Op};

fn main() {
    let prog = [Op::Halt];

    let bytes = serialize(&prog);
    print!("serialize([Halt]) =");
    for b in &bytes {
        print!(" {:02x}", b);
    }
    println!("   (expect: 01 0b)");

    let c = commit(&prog);
    print!("commit            = 0x");
    for b in &c {
        print!("{:02x}", b);
    }
    println!("   (SC vector: 0x55c0..369f)");

    let limbs = commit_limbs(&prog);
    for (i, l) in limbs.iter().enumerate() {
        println!("limb[{i}]           = {:>20}  (0x{:016x})", l.value(), l.value());
    }
}
