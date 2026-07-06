# usb_proofs

Host-runnable proofs for the USB HID driver's descriptor parser. The real
`descriptors::hid_bindings` source is included through `#[path]` and run on the
host.

## HID configuration descriptor

A USB configuration descriptor is supplied by the device and is fully
attacker-controlled. It is a variable-length list of descriptors, each with its
own length byte, which makes it a classic parser attack surface. The proof
establishes that parsing never panics, that a descriptor claiming a zero length
is rejected rather than looped on (the descriptor-walk denial of service), and
that the number of returned bindings never exceeds the fixed cap.

A malicious or malfunctioning USB device is inside the threat model for any
machine with an exposed port, so this parser runs on data the system does not
control.

## Run

```sh
cd userland/usb_proofs
cargo test --release
```
