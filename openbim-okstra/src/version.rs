//! OKSTRA version identifiers.

use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;

/// An OKSTRA version identifier: a release such as `2.023`, or a development
/// version such as `2.023.1`.
///
/// # Numbering scheme
///
/// OKSTRA releases are numbered `<major>.<minor>`, where the minor number is
/// always written with three digits (`1.009`, `2.016`, `2.023`). The
/// OKSTRA web site describes development versions (*Entwicklungs-Versionen*)
/// as sub-versions of a release: `1.009.3` is the development version with
/// ordinal 3 for OKSTRA 1.009.
///
/// Major version 1 covers the releases modelled in EXPRESS (1.000 to 1.015);
/// major version 2 covers the releases modelled in UML, beginning with the
/// preview-only release 2.015 and the first UML-only release 2.016. See
/// [`Modelling`].
///
/// # Ordering
///
/// Versions compare by major, then minor, then development ordinal, with a
/// release ordering before its own development versions:
/// `2.022 < 2.022.1 < 2.023`. This is the order in which the identifiers were
/// issued; it makes no claim that a development version is a superset of its
/// release.
///
/// # Examples
///
/// ```
/// use openbim_okstra::Version;
///
/// let v: Version = "2.019.1".parse()?;
/// assert_eq!(v.major(), 2);
/// assert_eq!(v.minor(), 19);
/// assert_eq!(v.development(), Some(1));
/// assert_eq!(v.to_string(), "2.019.1");
///
/// // The three-digit minor number is required.
/// assert!("2.19".parse::<Version>().is_err());
/// # Ok::<(), openbim_okstra::ParseVersionError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    major: u8,
    minor: u16,
    development: Option<NonZeroU16>,
}

/// The modelling language an OKSTRA release line is defined in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modelling {
    /// Releases 1.000 to 1.015, modelled in EXPRESS.
    Express,
    /// Releases from 2.015 on, modelled in UML. Release 2.015 was a
    /// preview-only release; 2.016 was the first release modelled
    /// exclusively in UML.
    Uml,
}

const fn release(major: u8, minor: u16) -> Version {
    Version {
        major,
        minor,
        development: None,
    }
}

impl Version {
    /// Every OKSTRA release offered for download on okstra.de as of
    /// 2026-09-27, in ascending order: 1.000 to 1.015 (EXPRESS) and 2.015 to
    /// 2.023 (UML).
    ///
    /// Development versions are not listed. New releases are added by
    /// updating this crate.
    pub const KNOWN_RELEASES: &'static [Version] = &[
        release(1, 0),
        release(1, 1),
        release(1, 2),
        release(1, 3),
        release(1, 4),
        release(1, 5),
        release(1, 6),
        release(1, 7),
        release(1, 8),
        release(1, 9),
        release(1, 10),
        release(1, 11),
        release(1, 12),
        release(1, 13),
        release(1, 14),
        release(1, 15),
        release(2, 15),
        release(2, 16),
        release(2, 17),
        release(2, 18),
        release(2, 19),
        release(2, 20),
        release(2, 21),
        release(2, 22),
        release(2, 23),
    ];

    /// The most recent release this crate knows of: OKSTRA 2.023, adopted on
    /// 2026-05-27.
    pub const LATEST_KNOWN_RELEASE: Version = release(2, 23);

    /// Creates a release identifier, or `None` if `minor` does not fit the
    /// three-digit minor number (`minor > 999`).
    ///
    /// ```
    /// use openbim_okstra::Version;
    ///
    /// assert_eq!(Version::new(2, 23).unwrap().to_string(), "2.023");
    /// assert!(Version::new(2, 1000).is_none());
    /// ```
    #[must_use]
    pub const fn new(major: u8, minor: u16) -> Option<Self> {
        if minor > 999 {
            None
        } else {
            Some(release(major, minor))
        }
    }

    /// Returns the development version with the given ordinal for this
    /// version's release, or `None` if `ordinal` is zero.
    ///
    /// ```
    /// use openbim_okstra::Version;
    ///
    /// let dev = Version::LATEST_KNOWN_RELEASE.with_development(1).unwrap();
    /// assert_eq!(dev.to_string(), "2.023.1");
    /// assert!(Version::LATEST_KNOWN_RELEASE.with_development(0).is_none());
    /// ```
    #[must_use]
    pub const fn with_development(self, ordinal: u16) -> Option<Self> {
        match NonZeroU16::new(ordinal) {
            Some(ordinal) => Some(Self {
                development: Some(ordinal),
                ..self
            }),
            None => None,
        }
    }

    /// The major version number.
    #[must_use]
    pub const fn major(self) -> u8 {
        self.major
    }

    /// The minor version number, `0..=999`.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// The development-version ordinal, or `None` for a release.
    #[must_use]
    pub const fn development(self) -> Option<u16> {
        match self.development {
            Some(ordinal) => Some(ordinal.get()),
            None => None,
        }
    }

    /// Whether this identifies a development version rather than a release.
    #[must_use]
    pub const fn is_development(self) -> bool {
        self.development.is_some()
    }

    /// The release this version belongs to: itself for a release, the base
    /// release for a development version.
    #[must_use]
    pub const fn release(self) -> Self {
        release(self.major, self.minor)
    }

    /// Whether this is one of the [`KNOWN_RELEASES`](Self::KNOWN_RELEASES).
    /// Always `false` for development versions.
    #[must_use]
    pub fn is_known_release(self) -> bool {
        Self::KNOWN_RELEASES.binary_search(&self).is_ok()
    }

    /// The modelling language of this version's release line, or `None` for
    /// a major version this crate does not know.
    #[must_use]
    pub const fn modelling(self) -> Option<Modelling> {
        match self.major {
            1 => Some(Modelling::Express),
            2 => Some(Modelling::Uml),
            _ => None,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:03}", self.major, self.minor)?;
        if let Some(ordinal) = self.development {
            write!(f, ".{ordinal}")?;
        }
        Ok(())
    }
}

