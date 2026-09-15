# i2c_transfer_proofs

Host-runnable proofs for the Intel LPSS I2C capsule's transfer engine, probe
and IPC handler. The shipping driver source is included through `#[path]`;
the register accessor alone is replaced with one that forwards to the
DesignWare core in `nonos_i2cmodel`, with a HID touchpad on the bus behind it.

## What was found

Three defects in the shipping engine, each caught by a test here before the
fix and passing after it.

A NACKed transfer reported success. An abort empties the TX FIFO and idles
the bus exactly as a completed transfer does; only `TX_ABRT` tells them apart,
and the engine checked it at the top of its loop but declared completion at
the bottom. On the model the window is deterministic; on silicon it is a race
the driver loses a fraction of the time, and every probe of an absent address
could come back as a device. The engine now checks the abort again before
trusting completion.

The FIFO depths were assumed. The engine pushed against a constant 64 while
the core reports its depths in `IC_COMP_PARAM_1`, and silicon ships with 8,
16 and 32. Against an eight-entry core the driver lost 56 of 64 commands. The
depths are now read at bring-up and carried in the driver.

Reads in flight were not counted. `IC_RXFLR` counts bytes that have landed,
not reads issued, so a burst of read commands could exceed the room left in
the receive FIFO. The engine now counts issued reads against that room.

## What is proved

A write is one START, the bytes in order, one STOP, and leaves the core
disabled with no rule broken. A register read turns round with one repeated
START, returns the register, and NACKs exactly its last byte; without the
restart flag the core still repeats the START because bring-up sets
`RESTART_EN`. An address nobody answers and a device that NACKs its data are
both a NACK to the caller, with the abort acknowledged and the next transfer
going through. The target is written only while the core is disabled. The
OP_TRANSFER handler answers with the length, the abort source and the bytes
in the layout the HID driver decodes, refuses a body whose length field
disagrees with its size before touching the bus, and passes a NACK on as its
own errno. The HID probe recognises a device by its descriptor length and
answers no, not error, for an empty address.

## Run

```sh
cd userland/i2c_transfer_proofs
cargo test --release
```
