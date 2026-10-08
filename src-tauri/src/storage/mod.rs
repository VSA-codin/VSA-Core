use crate::core::settings::{reset_schema_supported, AppSettings, SettingsLoadState};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

const MAX_SETTINGS_BYTES: u64 = 16 * 1024;

// Reject direct links, including Windows junction/reparse metadata. Ancestor
// links and races between checks and operations remain outside this boundary.
fn is_link(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return true;
        }
    }
    false
}

fn create_private_file(path: &std::path::Path) -> std::io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)
}

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
    fn read_bytes(&self) -> Result<Option<Vec<u8>>, &'static str> {
        // Windows may report a child under a regular file as NotFound rather
        // than NotADirectory. Check the configured directory before interpreting
        // a missing settings file as an intentional default.
        match fs::symlink_metadata(&self.directory) {
            Ok(metadata) if !metadata.is_dir() || is_link(&metadata) => {
                return Err("Settings directory unavailable")
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err("Settings directory unavailable"),
            Ok(_) => {}
        }
        let path = self.directory.join("settings.json");
        match fs::symlink_metadata(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err("Settings cannot be read"),
            Ok(metadata)
                if !metadata.is_file()
                    || is_link(&metadata)
                    || metadata.len() > MAX_SETTINGS_BYTES =>
            {
                return Err("Settings file is invalid")
            }
            Ok(_) => {}
        }
        let mut bytes = Vec::new();
        File::open(path)
            .map_err(|_| "Settings cannot be read")?
            .take(MAX_SETTINGS_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| "Settings cannot be read")?;
        if bytes.len() as u64 > MAX_SETTINGS_BYTES {
            return Err("Settings file is too large");
        }
        Ok(Some(bytes))
    }
    pub fn load(&self) -> Result<AppSettings, &'static str> {
        match self.read_bytes()? {
            None => Ok(AppSettings::default()),
            Some(bytes) => serde_json::from_slice(&bytes)
                .map_err(|_| "Settings JSON is invalid; existing file was preserved"),
        }
    }
    // Read-only observation, not a write-access probe or directory health claim.
    pub fn load_state(&self) -> SettingsLoadState {
        match self.read_bytes() {
            Ok(None) => SettingsLoadState::Missing,
            Ok(Some(bytes)) if serde_json::from_slice::<AppSettings>(&bytes).is_ok() => {
                SettingsLoadState::Loaded
            }
            Ok(Some(_)) => SettingsLoadState::Invalid,
            Err(_) => SettingsLoadState::Unavailable,
        }
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
        let bytes = self.read_bytes()?.ok_or("Settings recovery unavailable")?;
        if !reset_schema_supported(&bytes) || serde_json::from_slice::<AppSettings>(&bytes).is_ok()
        {
            return Err("Settings recovery unavailable");
        }
        Ok(bytes)
    }
    pub fn recover(&self) -> Result<AppSettings, &'static str> {
        // Revalidate under the service lock; never infer corruption from a load error.
        let bytes = self.corrupted_bytes()?;
        let backup = self.directory.join("settings.json.corrupt.bak");
        let mut file = create_private_file(&backup)
            .map_err(|_| "Recovery backup unavailable; existing files preserved")?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|_| "Recovery backup failed; original settings preserved")?;
        drop(file);
        // Detect ordinary changes during backup before resetting. This is a
        // consistency check, not an atomic compare-and-replace or race defense.
        self.reset_after_backup(&bytes)?;
        Ok(AppSettings::default())
    }
    fn reset_after_backup(&self, backed_up: &[u8]) -> Result<(), &'static str> {
        if self.read_bytes()?.as_deref() != Some(backed_up) {
            return Err("Settings changed during recovery; backup preserved, reset refused");
        }
        self.write(&AppSettings::default())
    }
    fn write(&self, settings: &AppSettings) -> Result<(), &'static str> {
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder
            .create(&self.directory)
            .map_err(|_| "Settings directory cannot be created")?;
        let metadata =
            fs::symlink_metadata(&self.directory).map_err(|_| "Settings directory unavailable")?;
        if !metadata.is_dir() || is_link(&metadata) {
            return Err("Settings directory unavailable");
        }
        let bytes =
            serde_json::to_vec_pretty(settings).map_err(|_| "Settings cannot be encoded")?;
        if bytes.len() as u64 > MAX_SETTINGS_BYTES {
            return Err("Settings encoding exceeds size limit");
        }
        let temporary = self.directory.join("settings.json.tmp");
        let mut file = create_private_file(&temporary)
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
    fn unsupported_settings_versions_are_never_saved_or_reset() {
        for bytes in [
            br#"{"schemaVersion":2,"compactLayout":true}"#.as_slice(),
            br#"{"schemaVersion":null}"#,
            br#"{"schemaVersion":"2"}"#,
            br#"{"schemaVersion":2,"schemaVersion":1}"#,
            br#"{"schemaVersion":1,"schemaVersion":2}"#,
            br#"{"schemaVersion":2,"schema\u0056ersion":1}"#,
        ] {
            let fixture = Fixture::new();
            let path = fixture.0.join("settings.json");
            fs::write(&path, bytes).unwrap();
            let store = fixture.store();
            assert!(store.load().is_err());
            assert!(store.save(&AppSettings::default()).is_err());
            assert!(!store.recovery_available());
            assert!(store.recover().is_err());
            assert_eq!(fs::read(path).unwrap(), bytes);
            assert!(!fixture.0.join("settings.json.corrupt.bak").exists());
        }
    }
    #[test]
    fn legacy_settings_read_without_rewrite_and_explicit_save_versions_them() {
        let fixture = Fixture::new();
        let path = fixture.0.join("settings.json");
        fs::write(&path, br#"{"compactLayout":true}"#).unwrap();
        let store = fixture.store();
        let settings = store.load().unwrap();
        assert!(settings.compact_layout);
        assert_eq!(fs::read(&path).unwrap(), br#"{"compactLayout":true}"#);
        store.save(&settings).unwrap();
        let saved: serde_json::Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        assert_eq!(saved["schemaVersion"], 1);
    }
    #[cfg(unix)]
    #[test]
    fn new_directories_are_private_and_existing_directory_modes_are_preserved() {
        use std::os::unix::fs::PermissionsExt;
        let fixture = Fixture::new();
        let directory = fixture.0.join("new-parent").join("new-config");
        SettingsStore::new(directory.clone())
            .save(&AppSettings::default())
            .unwrap();
        for path in [&directory, &fixture.0.join("new-parent")] {
            assert_eq!(fs::metadata(path).unwrap().permissions().mode() & 0o077, 0);
        }
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o750)).unwrap();
        SettingsStore::new(directory.clone())
            .save(&AppSettings::default())
            .unwrap();
        assert_eq!(
            fs::metadata(directory).unwrap().permissions().mode() & 0o777,
            0o750
        );
    }
    #[test]
    fn recovery_reset_refuses_changes_after_backup_without_overwriting_either_file() {
        for changed in [b"{}".as_slice(), b"different corruption"] {
            let fixture = Fixture::new();
            let path = fixture.0.join("settings.json");
            let backup = fixture.0.join("settings.json.corrupt.bak");
            fs::write(&backup, b"original corruption").unwrap();
            fs::write(&path, changed).unwrap();
            assert!(fixture
                .store()
                .reset_after_backup(b"original corruption")
                .is_err());
            assert_eq!(fs::read(&path).unwrap(), changed);
            assert_eq!(fs::read(&backup).unwrap(), b"original corruption");
            assert!(!fixture.0.join("settings.json.tmp").exists());
        }
    }
    #[test]
    fn recovery_reset_refuses_missing_target_and_keeps_backup() {
        let fixture = Fixture::new();
        let backup = fixture.0.join("settings.json.corrupt.bak");
        fs::write(&backup, b"original corruption").unwrap();
        assert!(fixture
            .store()
            .reset_after_backup(b"original corruption")
            .is_err());
        assert!(!fixture.0.join("settings.json").exists());
        assert_eq!(fs::read(backup).unwrap(), b"original corruption");
    }
    #[test]
    fn unicode_directory_and_duplicate_escaped_settings_keys_are_handled_safely() {
        let fixture = Fixture::new();
        let directory = fixture.0.join("VSA Żółć 日本語 spaces");
        let store = SettingsStore::new(directory.clone());
        store
            .save(&AppSettings {
                compact_layout: true,
            })
            .unwrap();
        assert!(store.load().unwrap().compact_layout);
        let bytes = br#"{"compactLayout":true,"compact\u004cayout":false}"#;
        fs::write(directory.join("settings.json"), bytes).unwrap();
        assert!(store.save(&AppSettings::default()).is_err());
        assert_eq!(store.load_state(), SettingsLoadState::Invalid);
        store.recover().unwrap();
        assert_eq!(
            fs::read(directory.join("settings.json.corrupt.bak")).unwrap(),
            bytes
        );
    }
    #[cfg(unix)]
    #[test]
    fn direct_directory_link_is_refused_without_touching_destination() {
        use std::os::unix::fs::symlink;
        let fixture = Fixture::new();
        let destination = fixture.0.join("destination");
        fs::create_dir(&destination).unwrap();
        fs::write(destination.join("settings.json"), "broken").unwrap();
        let link = fixture.0.join("config-link");
        symlink(&destination, &link).unwrap();
        let store = SettingsStore::new(link);
        assert_eq!(store.load_state(), SettingsLoadState::Unavailable);
        assert!(store.load().is_err());
        assert!(store.save(&AppSettings::default()).is_err());
        assert!(store.recover().is_err());
        assert!(store.write(&AppSettings::default()).is_err());
        assert_eq!(
            fs::read(destination.join("settings.json")).unwrap(),
            b"broken"
        );
        assert!(!destination.join("settings.json.tmp").exists());
        assert!(!destination.join("settings.json.corrupt.bak").exists());
    }
    #[cfg(unix)]
    #[test]
    fn read_only_directory_refuses_writes_and_preserves_settings() {
        use std::os::unix::fs::PermissionsExt;
        let fixture = Fixture::new();
        let store = fixture.store();
        store.save(&AppSettings::default()).unwrap();
        fs::set_permissions(&fixture.0, fs::Permissions::from_mode(0o500)).unwrap();
        let writable = create_private_file(&fixture.0.join("permission-probe"));
        // A privileged test runner can bypass mode bits. Restore permissions
        // before any assertion/cleanup, then skip that unsupported boundary.
        if writable.is_ok() {
            fs::set_permissions(&fixture.0, fs::Permissions::from_mode(0o700)).unwrap();
            return;
        }
        let result = store.save(&AppSettings {
            compact_layout: true,
        });
        fs::set_permissions(&fixture.0, fs::Permissions::from_mode(0o700)).unwrap();
        assert!(result.is_err());
        assert_eq!(store.load().unwrap(), AppSettings::default());
        assert!(!fixture.0.join("settings.json.tmp").exists());
    }
    #[test]
    fn missing_config_directory_uses_defaults_without_creating_it() {
        let fixture = Fixture::new();
        let directory = fixture.0.join("missing-config-directory");
        let store = SettingsStore::new(directory.clone());
        assert_eq!(store.load().unwrap(), AppSettings::default());
        assert_eq!(store.load_state(), SettingsLoadState::Missing);
        assert!(!store.recovery_available());
        assert!(!directory.exists());
        store
            .save(&AppSettings {
                compact_layout: true,
            })
            .unwrap();
        assert!(store.load().unwrap().compact_layout);
    }
    #[test]
    fn size_limit_accepts_bounded_valid_json_and_rejects_extra_byte_without_mutation() {
        let fixture = Fixture::new();
        let path = fixture.0.join("settings.json");
        let mut bytes = b"{}".to_vec();
        bytes.resize(MAX_SETTINGS_BYTES as usize, b' ');
        fs::write(&path, &bytes).unwrap();
        assert_eq!(fixture.store().load().unwrap(), AppSettings::default());
        bytes.push(b' ');
        fs::write(&path, &bytes).unwrap();
        assert!(fixture.store().load().is_err());
        assert!(!fixture.store().recovery_available());
        assert!(fixture.store().save(&AppSettings::default()).is_err());
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
    #[test]
    fn replacement_failure_cleans_owned_temporary_and_preserves_target() {
        let fixture = Fixture::new();
        let target = fixture.0.join("settings.json");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("preserved"), "untouched").unwrap();
        // Exercise replacement after writing/flushing a temporary file, without
        // relying on platform-specific permissions or racing another process.
        assert!(fixture.store().write(&AppSettings::default()).is_err());
        assert_eq!(
            fs::read_to_string(target.join("preserved")).unwrap(),
            "untouched"
        );
        assert!(!fixture.0.join("settings.json.tmp").exists());
    }
    #[test]
    fn backup_creation_failure_preserves_corruption_and_existing_directory() {
        let fixture = Fixture::new();
        let path = fixture.0.join("settings.json");
        let backup = fixture.0.join("settings.json.corrupt.bak");
        fs::write(&path, "broken").unwrap();
        fs::create_dir(&backup).unwrap();
        fs::write(backup.join("preserved"), "untouched").unwrap();
        assert!(fixture.store().recover().is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "broken");
        assert_eq!(
            fs::read_to_string(backup.join("preserved")).unwrap(),
            "untouched"
        );
        assert!(!fixture.0.join("settings.json.tmp").exists());
    }
    #[test]
    fn read_only_health_distinguishes_missing_loaded_invalid_and_unavailable() {
        let fixture = Fixture::new();
        let store = fixture.store();
        assert_eq!(store.load_state(), SettingsLoadState::Missing);
        assert!(!fixture.0.join("settings.json").exists());
        fs::write(fixture.0.join("settings.json"), "{}").unwrap();
        assert_eq!(store.load_state(), SettingsLoadState::Loaded);
        fs::write(fixture.0.join("settings.json"), "broken").unwrap();
        assert_eq!(store.load_state(), SettingsLoadState::Invalid);
        fs::remove_file(fixture.0.join("settings.json")).unwrap();
        fs::create_dir(fixture.0.join("settings.json")).unwrap();
        assert_eq!(store.load_state(), SettingsLoadState::Unavailable);
        assert!(!fixture.0.join("settings.json.tmp").exists());
        assert!(!fixture.0.join("settings.json.corrupt.bak").exists());
    }
    #[test]
    fn unavailable_directory_does_not_create_or_replace_files() {
        let fixture = Fixture::new();
        let blocker = fixture.0.join("not-a-directory");
        fs::write(&blocker, "preserved").unwrap();
        let store = SettingsStore::new(blocker.clone());
        assert!(store.load().is_err());
        assert_eq!(store.load_state(), SettingsLoadState::Unavailable);
        assert!(store.save(&AppSettings::default()).is_err());
        assert!(store.recover().is_err());
        assert_eq!(fs::read_to_string(blocker).unwrap(), "preserved");
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
    #[cfg(unix)]
    #[test]
    fn recovery_backup_symlink_is_not_followed_and_files_are_private() {
        use std::os::unix::{fs::symlink, fs::PermissionsExt};
        let fixture = Fixture::new();
        let path = fixture.0.join("settings.json");
        let backup = fixture.0.join("settings.json.corrupt.bak");
        let unrelated = fixture.0.join("untouched");
        fs::write(&path, "broken").unwrap();
        fs::write(&unrelated, "untouched").unwrap();
        symlink(&unrelated, &backup).unwrap();
        assert!(fixture.store().recover().is_err());
        assert_eq!(fs::read_to_string(&unrelated).unwrap(), "untouched");
        assert_eq!(fs::read_to_string(&path).unwrap(), "broken");
        fs::remove_file(&backup).unwrap();
        fixture.store().recover().unwrap();
        for file in [path, backup] {
            assert_eq!(fs::metadata(file).unwrap().permissions().mode() & 0o077, 0);
        }
    }
}
