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
        // Ordinary saves never repair invalid settings.
        self.load()?;
        self.write(settings)
    }
    pub fn recovery_available(&self) -> bool {
        self.corrupted_bytes().is_ok()
    }
    fn corrupted_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let path = self.directory.join("settings.json");
        let metadata = fs::symlink_metadata(&path).map_err(|_| "Settings recovery unavailable")?;
        if !metadata.is_file() || metadata.len() > 16384 {
            return Err("Settings recovery unavailable");
        }
        let mut bytes = Vec::new();
        fs::File::open(path)
            .map_err(|_| "Settings recovery unavailable")?
            .take(16385)
            .read_to_end(&mut bytes)
            .map_err(|_| "Settings recovery unavailable")?;
        if bytes.len() > 16384 || serde_json::from_slice::<AppSettings>(&bytes).is_ok() {
            return Err("Settings recovery unavailable");
        }
        Ok(bytes)
    }
    pub fn recover(&self) -> Result<AppSettings, &'static str> {
        // Revalidate under the service lock; never infer corruption from a load error.
        let bytes = self.corrupted_bytes()?;
        let backup = self.directory.join("settings.json.corrupt.bak");
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(backup)
            .map_err(|_| "Recovery backup unavailable; existing files preserved")?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|_| "Recovery backup failed; original settings preserved")?;
        drop(file);
        let defaults = AppSettings::default();
        self.write(&defaults)?;
        Ok(defaults)
    }
    fn write(&self, settings: &AppSettings) -> Result<(), &'static str> {
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
        assert!(fixture.store().recover().is_err());
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
    #[test]
    fn recovery_preserves_exact_bytes_and_persists_defaults() {
        for bytes in [b"broken".as_slice(), b"{\"unknown\":true}", &[0xff, 0xfe]] {
            let fixture = Fixture::new();
            let store = fixture.store();
            fs::write(fixture.0.join("settings.json"), bytes).unwrap();
            assert!(store.recovery_available());
            assert_eq!(store.recover().unwrap(), AppSettings::default());
            assert_eq!(
                fs::read(fixture.0.join("settings.json.corrupt.bak")).unwrap(),
                bytes
            );
            assert_eq!(store.load().unwrap(), AppSettings::default());
            store
                .save(&AppSettings {
                    compact_layout: true,
                })
                .unwrap();
            assert!(store.load().unwrap().compact_layout);
            assert!(!store.recovery_available());
        }
    }
    #[test]
    fn recovery_refuses_missing_valid_changed_and_unsafe_targets() {
        let fixture = Fixture::new();
        let store = fixture.store();
        assert!(store.recover().is_err());
        let path = fixture.0.join("settings.json");
        fs::write(&path, "broken").unwrap();
        assert!(store.recovery_available());
        fs::write(&path, "{}").unwrap();
        assert!(store.recover().is_err());
        fs::write(&path, vec![0xff; 16385]).unwrap();
        assert!(store.recover().is_err());
        fs::remove_file(&path).unwrap();
        fs::create_dir(&path).unwrap();
        assert!(store.recover().is_err());
        assert!(!fixture.0.join("settings.json.corrupt.bak").exists());
    }
    #[test]
    fn recovery_collisions_preserve_original_and_backup() {
        let fixture = Fixture::new();
        let store = fixture.store();
        let path = fixture.0.join("settings.json");
        let backup = fixture.0.join("settings.json.corrupt.bak");
        fs::write(&path, "broken").unwrap();
        fs::write(&backup, "previous backup").unwrap();
        assert!(store.recover().is_err());
        assert_eq!(fs::read_to_string(&backup).unwrap(), "previous backup");
        fs::remove_file(&backup).unwrap();
        fs::write(fixture.0.join("settings.json.tmp"), "interrupted").unwrap();
        assert!(store.recover().is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "broken");
        assert_eq!(fs::read_to_string(&backup).unwrap(), "broken");
        assert_eq!(
            fs::read_to_string(fixture.0.join("settings.json.tmp")).unwrap(),
            "interrupted"
        );
    }
}
