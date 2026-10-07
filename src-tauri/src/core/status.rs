use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreStatus {
    pub name: &'static str,
    pub version: &'static str,
    pub runtime: &'static str,
    pub mode: &'static str,
    pub privacy: &'static str,
}

impl CoreStatus {
    pub fn current() -> Self {
        Self {
            name: "VSA CORE",
            version: env!("CARGO_PKG_VERSION"),
            runtime: "Tauri 2 + Rust",
            mode: "Local",
            privacy: "Local First",
        }
    }
}
