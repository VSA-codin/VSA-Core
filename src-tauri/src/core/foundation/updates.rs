use super::{trust::lower_hex, version::ReleaseVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateChannel {
    Stable,
    Beta,
    Nightly,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateState {
    Idle,
    ReviewRequired,
    Blocked,
    Failed,
    RecoveryRequired,
}

/// No URL or local executable path. A future resolver must use a reviewed source.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    schema_version: u32,
    version: ReleaseVersion,
    channel: UpdateChannel,
    artifact_id: String,
    size_bytes: u64,
    sha256: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fields {
    schema_version: u32,
    version: ReleaseVersion,
    channel: UpdateChannel,
    artifact_id: String,
    size_bytes: u64,
    sha256: String,
}
impl UpdateMetadata {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let f: Fields = super::parse_object(bytes)?;
        if f.schema_version != 1 {
            return Err("Update schema is unsupported");
        }
        super::super::module_registry::validate_identifier(&f.artifact_id)?;
        if f.size_bytes == 0 || f.size_bytes > 2 * 1024 * 1024 * 1024 || !lower_hex(&f.sha256, 64) {
            return Err("Update integrity metadata is invalid");
        }
        Ok(Self {
            schema_version: 1,
            version: f.version,
            channel: f.channel,
            artifact_id: f.artifact_id,
            size_bytes: f.size_bytes,
            sha256: f.sha256,
        })
    }
    pub fn preview(&self, current: ReleaseVersion, channel: UpdateChannel) -> UpdateState {
        if self.version <= current || self.channel != channel {
            UpdateState::Blocked
        } else {
            UpdateState::ReviewRequired
        }
    }
}

/// An inert review state machine. There is intentionally no downloaded/verified/applied state.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReview {
    state: UpdateState,
}
impl Default for UpdateReview {
    fn default() -> Self {
        Self {
            state: UpdateState::Idle,
        }
    }
}
impl UpdateReview {
    pub fn state(&self) -> UpdateState {
        self.state
    }
    pub fn inspect(
        &mut self,
        metadata: &UpdateMetadata,
        current: ReleaseVersion,
        channel: UpdateChannel,
    ) -> Result<(), &'static str> {
        if self.state != UpdateState::Idle {
            return Err("Update review transition is invalid");
        }
        self.state = metadata.preview(current, channel);
        Ok(())
    }
    pub fn fail(&mut self) -> Result<(), &'static str> {
        if self.state != UpdateState::ReviewRequired {
            return Err("Update review transition is invalid");
        }
        self.state = UpdateState::Failed;
        Ok(())
    }
    pub fn require_recovery(&mut self) -> Result<(), &'static str> {
        if self.state != UpdateState::Failed {
            return Err("Update review transition is invalid");
        }
        self.state = UpdateState::RecoveryRequired;
        Ok(())
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollbackPreview {
    previous_version: ReleaseVersion,
    preserve_user_data: bool,
    owner_review_required: bool,
}
impl RollbackPreview {
    pub fn new(current: ReleaseVersion, previous: ReleaseVersion) -> Result<Self, &'static str> {
        if previous >= current {
            return Err("Rollback version must be older");
        }
        Ok(Self {
            previous_version: previous,
            preserve_user_data: true,
            owner_review_required: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn metadata() -> String {
        format!(
            r#"{{"schemaVersion":1,"version":"1.2.3","channel":"stable","artifactId":"windows-x64","sizeBytes":123,"sha256":"{}"}}"#,
            "a".repeat(64)
        )
    }
    #[test]
    fn update_preview_blocks_downgrades_and_channel_changes() {
        let m = UpdateMetadata::parse(metadata().as_bytes()).unwrap();
        let current = ReleaseVersion::parse("1.0.0").unwrap();
        assert_eq!(
            m.preview(current, UpdateChannel::Stable),
            UpdateState::ReviewRequired
        );
        assert_eq!(
            m.preview(current, UpdateChannel::Beta),
            UpdateState::Blocked
        );
        assert_eq!(
            m.preview(m.version, UpdateChannel::Stable),
            UpdateState::Blocked
        );
        assert_eq!(
            m.preview(
                ReleaseVersion::parse("2.0.0").unwrap(),
                UpdateChannel::Stable
            ),
            UpdateState::Blocked
        );
        assert!(UpdateMetadata::parse(&serde_json::to_vec(&m).unwrap()).is_ok());
        let rollback = RollbackPreview::new(m.version, current).unwrap();
        assert!(rollback.preserve_user_data && rollback.owner_review_required);
        assert!(RollbackPreview::new(current, m.version).is_err());
    }
    #[test]
    fn review_transitions_are_inert_and_guarded() {
        let m = UpdateMetadata::parse(metadata().as_bytes()).unwrap();
        let mut review = UpdateReview::default();
        assert!(review.fail().is_err());
        assert!(review.require_recovery().is_err());
        review
            .inspect(
                &m,
                ReleaseVersion::parse("1.0.0").unwrap(),
                UpdateChannel::Stable,
            )
            .unwrap();
        assert!(review
            .inspect(&m, m.version, UpdateChannel::Stable)
            .is_err());
        review.fail().unwrap();
        review.require_recovery().unwrap();
        assert_eq!(review.state(), UpdateState::RecoveryRequired);
        assert!(review.fail().is_err());
    }
    #[test]
    fn update_input_is_bounded_strict_and_has_no_execution_location() {
        for text in [
            metadata().replace("123", "0"),
            metadata().replace("123", "2147483649"),
            metadata().replace("stable", "other"),
            metadata().replace("windows-x64", "../payload"),
            metadata().replace(&"a".repeat(64), "bad"),
            metadata().replace("\"schemaVersion\":1", "\"schemaVersion\":2"),
            metadata().replace("\"sizeBytes\":123", "\"sizeBytes\":123,\"sizeBytes\":124"),
            metadata().replace("\"artifactId\"", "\"url\""),
            "[]".into(),
        ] {
            assert!(UpdateMetadata::parse(text.as_bytes()).is_err());
        }
    }
}
