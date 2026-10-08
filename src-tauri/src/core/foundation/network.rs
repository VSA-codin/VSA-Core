use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Inbound,
    Outbound,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Protocol {
    Tcp,
    Udp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Decision {
    Allow,
    Deny,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Rule {
    id: String,
    owner_id: String,
    direction: Direction,
    protocol: Protocol,
    port: u16,
    decision: Decision,
}
impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Rule;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a network rule object")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<Rule, M::Error> {
                #[derive(Deserialize)]
                #[serde(rename_all = "camelCase", deny_unknown_fields)]
                struct Fields {
                    id: String,
                    owner_id: String,
                    direction: Direction,
                    protocol: Protocol,
                    port: u16,
                    decision: Decision,
                }
                let f = Fields::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                Ok(Rule {
                    id: f.id,
                    owner_id: f.owner_id,
                    direction: f.direction,
                    protocol: f.protocol,
                    port: f.port,
                    decision: f.decision,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fields {
    schema_version: u32,
    rules: Vec<Rule>,
}
#[derive(Serialize)]
#[serde(transparent)]
pub struct NetworkPolicy(Fields);
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPreview {
    rule_count: usize,
    allow_count: usize,
    default_decision: Decision,
    backend_available: bool,
}
impl NetworkPolicy {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let f: Fields = super::parse_object(bytes)?;
        if f.schema_version != 1 || f.rules.len() > 128 {
            return Err("Network policy schema or count is invalid");
        }
        let mut ids = BTreeSet::new();
        let mut scopes = BTreeMap::new();
        for rule in &f.rules {
            super::super::module_registry::validate_identifier(&rule.id)?;
            super::super::module_registry::validate_identifier(&rule.owner_id)?;
            if rule.port == 0 || !ids.insert(&rule.id) {
                return Err("Network rule is invalid or duplicated");
            }
            let scope = (&rule.owner_id, rule.direction, rule.protocol, rule.port);
            if let Some(previous) = scopes.insert(scope, rule.decision) {
                return Err(if previous == rule.decision {
                    "Network scope is duplicated"
                } else {
                    "Network rules conflict"
                });
            }
        }
        Ok(Self(f))
    }
    pub fn preview(&self) -> NetworkPreview {
        NetworkPreview {
            rule_count: self.0.rules.len(),
            allow_count: self
                .0
                .rules
                .iter()
                .filter(|r| r.decision == Decision::Allow)
                .count(),
            default_decision: Decision::Deny,
            backend_available: false,
        }
    }
}
/// Future platform adapters expose availability only; no mutation method exists.
pub trait NetworkBackend {
    fn available(&self) -> bool;
}
pub struct DisabledNetworkBackend;
impl NetworkBackend for DisabledNetworkBackend {
    fn available(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const RULE: &str = r#"{"id":"rule-1","ownerId":"example","direction":"outbound","protocol":"tcp","port":443,"decision":"allow"}"#;
    #[test]
    fn network_preview_is_default_deny_with_no_backend() {
        let text = format!(r#"{{"schemaVersion":1,"rules":[{RULE}]}}"#);
        let policy = NetworkPolicy::parse(text.as_bytes()).unwrap();
        let preview = policy.preview();
        assert_eq!(preview.rule_count, 1);
        assert_eq!(preview.allow_count, 1);
        assert_eq!(preview.default_decision, Decision::Deny);
        assert!(!preview.backend_available && !DisabledNetworkBackend.available());
        assert!(NetworkPolicy::parse(&serde_json::to_vec(&policy).unwrap()).is_ok());
    }
    #[test]
    fn conflicts_duplicates_unknown_fields_and_array_rules_are_refused() {
        for second in [
            RULE.to_owned(),
            RULE.replace("rule-1", "rule-2"),
            RULE.replace("rule-1", "rule-2").replace("allow", "deny"),
        ] {
            let text = format!(r#"{{"schemaVersion":1,"rules":[{RULE},{second}]}}"#);
            assert!(NetworkPolicy::parse(text.as_bytes()).is_err());
        }
        for rule in [
            RULE.replace("443", "0"),
            RULE.replace("443", "65536"),
            RULE.replace("example", "../owner"),
            RULE.replace("tcp", "shell"),
            RULE.replace("\"port\":443", "\"port\":443,\"port\":80"),
            RULE.replace("\"id\"", "\"path\""),
            r#"["rule-1","example","outbound","tcp",443,"allow"]"#.into(),
        ] {
            let text = format!(r#"{{"schemaVersion":1,"rules":[{rule}]}}"#);
            assert!(NetworkPolicy::parse(text.as_bytes()).is_err());
        }
    }
}
