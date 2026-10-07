use serde::{Deserialize, Serialize};

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
    pub description: String,
    pub lifecycle: ModuleLifecycle,
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
            description: description.into(),
            lifecycle: ModuleLifecycle::Planned,
        }
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
        for (id, name, description) in [
            (
                "steam-power-suite",
                "Steam Power Suite",
                "Planned Steam utilities.",
            ),
            ("vsa-asf", "VSA ASF", "Planned ASF integration."),
            (
                "vsa-stream-drop-collector",
                "VSA StreamDropCollector",
                "Planned stream drop tools.",
            ),
            (
                "vsa-r4r-sda",
                "VSA R4R + SDA",
                "Planned R4R and SDA integration.",
            ),
        ] {
            registry.register(ModuleDescriptor::new(id, name, description))?;
        }
        Ok(registry)
    }
    pub fn register(&mut self, module: ModuleDescriptor) -> Result<(), &'static str> {
        if module.id.trim().is_empty() {
            return Err("Module ID must not be empty");
        }
        if self.modules.iter().any(|existing| existing.id == module.id) {
            return Err("Module ID already registered");
        }
        self.modules.push(module);
        Ok(())
    }
    pub fn modules(&self) -> &[ModuleDescriptor] {
        &self.modules
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
}