/// Why a string is not an OKSTRA version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseVersionError {
    /// The major number is missing, has a leading zero, contains a
    /// non-digit, or exceeds 255.
    InvalidMajor,
    /// The minor number is missing or is not exactly three digits.
    InvalidMinor,
    /// The development ordinal is empty, zero, has a leading zero, contains a
    /// non-digit, or exceeds 65535.
    InvalidDevelopment,
    /// More than three dot-separated components.
    TooManyComponents,
}

impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidMajor => "invalid OKSTRA major version number",
            Self::InvalidMinor => "OKSTRA minor version number must be exactly three digits",
            Self::InvalidDevelopment => "invalid OKSTRA development-version ordinal",
            Self::TooManyComponents => "OKSTRA version has more than three components",
        })
    }
}

impl std::error::Error for ParseVersionError {}

/// Parses a canonical decimal number: ASCII digits only, no sign, no leading
/// zero unless the number is zero itself.
fn canonical_number<T: FromStr>(text: &str) -> Option<T> {
    let digits_only = !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit());
    let leading_zero = text.len() > 1 && text.starts_with('0');
    if digits_only && !leading_zero {
        text.parse().ok()
    } else {
        None
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    /// Parses `<major>.<minor>` or `<major>.<minor>.<development>`.
    ///
    /// The input must be canonical: no surrounding whitespace, no prefix such
    /// as `OKSTRA`, exactly three minor digits, and no leading zeros in the
    /// major number or development ordinal. `Display` output always parses
    /// back to the same value.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major = parts
            .next()
            .and_then(canonical_number::<u8>)
            .ok_or(ParseVersionError::InvalidMajor)?;
        let minor = parts
            .next()
            .filter(|m| m.len() == 3 && m.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|m| m.parse::<u16>().ok())
            .ok_or(ParseVersionError::InvalidMinor)?;
        let development = match parts.next() {
            None => None,
            Some(d) => Some(
                canonical_number::<NonZeroU16>(d).ok_or(ParseVersionError::InvalidDevelopment)?,
            ),
        };
        if parts.next().is_some() {
            return Err(ParseVersionError::TooManyComponents);
        }
        Ok(Self {
            major,
            minor,
            development,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        s.parse().unwrap()
    }

    #[test]
    fn parses_releases_and_development_versions_seen_on_okstra_de() {
        for text in [
            "1.000", "1.008.1", "1.009.3", "1.009.4", "1.010.1", "1.015", "2.015", "2.016.1",
            "2.018.2", "2.019.1", "2.020", "2.022.1", "2.023", "2.023.1",
        ] {
            assert_eq!(v(text).to_string(), text);
        }
    }

    #[test]
    fn exposes_components() {
        let dev = v("2.018.2");
        assert_eq!(
            (dev.major(), dev.minor(), dev.development()),
            (2, 18, Some(2))
        );
        assert!(dev.is_development());
        assert_eq!(dev.release(), v("2.018"));
        assert!(!dev.release().is_development());
        assert_eq!(dev.release().development(), None);
    }

    #[test]
    fn rejects_non_canonical_input() {
        use ParseVersionError::{
            InvalidDevelopment, InvalidMajor, InvalidMinor, TooManyComponents,
        };
        let cases = [
            ("", InvalidMajor),
            ("2", InvalidMinor),
            ("2.", InvalidMinor),
            ("2.23", InvalidMinor),
            ("2.0230", InvalidMinor),
            ("2.02x", InvalidMinor),
            ("2.+23", InvalidMinor),
            ("02.023", InvalidMajor),
            ("+2.023", InvalidMajor),
            ("256.000", InvalidMajor),
            (" 2.023", InvalidMajor),
            ("OKSTRA 2.023", InvalidMajor),
            ("2.023 ", InvalidMinor),
            ("2.023.", InvalidDevelopment),
            ("2.023.0", InvalidDevelopment),
            ("2.023.01", InvalidDevelopment),
            ("2.023.65536", InvalidDevelopment),
            ("2.023.1.1", TooManyComponents),
        ];
        for (text, expected) in cases {
            assert_eq!(text.parse::<Version>(), Err(expected), "{text:?}");
        }
    }

    #[test]
    fn orders_release_before_its_development_versions() {
        assert!(v("2.022") < v("2.022.1"));
        assert!(v("2.022.1") < v("2.022.2"));
        assert!(v("2.022.9") < v("2.023"));
        assert!(v("1.015") < v("2.015"));
        assert!(v("2.009") < v("2.010"));
    }

    #[test]
    fn constructors_enforce_ranges() {
        assert_eq!(Version::new(1, 999), Some(v("1.999")));
        assert_eq!(Version::new(1, 1000), None);
        assert_eq!(Version::new(2, 23).unwrap().with_development(0), None);
        assert_eq!(
            Version::new(2, 23).unwrap().with_development(1),
            Some(v("2.023.1"))
        );
        // A development ordinal is replaced, not nested.
        assert_eq!(v("2.023.1").with_development(2), Some(v("2.023.2")));
    }

    #[test]
    fn known_releases_are_sorted_unique_and_match_the_published_list() {
        let known = Version::KNOWN_RELEASES;
        assert!(known.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(known.len(), 16 + 9);
        assert_eq!(known.first(), Some(&v("1.000")));
        assert_eq!(known.last(), Some(&Version::LATEST_KNOWN_RELEASE));
        assert!(known.iter().all(|r| !r.is_development()));
        let express = known
            .iter()
            .filter(|r| r.modelling() == Some(Modelling::Express))
            .count();
        assert_eq!(express, 16, "okstra.de lists the first 16 EXPRESS releases");
    }

    #[test]
    fn known_release_lookup() {
        assert!(v("2.020").is_known_release());
        assert!(v("1.000").is_known_release());
        assert!(!v("2.020.1").is_known_release());
        assert!(!v("1.016").is_known_release());
        assert!(!v("2.014").is_known_release());
        assert!(!v("2.024").is_known_release());
    }

    #[test]
    fn modelling_follows_major_version() {
        assert_eq!(v("1.015").modelling(), Some(Modelling::Express));
        assert_eq!(v("2.015").modelling(), Some(Modelling::Uml));
        assert_eq!(v("3.000").modelling(), None);
    }

    #[test]
    fn errors_display_as_messages() {
        let message = "2.23".parse::<Version>().unwrap_err().to_string();
        assert!(message.contains("three digits"), "{message}");
    }
}
