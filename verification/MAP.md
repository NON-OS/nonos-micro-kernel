# NØNOS verification map

The single idea: prove the code that runs, connect it to a specification, leave
no gap. Four verification technologies, each where it belongs, linked into one
refinement chain. Every row is checked by machine and reproducible from a clean
checkout.

```
                          THE REFINEMENT CHAIN

     SPECIFICATION                IMPLEMENTATION                 PRIMITIVE
   ┌────────────────┐          ┌────────────────┐          ┌────────────────┐
   │    Lean 4      │ refined  │ Verus  (SMT,   │  rests   │ Known-answer   │
   │   abstract     │─────────▶│        on Rust)│─────────▶│ tests + fuzz,  │
   │ security theorem│  onto   │ Kani (all      │   on     │ vs NIST / RFC  │
   │                │          │       inputs)  │          │ vectors        │
   └────────────────┘          └────────────────┘          └────────────────┘
   what must be true        the real Rust satisfies it     the primitive is exact


                      WHAT IS PROVEN, AND BY WHAT

  SURFACE               SPEC          IMPLEMENTATION        PRIMITIVE        PR
  ─────────────────────────────────────────────────────────────────────────────
  Cryptography          Crypto        .                     56 KATs +       #265
    SHA2/SHA3/BLAKE3                                         constant-time
    HMAC/HKDF                                                (2 ZK bugs
    ChaCha/AES-GCM                                           found + fixed)
    Ed25519/P-256/P-384
    secp256k1/RSA
  Capabilities          Capability    Verus                 .               #264
  IPC length gate       Ipc           Verus                 .               #264
  Paging permissions    Paging        Verus                 .               #264
  Boot anti-rollback    AntiRollback  Kani (all u64)        runnable        #266
  Boot image parser     BootImage     Kani (total)          fuzz            #266
  Kernel W^X            Isolation     Kani (all bits)       runnable        #268
  User-copy bounds      Isolation     Kani (all addr/len)   runnable        #268
  Syscall id decode     Syscall       Kani (all u64)        runnable        #268
  Authorization gate    Authorization .                     runnable        #268
  Capsule ELF loader    Loader        .                     fuzz            #268
  Filesystem paths      Path          Kani                  runnable        #263
  Network parsers       NetParse      .                     fuzz            #267
    DNS/ICMP/ARP                                            (no panic,
    TCP/DHCP                                                 no OOB,
                                                             no loop)
  AHCI block I/O        BlockIO       Kani (all req)        fuzz            #271
  USB HID descriptor    UsbHid        .                     fuzz            #274
  NVMe block I/O        .             .                     fuzz            #276
  USB MSC parsers       .             .                     fuzz            #278
  xHCI TRB encoding     .             .                     runnable        #284
  virtio-net RX slots   .             .                     fuzz            #287
  ─────────────────────────────────────────────────────────────────────────────
  Every crate: clippy -D warnings clean, zero allow(dead_code), source
  included (never copied) so a divergence is a build error.


              TRANSPARENT POST-QUANTUM PROOFS  (new)          #277

   hash-and-integer only, no trusted setup, no elliptic curve, so verification
   is post-quantum. Built from the field up, proven at each layer.

     Goldilocks     Merkle          polynomial          FRI        AIR +      end
     field      ─▶  commitment  ─▶  + low-degree    ─▶  low-    ─▶  Fiat-  ─▶  to
                    (BLAKE3)         extension           degree      Shamir     end
      [done]         [done]           [done]             test
     ring laws      binding,         LDE recovers       [next]     [then]     [goal]
     in Lean       tamper-proof      the polynomial
    Stark.Field    Stark.Merkle


                        WHY THIS, NOT A MODEL ALONE

   A model proof leaves a model-implementation gap. Here each abstract theorem
   in Lean names the Verus or Kani proof that discharges it on the real Rust,
   which rests on primitives checked against the published standard vectors.

   Lean spec  ─▶  Verus / Kani on real Rust  ─▶  KAT / fuzz on real primitives
```
