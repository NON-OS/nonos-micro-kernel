// NONOS Operating System (AGPL-3.0-or-later)
// classList sits on nearly every page that has a script at all.

import { mkEl } from './element.mjs';

export function classListChecks(ok) {
  const el = mkEl();
  const cl = globalThis.__njs_classlist(el);

  cl.add('a', 'b');
  ok(el.className === 'a b', `add writes the attribute: ${el.className}`);
  cl.add('a');
  ok(el.className === 'a b', `add does not duplicate: ${el.className}`);
  ok(cl.contains('b'), 'contains finds a class that is there');
  ok(!cl.contains('nope'), 'contains rejects one that is not');
  ok(cl.length === 2, `length counts classes: ${cl.length}`);

  cl.remove('a');
  ok(el.className === 'b', `remove drops one: ${el.className}`);
  ok(cl.toggle('c') === true && el.className === 'b c', 'toggle adds when absent');
  ok(cl.toggle('c') === false && el.className === 'b', 'toggle drops when present');

  // The forced form is what a component uses to mirror its own state, so it
  // must not flip a class that is already the way it was asked for.
  ok(cl.toggle('b', true) === true && el.className === 'b', 'forced toggle keeps');
  ok(cl.toggle('b', false) === false && el.className === '', 'forced toggle clears');

  cl.add('b');
  ok(cl.replace('b', 'z') === true && el.className === 'z', 'replace swaps');
  ok(cl.replace('absent', 'x') === false, 'replace reports a miss');
  ok(String(cl) === 'z', `toString gives the value: ${String(cl)}`);

  // Whitespace runs are what markup written by hand actually contains.
  el.className = '  one   two  ';
  ok(cl.length === 2, `runs of spaces do not become classes: ${cl.length}`);
}
