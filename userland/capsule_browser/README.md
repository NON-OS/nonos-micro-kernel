# capsule_browser

## Role

`capsule_browser` is the graphical web browser capsule. It owns browser chrome,
navigation state, document parsing, layout, paint, TLS client state, and the
network fetch loop for the user-facing browser app.

```text
desktop shell
    |
    | launch app.browser
    v
browser -- MkIpc --> net.sockets -- TCP/UDP/IP/L2 --> selected NIC capsule
    |
    `-- MkSurface* --> compositor
```

## Microkernel contract

The browser is a user capsule. It does not own NIC hardware, routing tables,
DNS policy, kernel packet parsing, global input focus, or framebuffer memory.
It talks to the network stack through MkIpc services and presents pixels
through the graphics surface ABI.

- `MkIpcSend` and `MkIpcRecv` carry network requests and replies.
- `MkSurfaceCreate` and display-query authority are used for the app surface.
- The manifest capability mask is `CAPSULE_REQUIRED_CAPS = 0x1839`.
- The service endpoint is `service:4760:app.browser`.
- The reply endpoint is `reply:4761:endpoint.app.browser.reply`.

## Interface contract

The browser accepts keyboard and pointer events through the app skeleton,
resolves a URL, opens a socket through `net.sockets`, performs HTTP or HTTPS
fetch, parses the response, builds a simple document flow, and paints the
result into its compositor-owned surface.

## Authority

Authority is intentionally narrow: core execution, memory, IPC, crypto, display
query, and surface creation. The browser has no driver, device enumeration,
MMIO, IRQ, DMA, PIO, filesystem, admin, or debug authority.

## Privacy and persistence

Navigation state, history, TLS transcript material, response bytes, and parsed
documents live in capsule memory. The capsule does not persist browsing history
or write a cache. It does not receive raw NIC frames, device registers, or
kernel-global input state.

## Runtime lifecycle

The launcher starts the capsule, the browser creates a surface, paints chrome,
and waits for navigation input. Each navigation creates one bounded fetch job,
uses DNS and sockets over MkIpc, closes the socket on completion or error, then
renders a parsed document or a clear failure state.

## Failure model

Bad URLs, DNS failure, connect failure, send failure, TLS initialization
failure, TLS certificate failure, oversized responses, oversized TLS server
flights, malformed HTTP, redirect loops, and fetch timeouts become explicit
browser status states. They must not panic the capsule or leave sockets open.

## Current implemented surface

- HTTP request construction and response parsing are present.
- HTTPS fetch uses an in-capsule TLS 1.3 client path.
- TLS server flights and response bodies are bounded before append.
- Redirect count is bounded.
- Socket cleanup runs on fetch completion and fetch failure.
- HTML flow parsing, layout, chrome paint, and document paint are present.

## State ownership

The browser owns address-bar text, navigation history, fetch state, TLS client
state, parsed document state, scroll state, and surface paint buffers.
`net.sockets` owns socket handles. Lower network capsules own packet and link
state. The compositor owns final scene composition and focus policy.

## Operating rules

- Keep all network input bounded before allocation growth.
- Close the active socket on every terminal fetch state.
- Treat TLS and HTTP parse failure as user-visible failure, not success.
- Keep hardware, routing, and packet policy outside the browser.

## Release evidence

Release evidence for this capsule is a signed and attested browser capsule,
static proof that it has no hardware authority, a successful HTTP fetch through
the NØNOS socket stack, a successful HTTPS fetch through the TLS path, bounded
oversize-response rejection, redirect-loop rejection, and GUI render evidence.
