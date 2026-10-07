use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SettingsLoadState {
    Missing,
    Loaded,
    Invalid,
    Unavailable,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppSettings {
    pub compact_layout: bool,
}

// Require a map at both storage and IPC boundaries. Derived struct deserialization
// also accepts positional sequences, which are not part of our JSON contract.
impl<'de> Deserialize<'de> for AppSettings {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SettingsVisitor;
        impl<'de> serde::de::Visitor<'de> for SettingsVisitor {
            type Value = AppSettings;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a settings object")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                map: M,
            ) -> Result<Self::Value, M::Error> {
                #[derive(Deserialize)]
                #[serde(rename_all = "camelCase", deny_unknown_fields)]
                struct Fields {
                    #[serde(default)]
                    compact_layout: bool,
                }
                let fields =
                    Fields::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                Ok(AppSettings {
                    compact_layout: fields.compact_layout,
                })
            }
        }
        deserializer.deserialize_map(SettingsVisitor)
    }
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
    #[test]
    fn strict_ipc_and_storage_schema_rejects_wrong_types_and_duplicate_fields() {
        for input in [
            "null",
            "[]",
            "[true]",
            "true",
            r#"{"compactLayout":null}"#,
            r#"{"compactLayout":"true"}"#,
            r#"{"compactLayout":1}"#,
            r#"{"compactLayout":true,"compactLayout":false}"#,
        ] {
            assert!(
                serde_json::from_str::<AppSettings>(input).is_err(),
                "{input}"
            );
        }
        assert_eq!(
            serde_json::to_string(&AppSettings::default()).unwrap(),
            r#"{"compactLayout":false}"#
        );
    }
}
