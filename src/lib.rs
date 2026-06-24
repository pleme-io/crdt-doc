//! crdt-doc — the typed CRDT-document border for Saber (SABER.md §5).
//!
//! A verbete body is ALWAYS a `CrdtDoc` by construction (solo = a one-replica
//! CRDT). M0a is deliberately degenerate: the ONLY realized engine is
//! [`SoloDoc`] (single writer, no concurrent merge). [`CrdtKind::Loro`] and
//! [`CrdtKind::YCrdt`] are typed-and-named but UNBUILT — constructing one
//! returns [`SpecError::Unimplemented`], never a silent `Ok`, never a
//! `todo!()`/`panic!()` (★★ UNREPRESENTABILITY / no-stub-Ok; ★★ TYPED-SPEC
//! triplet "every unimplemented surface returns a typed error so consumers see
//! the gap mechanically").
//!
//! The seam-clean rule (SABER §5): `Edit` is the local-author border,
//! `CrdtUpdate` is the wire border, `materialize()→Rope` is the render border —
//! never crossed raw. At M0a only `materialize()` is realized; `local_edit` /
//! `apply_update` are the M2 wire borders and return `SpecError` for non-Solo
//! kinds.
#![forbid(unsafe_code)]

use ropey::Rope;
use serde::{Deserialize, Serialize};

/// The typed error surface for the border. Every unimplemented engine arm
/// returns one of these (no `format!()` — `thiserror`'s `#[error]` is the typed
/// emission surface, allowed surface #2).
#[derive(Debug, thiserror::Error)]
pub enum SpecError {
    /// A `CrdtKind` arm has a type but no realized interpreter yet (M2 work).
    /// Carries the kind so a consumer sees exactly which engine is missing.
    #[error("crdt engine not implemented at M0a: {kind:?} (lands at M2 behind the convergence gate)")]
    Unimplemented { kind: CrdtKind },
    /// A wire operation (`local_edit`/`apply_update`) was attempted on the M0a
    /// degenerate doc, which has no wire form.
    #[error("operation '{op}' has no wire form on the M0a single-writer SoloDoc")]
    NoWireForm { op: &'static str },
}

/// The CRDT engine selector (SABER §2 `CrdtKind`). `Solo` is the M0a
/// degenerate; `Loro` the M2 primary; `YCrdt` the M2 interop arm.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrdtKind {
    /// The M0a one-replica degenerate (single writer; no concurrent merge).
    Solo,
    /// loro — the M2 primary co-editing engine (UNBUILT at M0a).
    Loro,
    /// y-crdt (yrs) — the M2 interop arm (UNBUILT at M0a).
    YCrdt,
}

/// The wire delta exchanged between replicas (SABER §2 `CrdtUpdate`). At M0a
/// this is an opaque byte vector with no producer — the type exists so the M2
/// `local_edit`/`apply_update` signatures are stable now.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrdtUpdate(pub Vec<u8>);

/// The CRDT-document border (SABER §5). A verbete body is ALWAYS one.
///
/// At M0a only [`CrdtDoc::materialize`] is realized. `local_edit`/`apply_update`
/// are the M2 wire borders; the default impls return a typed [`SpecError`] so a
/// caller that reaches for them before M2 sees the gap mechanically.
pub trait CrdtDoc: Send + Sync {
    /// The current materialized text the renderer consumes (SABER §5 render
    /// border). Realized at M0a.
    fn materialize(&self) -> Rope;

    /// The engine kind backing this doc.
    fn kind(&self) -> CrdtKind;

    /// Apply a wire delta from another replica (SABER §2 `CrdtUpdate`).
    /// M2 work — degenerate docs have no wire form.
    ///
    /// # Errors
    /// Always [`SpecError::NoWireForm`] at M0a for the [`SoloDoc`].
    fn apply_update(&mut self, _update: &CrdtUpdate) -> Result<(), SpecError> {
        Err(SpecError::NoWireForm { op: "apply_update" })
    }
}

/// The single-writer degenerate document (M0a). Holds the body text directly —
/// no merge, no concurrent edits. A loro `CrdtDoc` replaces it at M2 behind the
/// same trait.
pub struct SoloDoc {
    text: Rope,
}

impl SoloDoc {
    /// Build a solo doc from materialized body bytes.
    #[must_use]
    pub fn from_str(body: &str) -> Self {
        Self {
            text: Rope::from_str(body),
        }
    }
}

impl CrdtDoc for SoloDoc {
    fn materialize(&self) -> Rope {
        self.text.clone()
    }

    fn kind(&self) -> CrdtKind {
        CrdtKind::Solo
    }
}

/// The typed dispatch over `CrdtKind` (SABER §2 "the dispatch tag"). M0a builds
/// only `Solo`; `Loro`/`YCrdt` return [`SpecError::Unimplemented`] — the
/// typed-but-degenerate border the M0a spec mandates.
///
/// # Errors
/// [`SpecError::Unimplemented`] for every non-`Solo` kind until M2.
pub fn open_doc(kind: CrdtKind, body: &str) -> Result<Box<dyn CrdtDoc>, SpecError> {
    match kind {
        CrdtKind::Solo => Ok(Box::new(SoloDoc::from_str(body))),
        // Typed, named, UNBUILT — a typed error, never a silent Ok.
        CrdtKind::Loro | CrdtKind::YCrdt => Err(SpecError::Unimplemented { kind }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_materializes_its_body() {
        let doc = open_doc(CrdtKind::Solo, "# hello\nworld").expect("solo opens");
        assert_eq!(doc.materialize().to_string(), "# hello\nworld");
        assert_eq!(doc.kind(), CrdtKind::Solo);
    }

    #[test]
    fn loro_is_typed_unimplemented_not_silent_ok() {
        // `Box<dyn CrdtDoc>` is not `Debug`, so match the Result directly
        // rather than `expect_err` (which would require Debug on the Ok type).
        assert!(matches!(
            open_doc(CrdtKind::Loro, "x"),
            Err(SpecError::Unimplemented { kind: CrdtKind::Loro })
        ));
    }

    #[test]
    fn ycrdt_is_typed_unimplemented() {
        assert!(matches!(
            open_doc(CrdtKind::YCrdt, "x"),
            Err(SpecError::Unimplemented { kind: CrdtKind::YCrdt })
        ));
    }

    #[test]
    fn solo_has_no_wire_form() {
        let mut doc = SoloDoc::from_str("x");
        assert!(matches!(
            doc.apply_update(&CrdtUpdate(vec![])),
            Err(SpecError::NoWireForm { op: "apply_update" })
        ));
    }

    // The degenerate (one-replica) case of the M2 convergence forcing-function
    // (SABER §9 risk #1): for a solo doc, materialize() round-trips its body
    // losslessly for arbitrary input. At M2 this generalizes to "two replicas,
    // interleaved edits → identical materialize()", the no-clobber merge gate.
    proptest::proptest! {
        #[test]
        fn solo_materialize_round_trips_arbitrary_body(body in ".*") {
            let doc = open_doc(CrdtKind::Solo, &body).expect("solo opens");
            proptest::prop_assert_eq!(doc.materialize().to_string(), body);
        }
    }
}
