# openbim-okstra

Typed foundations for OKSTRA®, the German road and traffic object catalogue
(*Objektkatalog für das Straßen- und Verkehrswesen*, <https://www.okstra.de/>).

```rust
use openbim_okstra::{Package, Version};

let version: Version = "2.023.1".parse().unwrap();
assert!(version.is_development());
assert_eq!(version.release(), Version::LATEST_KNOWN_RELEASE);
assert_eq!(Package::Bauwerke.schema_file_name(), "S_Bauwerke.xsd");
```

## Scope

- `Version`: strict parsing, canonical display, and ordering of OKSTRA
  release (`<major>.<minor>`, three-digit minor) and development-version
  (`<major>.<minor>.<n>`) identifiers; the known releases 1.000–1.015
  (EXPRESS) and 2.015–2.023 (UML).
- `Package`: the 38 `S_*` schema packages listed for OKSTRA 2.023.

This is an early foundation with no dependencies. It does not read or write
OKSTRA-XML and ships no OKSTRA schema or documentation files. Facts are taken
from the public okstra.de download pages as of 2026-09-27. OKSTRA® is a
registered trademark; this crate is independent and not endorsed by the
OKSTRA maintainers.

Licensed under AGPL-3.0-or-later.
