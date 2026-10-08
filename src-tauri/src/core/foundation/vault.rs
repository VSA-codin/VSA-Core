use serde::{Deserialize, Serialize};

/// No unlocked state can be produced by today's unavailable provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VaultState {
    Unavailable,
    Locked,
    RecoveryRequired,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    profile_id: String,
    module_id: String,
    reference_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fields {
    profile_id: String,
    module_id: String,
    reference_id: String,
}
impl SecretReference {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let f: Fields = super::parse_object(bytes)?;
        super::profiles::validate_profile_id(&f.profile_id)?;
        super::super::module_registry::validate_identifier(&f.module_id)?;
        super::super::module_registry::validate_identifier(&f.reference_id)?;
        Ok(Self {
            profile_id: f.profile_id,
            module_id: f.module_id,
            reference_id: f.reference_id,
        })
    }
}
/// Metadata inspection only. There is no API accepting or returning secret bytes.
pub trait VaultMetadataProvider {
    fn state(&self) -> VaultState;
}
pub struct UnavailableVault;
impl VaultMetadataProvider for UnavailableVault {
    fn state(&self) -> VaultState {
        VaultState::Unavailable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn references_are_scoped_metadata_and_provider_is_unavailable() {
        let valid =
            r#"{"profileId":"profile-default","moduleId":"example","referenceId":"reference-1"}"#;
        let reference = SecretReference::parse(valid.as_bytes()).unwrap();
        assert!(SecretReference::parse(&serde_json::to_vec(&reference).unwrap()).is_ok());
        assert_eq!(UnavailableVault.state(), VaultState::Unavailable);
        for text in [
            valid.replace("reference-1", "../escape"),
            valid.replace("profile-default", "con"),
            valid.replace("example", "UPPER"),
            valid.replace(
                "\"referenceId\":\"reference-1\"",
                "\"referenceId\":\"reference-1\",\"referenceId\":\"other\"",
            ),
            valid.replace("\"referenceId\"", "\"password\""),
            "[]".into(),
        ] {
            assert!(SecretReference::parse(text.as_bytes()).is_err());
        }
    }
}
