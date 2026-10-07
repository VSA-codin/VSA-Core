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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PermissionPolicy(BTreeMap<Permission, PermissionDecision>);
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
