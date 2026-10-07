use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "filesystem.read")]
    FilesystemRead,
    #[serde(rename = "filesystem.write")]
    FilesystemWrite,
    #[serde(rename = "process.execute")]
    ProcessExecute,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "secrets.read")]
    SecretsRead,
    #[serde(rename = "clipboard")]
    Clipboard,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionDecision {
    Allow,
    Deny,
}

// Policy metadata only. No execution path or OS sandbox exists.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct PermissionPolicy(BTreeMap<Permission, PermissionDecision>);
// Reject ambiguous duplicate decisions instead of accepting the last value.
impl<'de> Deserialize<'de> for PermissionPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PolicyVisitor;
        impl<'de> serde::de::Visitor<'de> for PolicyVisitor {
            type Value = PermissionPolicy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a permission decision object with unique keys")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut map: M,
            ) -> Result<Self::Value, M::Error> {
                let mut decisions = BTreeMap::new();
                while let Some((permission, decision)) =
                    map.next_entry::<Permission, PermissionDecision>()?
                {
                    if decisions.insert(permission, decision).is_some() {
                        return Err(serde::de::Error::custom("duplicate permission decision"));
                    }
                }
                Ok(PermissionPolicy(decisions))
            }
        }
        deserializer.deserialize_map(PolicyVisitor)
    }
}
impl PermissionPolicy {
    pub fn decision(&self, permission: Permission, declared: &[Permission]) -> PermissionDecision {
        if !declared.contains(&permission) {
            return PermissionDecision::Deny;
        }
        self.0
            .get(&permission)
            .copied()
            .unwrap_or(PermissionDecision::Deny)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ambiguous_and_malformed_policy_is_rejected() {
        for input in [
            r#"{"network":"deny","network":"allow"}"#,
            r#"{"network":"allow","network":"deny"}"#,
            r#"{"network":"deny","net\u0077ork":"allow"}"#,
            r#"{"unknown":"allow"}"#,
            r#"{"network":true}"#,
            r#"{"network":"unknown"}"#,
            "[]",
            "null",
        ] {
            assert!(
                serde_json::from_str::<PermissionPolicy>(input).is_err(),
                "{input}"
            );
        }
    }
    #[test]
    fn policy_serialization_order_is_stable_and_revoked_declarations_deny() {
        let first: PermissionPolicy = serde_json::from_str(
            r#"{"network":"allow","clipboard":"deny","filesystem.read":"allow"}"#,
        )
        .unwrap();
        let second: PermissionPolicy = serde_json::from_str(
            r#"{"filesystem.read":"allow","clipboard":"deny","network":"allow"}"#,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_vec(&first).unwrap(),
            serde_json::to_vec(&second).unwrap()
        );
        assert_eq!(
            first.decision(Permission::Network, &[Permission::Network]),
            PermissionDecision::Allow
        );
        assert_eq!(
            first.decision(Permission::Network, &[Permission::FilesystemRead]),
            PermissionDecision::Deny
        );
    }
    #[test]
    fn every_permission_requires_declaration_and_explicit_allow() {
        for permission in [
            Permission::Network,
            Permission::FilesystemRead,
            Permission::FilesystemWrite,
            Permission::ProcessExecute,
            Permission::Notifications,
            Permission::SecretsRead,
            Permission::Clipboard,
        ] {
            assert_eq!(
                PermissionPolicy::default().decision(permission, &[permission]),
                PermissionDecision::Deny
            );
            let allowed =
                PermissionPolicy(BTreeMap::from([(permission, PermissionDecision::Allow)]));
            assert_eq!(allowed.decision(permission, &[]), PermissionDecision::Deny);
            assert_eq!(
                allowed.decision(permission, &[permission]),
                PermissionDecision::Allow
            );
            let denied = PermissionPolicy(BTreeMap::from([(permission, PermissionDecision::Deny)]));
            assert_eq!(
                denied.decision(permission, &[permission]),
                PermissionDecision::Deny
            );
        }
    }
    #[test]
    fn default_unset_and_undeclared_permissions_are_denied() {
        let policy = PermissionPolicy::default();
        assert_eq!(
            policy.decision(Permission::Network, &[Permission::Network]),
            PermissionDecision::Deny
        );
        assert_eq!(
            policy.decision(Permission::Clipboard, &[]),
            PermissionDecision::Deny
        );
        let granted: PermissionPolicy = serde_json::from_str(r#"{"network":"allow"}"#).unwrap();
        assert_eq!(
            granted.decision(Permission::Network, &[]),
            PermissionDecision::Deny
        );
    }
    #[test]
    fn explicit_decisions_and_serialization() {
        let policy: PermissionPolicy =
            serde_json::from_str(r#"{"network":"allow","clipboard":"deny"}"#).unwrap();
        assert_eq!(
            policy.decision(Permission::Network, &[Permission::Network]),
            PermissionDecision::Allow
        );
        assert_eq!(
            policy.decision(Permission::Clipboard, &[Permission::Clipboard]),
            PermissionDecision::Deny
        );
        let encoded = serde_json::to_string(&policy).unwrap();
        let roundtrip: PermissionPolicy = serde_json::from_str(&encoded).unwrap();
        assert_eq!(
            roundtrip.decision(Permission::Network, &[Permission::Network]),
            PermissionDecision::Allow
        );
        assert!(serde_json::from_str::<PermissionPolicy>(r#"{"unknown":"allow"}"#).is_err());
    }
}
