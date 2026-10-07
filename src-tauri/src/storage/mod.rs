use crate::core::settings::AppSettings;
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

pub struct SettingsStore {
    directory: PathBuf,
}
impl SettingsStore {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }
    pub fn directory(&self) -> &std::path::Path {
        &self.directory
    }
    pub fn load(&self) -> Result<AppSettings, &'static str> {
        let path = self.directory.join("settings.json");
        match fs::symlink_metadata(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(AppSettings::default())
            }
            Err(_) => return Err("Settings cannot be read"),
            Ok(metadata) if !metadata.is_file() || metadata.len() > 16384 => {
                return Err("Settings file is invalid")
            }
            Ok(_) => {}
        }
        let file = fs::File::open(path).map_err(|_| "Settings cannot be read")?;
        let mut bytes = Vec::new();
        file.take(16385)
            .read_to_end(&mut bytes)
            .map_err(|_| "Settings cannot be read")?;
        if bytes.len() > 16384 {
            return Err("Settings file is too large");
        }
        serde_json::from_slice(&bytes)
            .map_err(|_| "Settings JSON is invalid; existing file was preserved")
    }
    pub fn save(&self, settings: &AppSettings) -> Result<(), &'static str> {
        // Validate before replacing: corrupted existing settings require manual recovery.
        self.load()?;
        fs::create_dir_all(&self.directory).map_err(|_| "Settings directory cannot be created")?;
        let bytes =
            serde_json::to_vec_pretty(settings).map_err(|_| "Settings cannot be encoded")?;
        let temporary = self.directory.join("settings.json.tmp");
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&temporary)
            .map_err(|_| "Settings temporary file unavailable; check for interrupted write")?;
        let result = (|| {
            file.write_all(&bytes)
                .map_err(|_| "Settings write failed")?;
            file.sync_all().map_err(|_| "Settings flush failed")?;
            drop(file);
            fs::rename(&temporary, self.directory.join("settings.json"))
                .map_err(|_| "Settings replacement failed")
        })();
        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "vsa-settings-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn store(&self) -> SettingsStore {
            SettingsStore::new(self.0.clone())
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn missing_save_load_and_replace() {
        let fixture = Fixture::new();
        let store = fixture.store();
        assert_eq!(store.load().unwrap(), AppSettings::default());
        let settings = AppSettings {
            compact_layout: true,
        };
        store.save(&settings).unwrap();
        assert_eq!(store.load().unwrap(), settings);
        store.save(&AppSettings::default()).unwrap();
        assert_eq!(store.load().unwrap(), AppSettings::default());
        assert!(!fixture.0.join("settings.json.tmp").exists());
    }
    #[test]
    fn corruption_is_preserved() {
        let fixture = Fixture::new();
        let store = fixture.store();
        fs::write(fixture.0.join("settings.json"), "broken").unwrap();
        assert!(store.load().is_err());
        assert!(store.save(&AppSettings::default()).is_err());
        assert_eq!(
            fs::read_to_string(fixture.0.join("settings.json")).unwrap(),
            "broken"
        );
    }
    #[test]
    fn oversized_and_non_regular_settings_are_rejected() {
        let fixture = Fixture::new();
        let store = fixture.store();
        let target = fixture.0.join("settings.json");
        fs::write(&target, vec![b' '; 16385]).unwrap();
        assert!(store.load().is_err());
        assert!(store.save(&AppSettings::default()).is_err());
        assert_eq!(fs::metadata(&target).unwrap().len(), 16385);
        fs::remove_file(&target).unwrap();
        fs::create_dir(&target).unwrap();
        assert!(store.load().is_err());
        assert!(store.save(&AppSettings::default()).is_err());
    }
    #[cfg(unix)]
    #[test]
    fn symlink_target_is_not_read_or_replaced() {
        use std::os::unix::fs::symlink;
        let fixture = Fixture::new();
        let target = fixture.0.join("unrelated.json");
        fs::write(&target, r#"{"compactLayout":true}"#).unwrap();
        symlink(&target, fixture.0.join("settings.json")).unwrap();
        assert!(fixture.store().load().is_err());
        assert!(fixture.store().save(&AppSettings::default()).is_err());
        assert_eq!(
            fs::read_to_string(&target).unwrap(),
            r#"{"compactLayout":true}"#
        );
    }
    #[test]
    fn temporary_collision_preserves_existing_file() {
        let fixture = Fixture::new();
        let store = fixture.store();
        store.save(&AppSettings::default()).unwrap();
        fs::write(fixture.0.join("settings.json.tmp"), "do not overwrite").unwrap();
        assert!(store
            .save(&AppSettings {
                compact_layout: true
            })
            .is_err());
        assert_eq!(store.load().unwrap(), AppSettings::default());
        assert_eq!(
            fs::read_to_string(fixture.0.join("settings.json.tmp")).unwrap(),
            "do not overwrite"
        );
    }
}
