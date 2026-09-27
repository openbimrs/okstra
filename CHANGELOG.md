# Changelog

All notable changes to this project are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-09-27

### Added

- `openbim-okstra` crate, an early foundation without dependencies.
- `Version`: OKSTRA release (`2.023`) and development-version (`2.023.1`)
  identifiers with strict parsing, canonical display, ordering, the known
  releases 1.000–1.015 and 2.015–2.023, and the EXPRESS/UML modelling line.
- `Package`: the 38 `S_*` schema packages listed for OKSTRA 2.023, with their
  schema file names.
- Gate, CI, and a crates.io trusted-publishing release workflow.

### Security

- `references/` is ignored, and the gate rejects OKSTRA schema, model, and
  documentation payloads in Git and in the crate package.

[Unreleased]: https://github.com/openbimrs/okstra/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/openbimrs/okstra/releases/tag/v0.1.0
