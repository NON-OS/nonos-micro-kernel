# The constant-time argument

The primitives in `src/crypto/util/constant_time` gate every MAC, tag, and
signature comparison in the kernel. Their functional correctness is
machine-checked in this crate: Kani proves `ct_eq_32`, the integer
comparators, and the selects equal to the ordinary operations they replace
over all inputs, and the runnable suite exercises them over roughly a
million sampled inputs. Functional correctness is necessary but not the
point of these functions; the point is that their timing does not depend on
secrets. No model checker we run can see timing, so this document gives the
argument explicitly, function by function, and states exactly where it
ends.

## What "constant time" claims here

For every function below: the sequence of executed instructions and the
sequence of memory addresses accessed depend only on public values (buffer
lengths and table sizes), never on secret contents. Two calls with equal
lengths execute the same trace, whatever the bytes are.

## Function by function

`ct_eq(a, b)` (compare.rs): the length comparison branches, and lengths are
public. On equal lengths the loop bound is the public length; the body is
straight-line XOR and OR into an accumulator with no early exit; the single
data-dependent branch is `diff == 0` after the fence, and its predicate is
the function's public output. The unequal-length arm runs a dummy loop over
the longer public length and volatile-reads its accumulator so the compiler
cannot delete it.

`ct_eq_32`, `ct_eq_64`: the fixed-length forms of the same argument with no
length branch at all.

`ct_select_*` (select.rs): branch-free by construction; the condition is
widened to a mask arithmetically (`-(cond as iN)`), and the result is
mask-AND-OR. No memory indexing at all.

`ct_lookup_u8`, `ct_lookup_u8_16` (lookup.rs): the classic full-scan
lookup. Every call reads all 256 (or 16) table entries in the same order;
the secret index never becomes an address; selection is by arithmetic mask
from `ct_eq_u8`, which is itself branch-free. The memory trace is constant
by construction, which is the defense against cache-line leakage of the
index.

`ct_conditional_move`, `ct_conditional_swap` (copy.rs): the length guard
branches on public lengths only; the loops are mask-AND-OR over every byte
with no data-dependent exit.

## The boundary of the claim

Everything above is an argument about the source. Between the source and
the silicon stand two things this crate does not verify:

The compiler. rustc/LLVM may in principle rewrite mask arithmetic into a
branch or a conditional move with data-dependent timing. The code lowers
that risk deliberately: `#[inline(never)]` pins the comparison functions as
compilation units, and the empty-asm `compiler_fence` before each returned
predicate is opaque to LLVM, so the accumulator cannot be folded into an
early-exit comparison. Kani verifies the Rust semantics, not the emitted
machine code; no objdump-level check runs in CI today, and a dudect-style
statistical measurement is listed as future work in
verification/ARCHITECTURE.md. Anyone auditing a release binary should
disassemble these functions; each is small enough to read in a minute.

The hardware. Data-dependent execution below the ISA (frequency scaling,
DRAM effects) is out of scope, as it is for every software constant-time
claim.

## Where the fences matter

`compiler_fence` (empty inline asm with a memory clobber) ends every
accumulating loop before the final predicate. Kani cannot model inline
assembly; the harnesses treat it as a no-op, which is sound for functional
correctness because it has no architectural effect. Its purpose is purely
to keep the compiler from restructuring the loop it terminates, and that
purpose lives outside what the model checker can confirm; it is covered by
the source argument above and by the sampled KAT suite running the real
compiled code.
