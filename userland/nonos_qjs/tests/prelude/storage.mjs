// NONOS Operating System (AGPL-3.0-or-later)
// Storage was absent, so a script that kept a preference threw on the first
// call and took the rest of its setup with it.

export function storageChecks(ok) {
  const s = globalThis.localStorage;
  s.setItem('k', 'v');
  ok(s.getItem('k') === 'v', 'a value round trips');
  ok(s.getItem('absent') === null, 'a missing key is null, not undefined');
  ok(s.length === 1, `length counts what is held: ${s.length}`);
  ok(s.key(0) === 'k', 'key names the entry at a position');

  // Values are strings, and code that stores a number and compares the
  // result to one would otherwise silently disagree.
  s.setItem('n', 5);
  ok(s.getItem('n') === '5', 'values come back as strings');

  s.removeItem('k');
  ok(s.getItem('k') === null, 'removeItem drops it');
  s.clear();
  ok(s.length === 0, 'clear empties the store');

  // Two stores that shared state would leak a session value into the one a
  // page expects to keep separate.
  globalThis.sessionStorage.setItem('only', 'here');
  ok(s.getItem('only') === null, 'the two stores are independent');
}
