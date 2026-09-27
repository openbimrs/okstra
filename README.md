# openbimrs/okstra

Canonical OpenBIM.rs repository for OKSTRA®, the *Objektkatalog für das
Straßen- und Verkehrswesen* — the German object catalogue for road and traffic
data, published at <https://www.okstra.de/>.

## Crate

- [`openbim-okstra`](openbim-okstra/) — typed foundations: OKSTRA release and
  development-version identifiers (`2.023`, `2.023.1`) and the `S_*` schema
  packages listed for OKSTRA 2.023.

## Status

Early foundation. The crate models identifiers only; it does not yet read or
write OKSTRA-XML and contains no object-type catalogue. Planned next steps, in
roughly this order:

1. detect the OKSTRA version of an OKSTRA-XML document;
2. a typed model of the shared basis types;
3. streaming OKSTRA-XML reading for selected packages;
4. integration with the IFC family for road and bridge exchange.

## Standards material

OKSTRA schema files, UML/XMI exports, Enterprise Architect projects, and PDF
or HTML documentation are **not** part of this repository or the crate. Their
redistribution rights have not been established. Keep lawfully obtained local
copies in the ignored `references/` directory. OKSTRA® is a registered
trademark; this project is independent and not endorsed by the OKSTRA
maintainers.

```bash
bash scripts/gate.sh
```

AGPL-3.0-or-later licensed. See [LICENSING.md](LICENSING.md).
