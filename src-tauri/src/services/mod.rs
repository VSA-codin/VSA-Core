use crate::{core::settings::AppSettings, storage::SettingsStore};

pub struct SettingsService {
    store: SettingsStore,
}
impl SettingsService {
    pub fn new(store: SettingsStore) -> Self {
        Self { store }
    }
    pub fn get(&self) -> Result<AppSettings, &'static str> {
        self.store.load()
    }
    pub fn update(&self, settings: AppSettings) -> Result<AppSettings, &'static str> {
        self.store.save(&settings)?;
        Ok(settings)
    }
    pub fn directory(&self) -> &std::path::Path {
        self.store.directory()
    }
}
