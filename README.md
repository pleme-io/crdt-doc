# crdt-doc

The typed **CRDT-text border** for the pleme-io fleet — the first CRDT-text
dependency in the fleet, authored for [Saber](https://github.com/pleme-io/theory/blob/main/SABER.md)
(the AI-native, GitHub-melded, markdown-first docs/knowledge SaaS).

A document body is **always** a `CrdtDoc` by construction. A solo author is just
a one-replica CRDT — the *same* codepath as live co-editing — so single-writer
and collaborative editing are never two implementations.

```rust
use crdt_doc::{open_doc, CrdtKind, CrdtDoc};

let doc = open_doc(CrdtKind::Solo, "# hello\nworld")?;
assert_eq!(doc.materialize().to_string(), "# hello\nworld");
```

## Status — M0a (typed-and-degenerate)

The border is **typed and compiling**, but only the single-writer engine
(`SoloDoc`) is realized. The collaborative engines are typed, named, and
**unbuilt** — constructing one returns a typed error, never a silent `Ok`:

| `CrdtKind` | M0a state |
|---|---|
| `Solo` | ✅ realized (one-replica, no merge) |
| `Loro` | typed → `SpecError::Unimplemented` (M2 primary) |
| `YCrdt` | typed → `SpecError::Unimplemented` (M2 interop) |

The real `loro`/`yrs` merge engines land at **M2**, behind the *same* trait and
a mocked-transport convergence forcing-function (the no-clobber merge gate).

## The seam-clean rule

- `Edit` — the local-author border (escriba).
- `CrdtUpdate` — the wire border between replicas.
- `materialize() → Rope` — the render border (mojiban / nami-core consume it).

These are never crossed raw. At M0a only `materialize()` is realized.

## License

MIT
