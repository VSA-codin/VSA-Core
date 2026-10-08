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
    pub fn recovery_available(&self) -> bool {
        self.store.recovery_available()
    }
    pub fn recover(&self) -> Result<AppSettings, &'static str> {
        self.store.recover()
    }
    pub fn diagnostics(
        &self,
        registry: &crate::core::ModuleRegistry,
        data_directory: &std::path::Path,
        include_paths: bool,
    ) -> crate::core::diagnostics::Diagnostics {
        crate::core::diagnostics::Diagnostics::current(
            registry,
            self.store.directory(),
            data_directory,
            self.store.load_state(),
            include_paths,
        )
    }
}
