use crate::security::{Permission, PermissionDecision, PermissionPolicy};
use serde::{Deserialize, Serialize};

pub(crate) fn validate_identifier(id: &str) -> Result<(), &'static str> {
    if id.trim().is_empty() {
        return Err("Metadata ID must not be empty");
    }
    if !id
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err("Metadata ID must use lowercase ASCII letters, digits, or hyphens");
    }
    if id.len() > 64 || id.starts_with('-') || id.ends_with('-') {
        return Err(
            "Metadata ID must be at most 64 bytes and start and end with a letter or digit",
        );
    }
    Ok(())
}

// Directional formatting characters can disguise labels and versions in the
// UI. Ordinary international text, including combining characters, is allowed.
pub(crate) fn has_unsafe_display_characters(value: &str) -> bool {
    value.chars().any(|character| {
        character.is_control()
            || matches!(character, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModuleLifecycle {
    Planned,
    Available,
    Installed,
    Enabled,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDescriptor {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub required_core_api: Option<super::foundation::version::ReleaseVersion>,
    pub publisher_id: Option<String>,
    pub description: String,
    pub lifecycle: ModuleLifecycle,
    pub declared_permissions: Vec<Permission>,
    pub permission_policy: PermissionPolicy,
}

impl ModuleDescriptor {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: None,
            required_core_api: None,
            publisher_id: None,
            description: description.into(),
            lifecycle: ModuleLifecycle::Planned,
            declared_permissions: Vec::new(),
            permission_policy: PermissionPolicy::default(),
        }
    }
    pub fn validate(&self) -> Result<(), &'static str> {
        validate_identifier(&self.id)?;
        for (value, limit) in [
            (self.name.as_str(), 128),
            (self.description.as_str(), 2048),
            (self.version.as_deref().unwrap_or(""), 64),
        ] {
            if value.len() > limit || has_unsafe_display_characters(value) {
                return Err("Module metadata exceeds its limit or contains control or directional formatting characters");
            }
        }
        if self.name.trim().is_empty() {
            return Err("Module name must not be empty");
        }
        match (self.lifecycle, self.version.as_deref()) {
            (ModuleLifecycle::Planned, Some(_)) => {
                return Err("Planned modules must not declare a version")
            }
            (
                ModuleLifecycle::Available | ModuleLifecycle::Installed | ModuleLifecycle::Enabled,
                None | Some(""),
            ) => return Err("Available, installed, and enabled modules require a version"),
            (_, Some(version)) if version.trim().is_empty() => {
                return Err("Module version must not be blank")
            }
            _ => {}
        }
        if let Some(version) = &self.version {
            super::foundation::version::ReleaseVersion::parse(version)?;
        }
        if let Some(publisher) = &self.publisher_id {
            validate_identifier(publisher)?;
        }
        for (index, permission) in self.declared_permissions.iter().enumerate() {
            if self.declared_permissions[..index].contains(permission) {
                return Err("Module permission already declared");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ModuleRegistry {
    modules: Vec<ModuleDescriptor>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn roadmap() -> Result<Self, &'static str> {
        let mut registry = Self::new();
        for bytes in [
            include_bytes!("../../../modules/steam-power-suite.json").as_slice(),
            include_bytes!("../../../modules/vsa-asf.json").as_slice(),
            include_bytes!("../../../modules/vsa-stream-drop-collector.json").as_slice(),
            include_bytes!("../../../modules/vsa-r4r-sda.json").as_slice(),
        ] {
            registry.register(
                super::module_manifest::ModuleManifest::parse(bytes)?.into_descriptor(),
            )?;
        }
        Ok(registry)
    }
    pub fn register(&mut self, module: ModuleDescriptor) -> Result<(), &'static str> {
        module.validate()?;
        if self.modules.iter().any(|existing| existing.id == module.id) {
            return Err("Module ID already registered");
        }
        self.modules.push(module);
        Ok(())
    }
    pub fn modules(&self) -> &[ModuleDescriptor] {
        &self.modules
    }
    pub fn allowed_permission_count(&self) -> usize {
        self.modules
            .iter()
            .map(|module| {
                module
                    .declared_permissions
                    .iter()
                    .filter(|permission| {
                        module
                            .permission_policy
                            .decision(**permission, &module.declared_permissions)
                            == PermissionDecision::Allow
                    })
                    .count()
            })
            .sum()
    }
    pub fn enabled_count(&self) -> usize {
        self.modules
            .iter()
            .filter(|module| module.lifecycle == ModuleLifecycle::Enabled)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directional_spoofing_is_rejected_without_rejecting_international_labels() {
        let mut registry = ModuleRegistry::new();
        for character in [
            '\u{061c}', '\u{200e}', '\u{200f}', '\u{202e}', '\u{2066}', '\u{2069}',
        ] {
            for field in 0..3 {
                let mut module = ModuleDescriptor::new("example", "Example", "Description");
                module.lifecycle = ModuleLifecycle::Available;
                module.version = Some("1.0.0".into());
                let value = format!("label{character}text");
                match field {
                    0 => module.name = value,
                    1 => module.description = value,
                    _ => module.version = Some(value),
                }
                assert!(registry.register(module).is_err());
                assert!(registry.modules().is_empty());
            }
        }
        registry
            .register(ModuleDescriptor::new(
                "international",
                "Żółć 日本語 e\u{0301}",
                "مرحبا",
            ))
            .unwrap();
        assert_eq!(registry.modules().len(), 1);
    }
    #[test]
    fn larger_registry_retains_order_counts_and_contents_after_invalid_registration() {
        let mut registry = ModuleRegistry::new();
        for index in 0..128 {
            let mut module = ModuleDescriptor::new(format!("module-{index}"), "Metadata", "");
            if index % 3 == 0 {
                module.lifecycle = ModuleLifecycle::Enabled;
                module.version = Some("1.0.0".into());
            }
            registry.register(module).unwrap();
        }
        let before = serde_json::to_value(registry.modules()).unwrap();
        let enabled = registry.enabled_count();
        assert_eq!(enabled, 43);
        for module in [
            ModuleDescriptor::new("module-64", "Duplicate", ""),
            ModuleDescriptor::new("bad/id", "Malformed", ""),
        ] {
            assert!(registry.register(module).is_err());
            assert_eq!(registry.enabled_count(), enabled);
            assert_eq!(serde_json::to_value(registry.modules()).unwrap(), before);
        }
        for (index, module) in registry.modules().iter().enumerate() {
            assert_eq!(module.id, format!("module-{index}"));
        }
        assert_eq!(registry.allowed_permission_count(), 0);
    }
    #[test]
    fn registration_preserves_order_and_rejects_duplicate_without_mutation() {
        let mut registry = ModuleRegistry::new();
        registry
            .register(ModuleDescriptor::new("a", "A", "First"))
            .unwrap();
        registry
            .register(ModuleDescriptor::new("b", "B", "Second"))
            .unwrap();
        assert!(registry
            .register(ModuleDescriptor::new("a", "Replacement", ""))
            .is_err());
        assert!(registry
            .register(ModuleDescriptor::new(" ", "Empty", ""))
            .is_err());
        assert_eq!(
            registry
                .modules()
                .iter()
                .map(|m| m.id.as_str())
                .collect::<Vec<_>>(),
            ["a", "b"]
        );
        assert_eq!(registry.modules()[0].name, "A");
    }
    #[test]
    fn lifecycle_and_enabled_count() {
        let mut registry = ModuleRegistry::new();
        for (index, state) in [
            ModuleLifecycle::Planned,
            ModuleLifecycle::Available,
            ModuleLifecycle::Installed,
            ModuleLifecycle::Enabled,
        ]
        .into_iter()
        .enumerate()
        {
            let mut module = ModuleDescriptor::new(index.to_string(), "Test", "");
            module.lifecycle = state;
            if state != ModuleLifecycle::Planned {
                module.version = Some("1.0.0".into());
            }
            assert_eq!(
                serde_json::from_str::<ModuleLifecycle>(&serde_json::to_string(&state).unwrap())
                    .unwrap(),
                state
            );
            registry.register(module).unwrap();
        }
        assert_eq!(registry.enabled_count(), 1);
        assert!(serde_json::from_str::<ModuleLifecycle>("\"unknown\"").is_err());
    }
    #[test]
    fn permission_counts_require_unique_declarations_and_explicit_allow() {
        let mut registry = ModuleRegistry::new();
        let mut module = ModuleDescriptor::new("permissions", "Test", "");
        module.permission_policy =
            serde_json::from_str(r#"{"network":"allow","clipboard":"allow"}"#).unwrap();
        module.declared_permissions = vec![Permission::Network, Permission::Network];
        assert_eq!(
            registry.register(module.clone()),
            Err("Module permission already declared")
        );
        assert!(registry.modules().is_empty());
        module.declared_permissions = vec![Permission::Network, Permission::FilesystemRead];
        registry.register(module).unwrap();
        assert_eq!(registry.allowed_permission_count(), 1);
    }
    #[test]
    fn roadmap_is_metadata_only() {
        let registry = ModuleRegistry::roadmap().unwrap();
        assert_eq!(registry.modules().len(), 4);
        assert_eq!(registry.enabled_count(), 0);
        assert!(registry
            .modules()
            .iter()
            .all(|m| m.lifecycle == ModuleLifecycle::Planned && m.version.is_none()));
        assert_eq!(
            registry
                .modules()
                .iter()
                .map(|m| m.name.as_str())
                .collect::<Vec<_>>(),
            [
                "Steam Power Suite",
                "VSA ASF",
                "VSA StreamDropCollector",
                "VSA R4R + SDA"
            ]
        );
    }
    #[test]
    fn invalid_metadata_never_mutates_registry() {
        let mut registry = ModuleRegistry::new();
        for id in [
            "UPPER",
            " leading",
            "trailing ",
            "path/segment",
            "non-ascii-é",
            "-leading",
            "trailing-",
        ] {
            assert!(registry
                .register(ModuleDescriptor::new(id, "Name", ""))
                .is_err());
        }
        assert!(registry
            .register(ModuleDescriptor::new("valid", " ", ""))
            .is_err());
        for module in [
            ModuleDescriptor::new("x".repeat(65), "Name", ""),
            ModuleDescriptor::new("valid", "é".repeat(65), ""),
            ModuleDescriptor::new("valid", "Name", "x".repeat(2049)),
            ModuleDescriptor::new("valid", "Name", "control\0character"),
        ] {
            assert!(registry.register(module).is_err());
        }
        for state in [
            ModuleLifecycle::Planned,
            ModuleLifecycle::Available,
            ModuleLifecycle::Installed,
            ModuleLifecycle::Enabled,
        ] {
            let mut module = ModuleDescriptor::new("valid", "Name", "");
            module.lifecycle = state;
            module.version = if state == ModuleLifecycle::Planned {
                Some("1.0.0".into())
            } else {
                None
            };
            assert!(registry.register(module.clone()).is_err());
            module.version = Some("  ".into());
            assert!(registry.register(module).is_err());
        }
        assert!(registry.modules().is_empty());
        assert_eq!(registry.enabled_count(), 0);
    }
}
