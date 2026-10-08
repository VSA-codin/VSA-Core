use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Display names never become paths. IDs have a fixed prefix to avoid Windows device names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileMetadata {
    schema_version: u32,
    id: String,
    name: String,
    created_at_unix_seconds: Option<u64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fields {
    schema_version: u32,
    id: String,
    name: String,
    created_at_unix_seconds: Option<u64>,
}
impl ProfileMetadata {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let fields: Fields = super::parse_object(bytes)?;
        if fields.schema_version != 1 {
            return Err("Profile schema is unsupported");
        }
        validate_profile_id(&fields.id)?;
        super::label(&fields.name, 128)?;
        if fields
            .created_at_unix_seconds
            .is_some_and(|t| t > 253_402_300_799)
        {
            return Err("Profile timestamp is invalid");
        }
        Ok(Self {
            schema_version: 1,
            id: fields.id,
            name: fields.name,
            created_at_unix_seconds: fields.created_at_unix_seconds,
        })
    }
    pub fn default_local() -> Self {
        Self {
            schema_version: 1,
            id: "profile-default".into(),
            name: "Default local profile".into(),
            created_at_unix_seconds: None,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    /// A relative layout contract only; no directories are created or selected.
    pub fn relative_directory(&self) -> String {
        format!("profiles/{}", self.id)
    }
}
pub(crate) fn validate_profile_id(id: &str) -> Result<(), &'static str> {
    super::super::module_registry::validate_identifier(id)?;
    if !id.starts_with("profile-") || id.len() <= 8 {
        return Err("Profile ID is invalid");
    }
    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSelection {
    profiles: Vec<ProfileMetadata>,
    active_profile_id: String,
}
impl ProfileSelection {
    pub fn new(
        profiles: Vec<ProfileMetadata>,
        active_profile_id: String,
    ) -> Result<Self, &'static str> {
        if profiles.is_empty() || profiles.len() > 64 {
            return Err("Profile count is invalid");
        }
        let mut ids = BTreeSet::new();
        for profile in &profiles {
            if !ids.insert(profile.id()) {
                return Err("Profile ID is duplicated");
            }
        }
        if !ids.contains(active_profile_id.as_str()) {
            return Err("Active profile is unknown");
        }
        Ok(Self {
            profiles,
            active_profile_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID: &str =
        r#"{"schemaVersion":1,"id":"profile-example","name":"Example","createdAtUnixSeconds":123}"#;
    #[test]
    fn profile_layout_selection_and_roundtrip() {
        let p = ProfileMetadata::parse(VALID.as_bytes()).unwrap();
        assert_eq!(p.relative_directory(), "profiles/profile-example");
        assert_eq!(
            ProfileMetadata::parse(&serde_json::to_vec(&p).unwrap()).unwrap(),
            p
        );
        assert!(ProfileSelection::new(vec![p.clone()], p.id().into()).is_ok());
        assert!(ProfileSelection::new(vec![p.clone(), p.clone()], p.id().into()).is_err());
        assert!(ProfileSelection::new(vec![p], "profile-missing".into()).is_err());
        assert!(ProfileSelection::new(vec![], "profile-default".into()).is_err());
        assert!(ProfileMetadata::default_local()
            .created_at_unix_seconds
            .is_none());
    }
    #[test]
    fn profile_input_rejects_ambiguity_paths_and_future_versions() {
        for text in [
            VALID.replace("profile-example", "../escape"),
            VALID.replace("profile-example", "CON"),
            VALID.replace("profile-example", "PROFILE-test"),
            VALID.replace("\"schemaVersion\":1", "\"schemaVersion\":2"),
            VALID.replace("123", "253402300800"),
            VALID.replace("Example", "bad\\nname"),
            VALID.replace(
                "\"name\":\"Example\"",
                "\"name\":\"Example\",\"na\\u006de\":\"Other\"",
            ),
            VALID.replace("123}", "123,\"path\":\"C:\\\\private\"}"),
            "[]".into(),
            " ".repeat(16385),
        ] {
            assert!(ProfileMetadata::parse(text.as_bytes()).is_err(), "{text}");
        }
    }
}
