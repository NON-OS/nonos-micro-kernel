// NONOS Operating System (AGPL-3.0-or-later)
// A page that pushes state is telling the reader it moved, so the address
// has to follow. Otherwise the next relative link resolves against the old
// one and code reading the path after a route change sees the previous one.

export function historyChecks(ok) {
  globalThis.__njs_resolve = rel =>
    rel.startsWith('http') ? rel : `https://example.org${rel}`;
  globalThis.location = globalThis.__njs_mkloc('https://example.org/start');

  globalThis.history.pushState({ page: 1 }, '', '/next');
  ok(globalThis.location.pathname === '/next',
     `pushState moves the address: ${globalThis.location.pathname}`);
  ok(globalThis.history.state.page === 1, 'the state is kept');

  globalThis.history.replaceState({ page: 2 }, '', '/other');
  ok(globalThis.location.pathname === '/other', 'replaceState moves it too');
  ok(globalThis.history.state.page === 2, 'and replaces the state');

  // A push with no address is a state change only, and moving the address
  // would send the next relative link somewhere the page never went.
  const before = globalThis.location.href;
  globalThis.history.pushState({ page: 3 }, '');
  ok(globalThis.location.href === before, 'a push without an address stays put');
  ok(globalThis.history.state.page === 3, 'but still records the state');
}
