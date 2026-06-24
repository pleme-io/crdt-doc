# crdt-doc — AI agent instructions

The typed **CRDT-text border** for the pleme-io fleet — the `CrdtDoc` trait +
`CrdtKind` dispatch authored for [Saber](https://github.com/pleme-io/theory/blob/main/SABER.md)
(see SABER §5). A document body is always a `CrdtDoc`; a solo author is a
one-replica CRDT — the same codepath as live co-editing.

## Status: M0a — typed-and-degenerate

Only `SoloDoc` (single-writer) is realized. `CrdtKind::{Loro,YCrdt}` are typed,
named, and **unbuilt** — `open_doc` returns `SpecError::Unimplemented`, never a
silent `Ok`, never `todo!()`/`panic!()` (★★ UNREPRESENTABILITY / no-stub-Ok).
The real loro (primary) + yrs (interop) merge engines land at **M2**, behind
this same trait + a mocked-transport convergence forcing-function (the
no-clobber merge gate). At M0a the no-clobber claim is **only-mitigated**; it
becomes truly-unrep iff that test passes at M2.

## Directive waivers (declared, not forgotten)

- **`skip-typed-spec-triplet: single-file-border-no-algorithm`** — at M0a this
  is a single Rust trait border, not an algorithm-with-ordered-phases, so the
  full TYPED-SPEC triplet (a `specs/crdt.lisp (defcrdt-merge)` Lisp phase-spec +
  an `apply(spec,args,env)` interpreter + a mock `Environment`) is degenerate.
  **This waiver EXPIRES at M2:** the convergence merge algorithm + wire protocol
  IS the "do these steps in this order against these inputs" shape, and the
  mock-transport `Environment` IS the testability contract — the triplet becomes
  **mandatory** then.
- **`skip-catalog: single-domain-library`** — one trait border (<3 domains), so
  CATALOG REFLECTION's meta-spec + invariant tests are degenerate. The
  `.typescape.yaml` carries the single border row. Also expires at M2 if the
  domain count grows.

## Conventions

- Edition 2024, MIT, `#![forbid(unsafe_code)]`.
- **No `format!()`** (★★ TYPED EMISSION) — `thiserror`'s `#[error]` is the typed
  emission surface for `SpecError`. NOTE: the ban is currently *clean-by-hand*,
  not enforced-by-construction — `substrate/lib/build/rust/rust-library.nix`
  does not yet wire `with-format-ban`. That fleet-wide fix is tracked; until it
  lands, keep this crate `format!()`-free by review.
- The seam-clean rule (SABER §5): `Edit` (escriba, local-author) / `CrdtUpdate`
  (wire) / `materialize()→Rope` (render) are never crossed raw.
- AUTO-RELEASE: single-crate → `cargo-auto-release.yml@main` (NOT the
  polymorphic `auto-release.yml`, which has no green single-crate consumers).
- Tests: example-based + the degenerate convergence proptest. The full
  interleaved-edit convergence property ships in the SAME commit as the loro
  engine at M2 — never after.
