# i2c_hid_proofs

Host-runnable proofs for the HID-over-I2C touchpad capsule, from service
lookup to posted pointer events. The shipping driver source is included
through `#[path]`. Its calls into `nonos_libc` land in a shim shared with
`i2c_transfer_proofs`: the service lookup finds a controller, the IPC call
reaches the shipping i2c_pci OP_TRANSFER handler, and every input event the
driver posts is kept for the test. Below the handler is the modelled
DesignWare core with a modelled precision touchpad on its bus. Both drivers'
real wire code meets in one process, and nothing between the HID driver's
request encoder and the pad's register file is a stand-in.

## What is proved

Setup binds the pad the firmware named and reads its HID descriptor; without
a hint the bus scan finds it; a wrong hint falls back to the scan; a pad at an
address the scan never tries leaves the driver unbound and alive; no
controller service is a refusal.

Wake sends SET_POWER ON before RESET, and drains the zero-length report the
specification says a reset device sends first, so the first poll sees a real
frame.

The report descriptor read over the bus parses to the same field map as the
bytes parsed directly, with every input field at the bit the descriptor puts
it and the feature fields found under their own report id.

Configuration writes input mode 3 and both switches into the feature report,
reads the report before writing it so a vendor's other fields survive, and
leaves a pad already in that mode alone.

Polling turns a finger sliding right into one relative motion to the right
and nothing else, a touch and lift into a click on button one, two fingers
moving down into a wheel step, a palm into nothing, and an empty input
register into a quiet poll. The gesture engine is also checked on decoded
frames alone: motion scaling, the discontinuity that is a new finger, the tap
and the drag that is not one, scrolling that freezes the pointer until both
fingers lift, the palm that freezes it until it lifts, and the click button's
two-frame debounce. The boot-mouse fallback decode is checked on frames with
and without a report id and refuses a length prefix past the buffer.

## What changed in the driver

`TouchGesture::on_touch` takes the decoded `TouchSample` rather than its
eight fields, which removed the one lint switch in the capsule. Two modules
were renamed to stop shadowing their parents, and one rounding was spelled
as the division it is. No behaviour changed.

## Run

```sh
cd userland/i2c_hid_proofs
cargo test --release
```
