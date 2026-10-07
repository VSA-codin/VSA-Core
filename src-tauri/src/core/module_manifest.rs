//! Bounded metadata admission only: no filesystem, grants, or execution.
use super::module_registry::{ModuleDescriptor, ModuleLifecycle};
use crate::security::Permission;
use serde::{Deserialize, Serialize};

const MAX_MANIFEST_BYTES: usize = 16 * 1024;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ManifestFields {
    schema_version: u32,
    id: String,
    name: String,
    description: String,
    version: Option<String>,
    lifecycle: ModuleLifecycle,
    declared_permissions: Vec<Permission>,
}

// Fields are private so every admitted manifest has passed validation.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct ModuleManifest(ManifestFields);

impl ModuleManifest {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > MAX_MANIFEST_BYTES {
            return Err("Module manifest is too large");
        }
        // Derived serde structs also accept positional arrays. JSON objects are
        // the only supported interchange format; parse directly to retain
        // duplicate-field detection (never intermediate serde_json::Value).
        if bytes.iter().find(|byte| !byte.is_ascii_whitespace()) != Some(&b'{') {
            return Err("Module manifest must be a JSON object");
        }
        let fields: ManifestFields =
            serde_json::from_slice(bytes).map_err(|_| "Module manifest JSON is invalid")?;
        if fields.schema_version != 1 {
            return Err("Module manifest schema is unsupported");
        }
        // A source manifest cannot attest local installation or activation.
        if !matches!(
            fields.lifecycle,
            ModuleLifecycle::Planned | ModuleLifecycle::Available
        ) {
            return Err("Module manifest cannot declare local installation or activation");
        }
        let manifest = Self(fields);
        manifest.descriptor().validate()?;
        Ok(manifest)
    }

    fn descriptor(&self) -> ModuleDescriptor {
        let mut descriptor = ModuleDescriptor::new(&self.0.id, &self.0.name, &self.0.description);
        descriptor.version = self.0.version.clone();
        descriptor.lifecycle = self.0.lifecycle;
        descriptor.declared_permissions = self.0.declared_permissions.clone();
        descriptor
    }

    pub fn into_descriptor(self) -> ModuleDescriptor {
        let mut descriptor = ModuleDescriptor::new(self.0.id, self.0.name, self.0.description);
        descriptor.version = self.0.version;
        descriptor.lifecycle = self.0.lifecycle;
        descriptor.declared_permissions = self.0.declared_permissions;
        descriptor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::ModuleRegistry, security::PermissionDecision};

    const VALID: &str = r#"{"schemaVersion":1,"id":"example","name":"Example É","description":"Metadata","version":"1.0.0","lifecycle":"available","declaredPermissions":["network"]}"#;

    #[test]
    fn admission_roundtrip_never_grants_permissions() {
        let manifest = ModuleManifest::parse(VALID.as_bytes()).unwrap();
        let encoded = serde_json::to_vec(&manifest).unwrap();
        let descriptor = ModuleManifest::parse(&encoded).unwrap().into_descriptor();
        assert_eq!(descriptor.name, "Example É");
        assert_eq!(
            descriptor
                .permission_policy
                .decision(Permission::Network, &descriptor.declared_permissions),
            PermissionDecision::Deny
        );
        let mut registry = ModuleRegistry::new();
        registry.register(descriptor).unwrap();
        let before = serde_json::to_value(registry.modules()).unwrap();
        assert!(registry
            .register(ModuleManifest::parse(&encoded).unwrap().into_descriptor())
            .is_err());
        assert_eq!(serde_json::to_value(registry.modules()).unwrap(), before);
        assert_eq!(registry.enabled_count(), 0);
        assert_eq!(registry.allowed_permission_count(), 0);
    }

    #[test]
    fn hostile_schema_and_runtime_claims_are_rejected() {
        for input in [
            VALID.replace("\"schemaVersion\":1", "\"schemaVersion\":2"),
            VALID.replace(
                "\"schemaVersion\":1",
                "\"schemaVersion\":1,\"schemaVersion\":1",
            ),
            VALID.replace(
                "\"version\":\"1.0.0\"",
                "\"version\":null,\"version\":\"1.0.0\"",
            ),
            VALID.replace("\"available\"", "\"installed\""),
            VALID.replace("\"available\"", "\"enabled\""),
            VALID.replace("\"available\"", "\"unknown\""),
            VALID.replace("\"available\"", "\"planned\""),
            VALID.replace("\"1.0.0\"", "null"),
            VALID.replace("[\"network\"]", "[\"network\",\"network\"]"),
            VALID.replace("[\"network\"]", "[\"unknown\"]"),
            VALID.replace(
                "\"schemaVersion\":1",
                "\"schemaVersion\":1,\"executable\":\"run.exe\"",
            ),
            VALID.replace(
                "\"schemaVersion\":1",
                "\"schemaVersion\":1,\"permissionPolicy\":{\"network\":\"allow\"}",
            ),
            VALID.replace("\"example\"", "\"../example\""),
            VALID.replace(
                "\"id\":\"example\"",
                "\"id\":\"example\",\"i\\u0064\":\"other\"",
            ),
            "[1,\"example\",\"Example\",\"Metadata\",\"1.0.0\",\"available\",[]]".into(),
            format!("{VALID} {{}}"),
            "null".into(),
        ] {
            assert!(ModuleManifest::parse(input.as_bytes()).is_err(), "{input}");
        }
        assert!(ModuleManifest::parse(&[0xff, 0xfe]).is_err());
    }

    #[test]
    fn bounded_inputs_and_metadata_are_validated() {
        let mut padded = VALID.as_bytes().to_vec();
        padded.resize(MAX_MANIFEST_BYTES, b' ');
        assert!(ModuleManifest::parse(&padded).is_ok());
        padded.push(b' ');
        assert_eq!(
            ModuleManifest::parse(&padded).unwrap_err(),
            "Module manifest is too large"
        );
        for (old, replacement) in [
            ("example", "x".repeat(65)),
            ("example", "-example".into()),
            ("example", "example-".into()),
            ("Example É", "é".repeat(65)),
            ("Metadata", "x".repeat(2049)),
            ("Metadata", "bad\\u0000text".into()),
            ("1.0.0", "x".repeat(65)),
            ("1.0.0", "bad\\nversion".into()),
        ] {
            assert!(ModuleManifest::parse(VALID.replace(old, &replacement).as_bytes()).is_err());
        }
    }
}
