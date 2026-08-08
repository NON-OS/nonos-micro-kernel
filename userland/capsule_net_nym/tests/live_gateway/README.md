# Live gateway interop

Runs the capsule's own `run_handshake` against a real Nym entry gateway. The
handshake sources are used unmodified; only the two syscalls it reaches for
are stood in for on the host, and the `Wire` trait is implemented over a real
socket instead of the capsule's websocket stack.

This is what separates a vector-verified implementation from an interoperable
one. Every offline test can pass while a field order or a key derivation is
wrong; a gateway completing registration cannot.

## Running

Gateways come from nym-api. Entry gateways expose a plaintext websocket on
port 9000, so no TLS is needed for this path.

    curl -s https://validator.nymtech.net/api/v1/unstable/nym-nodes/skimmed/entry-gateways/all \
      | jq -r '.nodes.data[] | select(.entry.ws_port == 9000) | "\(.ip_addresses[0]) \(.ed25519_identity_pubkey)"' \
      | head -5

    cargo run --release -- <ip>:9000 <ed25519_identity_pubkey>

A run that ends in `run_handshake() COMPLETED` means the gateway derived the
same shared key, our AEAD opened its sealed signature, its Ed25519 signature
verified over both ephemeral keys, and it accepted ours in return.

Needs network, so it is not part of the offline gate.
