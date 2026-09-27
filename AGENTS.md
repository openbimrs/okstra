# OKSTRA workspace

Canonical `openbimrs/okstra` repository for OKSTRA®, the German road and
traffic object catalogue.

## Children

- `openbim-okstra/` — typed OKSTRA identifiers (versions, schema packages).
- `scripts/` — authoritative gate (`bash scripts/gate.sh`).
- `.github/workflows/` — CI and the crates.io release workflow.
- `references/` — ignored; local OKSTRA XSD/XMI/EA/PDF material only.

## Boundaries

- May depend only on `openbim-core`, `openbim-step`, and IFC crates (see the
  parent `packages/AGENTS.md`); currently it depends on nothing.
- Never commit, package, or paste OKSTRA schema files, model exports, or
  documentation text. Redistribution rights are not established.
- Every encoded fact (version, package name) must be traceable to a public
  okstra.de page; cite it in rustdoc. Do not invent normative content.

Run `bash scripts/gate.sh` before committing.
