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
