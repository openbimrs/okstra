//! OKSTRA schema packages.

use std::fmt;
use std::str::FromStr;

macro_rules! packages {
    ($($variant:ident => $name:literal,)+) => {
        /// A schema package (*Paket*) of OKSTRA, as listed for release 2.023.
        ///
        /// OKSTRA is organised into packages named `S_<Topic>`. For each
        /// package the OKSTRA-XML schema of a release contains a schema file
        /// `S_<Topic>.xsd`, included from the central `okstra.xsd`.
        ///
        /// The variants are exactly the 38 packages listed on the okstra.de
        /// download page for OKSTRA 2.023 (retrieved 2026-09-27). Other
        /// releases may have a different set; this enum does not claim that a
        /// package exists in any particular older release. The shared schema
        /// files (`okstra-typen.xsd`, `okstra-basis.xsd`, `Datentypen.xsd`,
        /// `Schluesseltabellen.xsd`) are not packages and are not represented.
        ///
        /// ```
        /// use openbim_okstra::Package;
        ///
        /// let p: Package = "S_Strassennetz".parse()?;
        /// assert_eq!(p, Package::Strassennetz);
        /// assert_eq!(p.name(), "S_Strassennetz");
        /// assert_eq!(p.to_string(), "S_Strassennetz");
        /// assert_eq!(Package::ALL.len(), 38);
        /// # Ok::<(), openbim_okstra::ParsePackageError>(())
        /// ```
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        pub enum Package {
            $(
                #[doc = concat!("Package `", $name, "`.")]
                $variant,
            )+
        }

        impl Package {
            /// All packages, in the alphabetical order of their names as
            /// listed on okstra.de.
            pub const ALL: &'static [Package] = &[$(Package::$variant,)+];

            /// The package name, e.g. `S_Bauwerke`.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Package::$variant => $name,)+
                }
            }

            /// The file name of the package's OKSTRA-XML schema, e.g.
            /// `S_Bauwerke.xsd`.
            #[must_use]
            pub const fn schema_file_name(self) -> &'static str {
                match self {
                    $(Package::$variant => concat!($name, ".xsd"),)+
                }
            }
        }
    };
}

packages! {
    Administration => "S_Administration",
    AllgemeineGeometrieobjekte => "S_Allgemeine_Geometrieobjekte",
    AllgemeineMengenberechnung => "S_Allgemeine_Mengenberechnung",
    AllgemeineObjekte => "S_Allgemeine_Objekte",
    ArbeitsstelleAnStrassen => "S_Arbeitsstelle_an_Strassen",
    BaulicheStrasseneigenschaften => "S_Bauliche_Strasseneigenschaften",
    Bauwerke => "S_Bauwerke",
    DienstZentraleObjektnummernverwaltung => "S_Dienst_Zentrale_Objektnummernverwaltung",
    DynamischeVerkehrsdaten => "S_Dynamische_Verkehrsdaten",
    Entwaesserung => "S_Entwaesserung",
    Entwurf => "S_Entwurf",
    Flaechenmodell => "S_Flaechenmodell",
    Grunderwerb => "S_Grunderwerb",
    Hausnummern => "S_Hausnummern",
    Historisierung => "S_Historisierung",
    Kataster => "S_Kataster",
    Kostenmanagement => "S_Kostenmanagement",
    Kreuzungen => "S_Kreuzungen",
    Landschaftsplanung => "S_Landschaftsplanung",
    Lichtsignalanlage => "S_Lichtsignalanlage",
    Liegenschaftsverwaltung => "S_Liegenschaftsverwaltung",
    Netzaenderungsprotokoll => "S_Netzaenderungsprotokoll",
    Oekologie => "S_Oekologie",
    Organisation => "S_Organisation",
    Projektressourcen => "S_Projektressourcen",
    Pruefdaten => "S_Pruefdaten",
    Reb => "S_REB",
    Schwertransport => "S_Schwertransport",
    StatischeBeschilderung => "S_Statische_Beschilderung",
    Strassenausstattungen => "S_Strassenausstattungen",
    Strassennetz => "S_Strassennetz",
    Strassenverzeichnis => "S_Strassenverzeichnis",
    Strassenzustandsdaten => "S_Strassenzustandsdaten",
    Topografie => "S_Topografie",
    Unfall => "S_Unfall",
    Verkehr => "S_Verkehr",
    Verkehrsbeeinflussungsanlagen => "S_Verkehrsbeeinflussungsanlagen",
    Vermessungspunkt => "S_Vermessungspunkt",
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The string is not the exact name of a known OKSTRA package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParsePackageError;

impl fmt::Display for ParsePackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("not a known OKSTRA package name")
    }
}

impl std::error::Error for ParsePackageError {}

impl FromStr for Package {
    type Err = ParsePackageError;

    /// Parses an exact, case-sensitive package name such as `S_Bauwerke`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .copied()
            .find(|p| p.name() == s)
            .ok_or(ParsePackageError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_the_38_packages_of_okstra_2_023() {
        assert_eq!(Package::ALL.len(), 38);
    }

    #[test]
    fn names_are_unique_prefixed_and_sorted() {
        let names: Vec<_> = Package::ALL.iter().map(|p| p.name()).collect();
        assert!(names.iter().all(|n| n.starts_with("S_")));
        assert!(names.windows(2).all(|w| w[0] < w[1]), "{names:?}");
    }

    #[test]
    fn enum_order_matches_name_order() {
        assert!(Package::ALL.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn round_trips_every_name() {
        for &p in Package::ALL {
            assert_eq!(p.name().parse::<Package>(), Ok(p));
            assert_eq!(p.to_string(), p.name());
            assert_eq!(p.schema_file_name(), format!("{}.xsd", p.name()));
        }
    }

    #[test]
    fn parsing_is_exact() {
        for text in [
            "",
            "Bauwerke",
            "s_bauwerke",
            "S_Bauwerke.xsd",
            " S_Bauwerke",
            "S_Straßennetz",
            "okstra-typen",
            "Schluesseltabellen",
        ] {
            assert_eq!(text.parse::<Package>(), Err(ParsePackageError), "{text:?}");
        }
        assert_eq!("S_REB".parse(), Ok(Package::Reb));
    }
}
