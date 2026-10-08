use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TrustState {
    Unsigned,
    VerificationUnavailable,
    UnknownPublisher,
    Untrusted,
    Revoked,
    Malformed,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureMetadata {
    publisher_id: String,
    key_id: String,
    algorithm: Algorithm,
    canonicalization: Canonicalization,
    signature_hex: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Algorithm {
    Ed25519,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Canonicalization {
    JcsRfc8785,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fields {
    publisher_id: String,
    key_id: String,
    algorithm: Algorithm,
    canonicalization: Canonicalization,
    signature_hex: String,
}
impl SignatureMetadata {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let f: Fields = super::parse_object(bytes)?;
        super::super::module_registry::validate_identifier(&f.publisher_id)?;
        super::super::module_registry::validate_identifier(&f.key_id)?;
        if !lower_hex(&f.signature_hex, 128) {
            return Err("Signature encoding is invalid");
        }
        Ok(Self {
            publisher_id: f.publisher_id,
            key_id: f.key_id,
            algorithm: f.algorithm,
            canonicalization: f.canonicalization,
            signature_hex: f.signature_hex,
        })
    }
    /// Parsing is never evidence of authenticity; no verified/trusted state exists.
    pub fn trust_state(&self) -> TrustState {
        TrustState::VerificationUnavailable
    }
}
pub(crate) fn lower_hex(value: &str, size: usize) -> bool {
    value.len() == size
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn valid() -> String {
        format!(
            r#"{{"publisherId":"example","keyId":"key-1","algorithm":"ed25519","canonicalization":"jcsRfc8785","signatureHex":"{}"}}"#,
            "a".repeat(128)
        )
    }
    #[test]
    fn valid_signature_shape_never_establishes_trust() {
        let parsed = SignatureMetadata::parse(valid().as_bytes()).unwrap();
        assert_eq!(parsed.trust_state(), TrustState::VerificationUnavailable);
        assert_eq!(
            SignatureMetadata::parse(&serde_json::to_vec(&parsed).unwrap())
                .unwrap()
                .trust_state(),
            TrustState::VerificationUnavailable
        );
    }
    #[test]
    fn malformed_signatures_and_claims_fail_closed() {
        for text in [
            valid().replace("ed25519", "custom"),
            valid().replace("jcsRfc8785", "raw"),
            valid().replace(&"a".repeat(128), &"A".repeat(128)),
            valid().replace(&"a".repeat(128), "aa"),
            valid().replace("example", "../publisher"),
            valid().replace(
                "\"keyId\":\"key-1\"",
                "\"keyId\":\"key-1\",\"keyId\":\"key-2\"",
            ),
            valid().replace("\"publisherId\"", "\"trusted\""),
            "[]".into(),
        ] {
            assert!(SignatureMetadata::parse(text.as_bytes()).is_err());
        }
    }
}
