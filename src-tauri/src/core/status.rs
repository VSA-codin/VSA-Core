use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreStatus {
    pub name: &'static str,
    pub version: &'static str,
    pub runtime: &'static str,
    pub mode: &'static str,
    pub privacy: &'static str,
    pub total_modules: usize,
    pub enabled_modules: usize,
}

impl CoreStatus {
    pub fn current(total_modules: usize, enabled_modules: usize) -> Self {
        Self {
            name: "VSA CORE",
            version: env!("CARGO_PKG_VERSION"),
            runtime: "Tauri 2 + Rust",
            mode: "Local",
            privacy: "Local First",
            total_modules,
            enabled_modules,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn frontend_package_version_matches_desktop_version_source() {
        let package: serde_json::Value =
            serde_json::from_str(include_str!("../../../package.json")).unwrap();
        assert_eq!(package["version"], env!("CARGO_PKG_VERSION"));
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../../tauri.conf.json")).unwrap();
        assert!(
            config.get("version").is_none(),
            "Tauri should inherit the Cargo version"
        );
    }
}
