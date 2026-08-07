// NONOS Operating System (AGPL-3.0-or-later)
// Event and CustomEvent were empty functions, so an event a script built
// carried neither its type nor its detail.

export function eventChecks(ok) {
  const ev = new globalThis.CustomEvent('pick', {
    detail: { id: 3 },
    bubbles: true,
    cancelable: true,
  });
  ok(ev.type === 'pick', 'the type is the one it was built with');
  ok(ev.detail && ev.detail.id === 3, 'the detail survives');
  ok(ev.bubbles === true, 'bubbles is what was asked for');

  ev.preventDefault();
  ok(ev.defaultPrevented === true, 'preventDefault records');

  // A default that was never cancellable must not read as prevented, or a
  // handler checking the flag takes the wrong branch.
  const plain = new globalThis.Event('x');
  plain.preventDefault();
  ok(plain.defaultPrevented === false, 'a non-cancelable event ignores preventDefault');
  ok(plain.detail === undefined, 'a plain event has no detail');

  const custom = new globalThis.CustomEvent('y');
  ok(custom.detail === null, 'a detail-less CustomEvent reports null, as the spec says');

  // Frameworks construct these by name.
  ok(globalThis.MouseEvent === globalThis.Event, 'the event aliases resolve');
}
