//! Pure, non-executable metadata contracts. No I/O, cryptography or authority.
pub mod catalog;
pub mod network;
pub mod profiles;
pub mod support;
pub mod trust;
pub mod updates;
pub mod vault;
pub mod version;

use serde::de::DeserializeOwned;

pub(crate) fn parse_object<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, &'static str> {
    if bytes.len() > 16 * 1024 {
        return Err("Metadata exceeds size limit");
    }
    if bytes.iter().find(|byte| !byte.is_ascii_whitespace()) != Some(&b'{') {
        return Err("Metadata must be a JSON object");
    }
    serde_json::from_slice(bytes).map_err(|_| "Metadata JSON is invalid")
}

pub(crate) fn label(value: &str, limit: usize) -> Result<(), &'static str> {
    if value.trim().is_empty()
        || value.len() > limit
        || super::module_registry::has_unsafe_display_characters(value)
    {
        return Err("Metadata label is invalid");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_byte_parser_is_bounded_object_only_and_returns_sanitized_errors() {
        type Parser = fn(&[u8]) -> Result<(), &'static str>;
        let parsers: [Parser; 8] = [
            |b| profiles::ProfileMetadata::parse(b).map(|_| ()),
            |b| trust::SignatureMetadata::parse(b).map(|_| ()),
            |b| updates::UpdateMetadata::parse(b).map(|_| ()),
            |b| network::NetworkPolicy::parse(b).map(|_| ()),
            |b| vault::SecretReference::parse(b).map(|_| ()),
            |b| crate::core::automation::AutomationPlan::parse(b).map(|_| ()),
            |b| crate::core::automation::RetryPolicy::parse(b).map(|_| ()),
            |b| crate::core::module_manifest::ModuleManifest::parse(b).map(|_| ()),
        ];
        for parse in parsers {
            for bytes in [
                b"[]".as_slice(),
                b"null",
                b"true",
                b"0",
                b"\"PRIVATE_INPUT_MARKER\"",
                br#"{"unknown":"PRIVATE_INPUT_MARKER"}"#,
                b"{",
                &[0xff],
            ] {
                let error = parse(bytes).unwrap_err();
                assert!(!error.contains("PRIVATE_INPUT_MARKER"));
            }
            assert!(parse(&vec![b' '; 16385]).is_err());
        }
    }
    #[test]
    fn profile_parser_accepts_exact_bound_and_rejects_growth() {
        let mut bytes = br#"{"schemaVersion":1,"id":"profile-default","name":"Default"}"#.to_vec();
        bytes.resize(16384, b' ');
        assert!(profiles::ProfileMetadata::parse(&bytes).is_ok());
        bytes.push(b' ');
        assert!(profiles::ProfileMetadata::parse(&bytes).is_err());
    }
}
