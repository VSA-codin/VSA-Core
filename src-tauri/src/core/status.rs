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
    fn installer_metadata_and_least_privilege_remain_explicit() {
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../../tauri.conf.json")).unwrap();
        assert_eq!(config["productName"], "VSA CORE");
        assert_eq!(config["identifier"], "com.vsa.core");
        assert_eq!(config["bundle"]["publisher"], "VSA-codin");
        assert_eq!(
            config["bundle"]["windows"]["nsis"]["installMode"],
            "currentUser"
        );
        assert_eq!(config["bundle"]["windows"]["allowDowngrades"], false);
        assert!(config["bundle"]["windows"]["nsis"]
            .get("installerHooks")
            .is_none());
        assert!(config["bundle"]["windows"]["nsis"]
            .get("template")
            .is_none());
        let capability: serde_json::Value =
            serde_json::from_str(include_str!("../../capabilities/default.json")).unwrap();
        assert_eq!(
            capability["permissions"],
            serde_json::json!([
                "allow-foundation",
                "core:window:allow-close",
                "core:window:allow-minimize",
                "core:window:allow-toggle-maximize",
                "core:window:allow-start-dragging"
            ])
        );
        assert_eq!(config["app"]["security"]["csp"], "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src ipc: http://ipc.localhost; object-src 'none'; base-uri 'none'; frame-src 'none'");
    }
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
