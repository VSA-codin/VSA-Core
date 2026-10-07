use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppSettings {
    #[serde(default)]
    pub compact_layout: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_and_serialization() {
        let settings = AppSettings::default();
        assert!(!settings.compact_layout);
        assert_eq!(serde_json::from_str::<AppSettings>("{}").unwrap(), settings);
        assert_eq!(
            serde_json::from_str::<AppSettings>(&serde_json::to_string(&settings).unwrap())
                .unwrap(),
            settings
        );
        assert!(serde_json::from_str::<AppSettings>(r#"{"token":"secret"}"#).is_err());
    }
}
