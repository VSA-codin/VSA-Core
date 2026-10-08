use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SettingsLoadState {
    Missing,
    Loaded,
    Invalid,
    Unavailable,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppSettings {
    pub compact_layout: bool,
}

pub const SETTINGS_SCHEMA_VERSION: u32 = 1;

impl Serialize for AppSettings {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut fields = serializer.serialize_struct("AppSettings", 2)?;
        fields.serialize_field("schemaVersion", &SETTINGS_SCHEMA_VERSION)?;
        fields.serialize_field("compactLayout", &self.compact_layout)?;
        fields.end()
    }
}

// Used only to refuse recovery, never to admit settings. A present unsupported
// or malformed version requires manual review rather than a reset/migration.
pub(crate) fn reset_schema_supported(bytes: &[u8]) -> bool {
    struct VersionGate(bool);
    impl<'de> Deserialize<'de> for VersionGate {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = VersionGate;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("a settings object")
                }
                fn visit_map<M: serde::de::MapAccess<'de>>(
                    self,
                    mut map: M,
                ) -> Result<VersionGate, M::Error> {
                    let mut supported = true;
                    let mut seen = false;
                    while let Some(key) = map.next_key::<String>()? {
                        if key == "schemaVersion" {
                            let value = map.next_value::<serde_json::Value>()?;
                            supported &=
                                !seen && value.as_u64() == Some(u64::from(SETTINGS_SCHEMA_VERSION));
                            seen = true;
                        } else {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                    Ok(VersionGate(supported))
                }
            }
            d.deserialize_map(Visitor)
        }
    }
    serde_json::from_slice::<VersionGate>(bytes).map_or(true, |gate| gate.0)
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
                    #[serde(default = "current_schema_version")]
                    schema_version: u32,
                    #[serde(default)]
                    compact_layout: bool,
                }
                fn current_schema_version() -> u32 {
                    SETTINGS_SCHEMA_VERSION
                }
                let fields =
                    Fields::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                if fields.schema_version != SETTINGS_SCHEMA_VERSION {
                    return Err(serde::de::Error::custom("Settings schema is unsupported"));
                }
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
            r#"{"schemaVersion":2}"#,
            r#"{"schemaVersion":0}"#,
            r#"{"schemaVersion":1,"schemaVersion":1}"#,
            r#"{"schemaVersion":null}"#,
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
            r#"{"schemaVersion":1,"compactLayout":false}"#
        );
    }
}
