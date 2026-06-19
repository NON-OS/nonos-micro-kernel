# capsule_input_proof

## Role

`capsule_input_proof` is the input end-to-end proof capsule. It runs as a
CPL=3 capsule, subscribes to the input router, and exercises the full
pointer and keyboard delivery path so a boot validation can assert that a
hardware event observed by a driver capsule reaches a focused client
unaltered. It owns no hardware and renders nothing; its only product is a
sequence of `MkDebug` markers and a PASS / FAIL verdict on the proof
surface.

```text
driver.ps2_kbd0 / driver.usb_hid0
        |
        v
input_router -- OP_SUBSCRIBE / NINP delivery --> capsule_input_proof
        |
        `-- MkDebug PASS/FAIL markers
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x1919
```

The capsule resolves the input router with `MkServiceLookup`, subscribes
with `MkIpcSend`, receives delivery envelopes with `MkIpcRecvFrom`, and
emits proof markers with `MkDebug`. `MkExit` is the only termination path.

## Interface contract

The capsule is a client, not a server. It posts a subscription, waits for
the expected event chain, and maps the outcome to a single PASS or FAIL
marker. It exposes no operations of its own.

## Authority

The capsule may talk to the input router over IPC and write to the debug
surface. It has no PCI, MMIO, IRQ, DMA, PIO, filesystem, network,
display, or focus-routing authority.

## Privacy and persistence

The capsule keeps no state across boots and writes no events to disk. The
markers it emits exist only on the ephemeral debug surface.
