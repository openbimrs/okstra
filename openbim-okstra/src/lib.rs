//! Typed foundations for OKSTRA®, the *Objektkatalog für das Straßen- und
//! Verkehrswesen* — the German object catalogue for road and traffic data.
//!
//! # Status: early foundation
//!
//! This crate is deliberately small. It models the identifiers that every
//! later OKSTRA capability has to agree on, and nothing more:
//!
//! - [`Version`]: OKSTRA release and development-version identifiers such as
//!   `2.023` and `2.023.1`, with strict parsing, canonical display, and
//!   ordering, plus the list of [known releases](Version::KNOWN_RELEASES).
//! - [`Package`]: the `S_*` schema packages listed for OKSTRA 2.023, each of
//!   which is published as its own XML schema file.
//!
//! It does **not** read or write OKSTRA-XML, does not contain the OKSTRA
//! object-type catalogue, and ships no OKSTRA schema, XMI, or documentation
//! files. Redistribution rights for that material have not been established.
//!
//! # Roadmap
//!
//! Later releases are expected to add, in roughly this order: version
//! detection from OKSTRA-XML documents, a typed model of the shared basis
//! types, streaming OKSTRA-XML reading for selected packages, and integration
//! with the IFC family for road and bridge exchange. None of that exists yet.
//!
//! # Example
//!
//! ```
//! use openbim_okstra::{Modelling, Package, Version};
//!
//! let release: Version = "2.023".parse()?;
//! let development: Version = "2.023.1".parse()?;
//!
//! assert!(release.is_known_release());
//! assert_eq!(development.release(), release);
//! assert!(development > release);
//! assert_eq!(release.modelling(), Some(Modelling::Uml));
//!
//! let package: Package = "S_Bauwerke".parse()?;
//! assert_eq!(package.schema_file_name(), "S_Bauwerke.xsd");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Sources
//!
//! All facts encoded here come from the public download pages of
//! <https://www.okstra.de/> (current version, older versions, and development
//! versions), as retrieved on 2026-09-27. OKSTRA® is a registered trademark;
//! this crate is an independent implementation and is not endorsed by the
//! OKSTRA maintainers.

mod package;
mod version;

pub use package::{Package, ParsePackageError};
pub use version::{Modelling, ParseVersionError, Version};
