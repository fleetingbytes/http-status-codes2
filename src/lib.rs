//! Provides a representation of
//! [IANA's](https://www.iana.org/)
//! [HTTP Status Code Registry](https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)
//! and search functions to query it.
//!
//! A CLI is provided by the [heman](https://docs.rs/heman) crate.
#![deny(missing_docs)]
pub mod status_code_registry;

use convert_case::{Case, Casing};

/// Search for a HTTP status by its code in a given registry.
///
/// Exaple:
/// ```rust
/// use http_status_codes2::{find_by_code, status_code_registry::CODE_REGISTRY};
///
/// assert_eq!(
///     find_by_code(100, &CODE_REGISTRY),
///     Some((
///         100,
///         "Continue",
///         "[RFC9110, Section 15.2.1]",
///         "https://www.rfc-editor.org/rfc/rfc9110.html#section-15.2.1"
///     ))
/// );
/// assert_eq!(find_by_code(600, &CODE_REGISTRY), None);
/// ```
pub fn find_by_code(
    code: usize,
    registry: &[(usize, &'static str, &'static str, &'static str)],
) -> Option<(usize, &'static str, &'static str, &'static str)> {
    Some(*registry.iter().find(|&&it| it.0 == code)?)
}

/// Search a given registry for a HTTP statuses containing a substring (needle) in their description. Returns an iterator over the results.
///
/// Example:
///
/// ```rust
/// use http_status_codes2::{find_by_substring, status_code_registry::UNOFFICIAL_CODE_REGISTRY};
///
/// let mut it = find_by_substring("teapot", &UNOFFICIAL_CODE_REGISTRY);
/// assert_eq!(
///     it.next(),
///     Some((
///         418,
///         "I'm a teapot",
///         "[RFC2324, Section 2.3.2]",
///         "https://www.rfc-editor.org/rfc/rfc2324.html#section-2.3.2"
///     ))
///     .as_ref()
/// );
/// assert_eq!(it.next(), None);
/// ```
pub fn find_by_substring<'a>(
    needle: &'a str,
    registry: &'static [(usize, &'static str, &'static str, &'static str)],
) -> impl Iterator<Item = &'static (usize, &'static str, &'static str, &'static str)> + 'a {
    registry.iter().filter(move |&&it| {
        it.1.to_case(Case::Lower)
            .contains(&needle.to_case(Case::Lower))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use status_code_registry::{CODE_REGISTRY, UNOFFICIAL_CODE_REGISTRY};

    #[test]
    fn test_find_by_code_ok() {
        assert_eq!(
            find_by_code(100, &CODE_REGISTRY),
            Some((
                100,
                "Continue",
                "[RFC9110, Section 15.2.1]",
                "https://www.rfc-editor.org/rfc/rfc9110.html#section-15.2.1"
            ))
        );
    }
    #[test]
    fn test_find_by_code_nok() {
        assert_eq!(find_by_code(600, &CODE_REGISTRY), None);
    }

    #[test]
    fn test_find_by_code_unofficial_ok() {
        assert_eq!(
            find_by_code(418, &UNOFFICIAL_CODE_REGISTRY),
            Some((
                418,
                "I'm a teapot",
                "[RFC2324, Section 2.3.2]",
                "https://www.rfc-editor.org/rfc/rfc2324.html#section-2.3.2"
            ))
        );
    }

    #[test]
    fn test_find_by_code_unofficial_nok() {
        assert_eq!(find_by_code(600, &UNOFFICIAL_CODE_REGISTRY), None);
    }

    #[test]
    fn test_find_by_substring_ok() {
        let mut it = find_by_substring("failed", &CODE_REGISTRY);
        assert_eq!(
            it.next(),
            Some((
                412,
                "Precondition Failed",
                "[RFC9110, Section 15.5.13]",
                "https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.13"
            ))
            .as_ref()
        );
        assert_eq!(
            it.next(),
            Some((
                417,
                "Expectation Failed",
                "[RFC9110, Section 15.5.18]",
                "https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.18"
            ))
            .as_ref()
        );
        assert_eq!(
            it.next(),
            Some((
                424,
                "Failed Dependency",
                "[RFC4918]",
                "https://www.rfc-editor.org/rfc/rfc4918.html"
            ))
            .as_ref()
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_find_by_substring_nok() {
        let mut it = find_by_substring("teapot", &CODE_REGISTRY);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_find_by_substring_unofficial_ok() {
        let mut it = find_by_substring("teapot", &UNOFFICIAL_CODE_REGISTRY);
        assert_eq!(
            it.next(),
            Some((
                418,
                "I'm a teapot",
                "[RFC2324, Section 2.3.2]",
                "https://www.rfc-editor.org/rfc/rfc2324.html#section-2.3.2"
            ))
            .as_ref()
        );
    }

    #[test]
    fn test_find_by_substring_unofficial_nok() {
        let mut it = find_by_substring("Prince Adam", &UNOFFICIAL_CODE_REGISTRY);
        assert_eq!(it.next(), None);
    }
}
