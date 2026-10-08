//! Inert workflow contracts. No timers, persistence, dispatch, or execution.
use crate::security::Permission;
use serde::{Deserialize, Serialize};
const MAX_TIMESTAMP: u64 = 253_402_300_799;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum AutomationTrigger {
    // An empty struct variant preserves strict unknown-field rejection.
    Manual {},
    Interval { every_minutes: u32 },
    Once { at_unix_seconds: u64 },
    DailyUtc { hour: u8, minute: u8 },
    WeeklyUtc { weekday: u8, hour: u8, minute: u8 },
}

impl AutomationTrigger {
    pub fn validate(self) -> Result<(), &'static str> {
        if let Self::Interval { every_minutes } = self {
            if !(1..=10_080).contains(&every_minutes) {
                return Err("Automation interval must be between 1 minute and 7 days");
            }
        }
        match self {
            Self::Once { at_unix_seconds } if at_unix_seconds > MAX_TIMESTAMP => {
                return Err("Automation timestamp is invalid")
            }
            Self::DailyUtc { hour, minute } | Self::WeeklyUtc { hour, minute, .. }
                if hour > 23 || minute > 59 =>
            {
                return Err("Automation UTC time is invalid")
            }
            Self::WeeklyUtc { weekday, .. } if weekday > 6 => {
                return Err("Automation weekday is invalid")
            }
            _ => {}
        }
        Ok(())
    }

    /// Preview only. Strictly after `now`; intervals are anchored to explicit metadata.
    /// Calendar triggers use UTC only; weekday 0 is Monday. No clock is read.
    pub fn next_after(self, now: u64, anchor: u64) -> Result<Option<u64>, &'static str> {
        self.validate()?;
        if now > MAX_TIMESTAMP || anchor > MAX_TIMESTAMP {
            return Err("Automation timestamp is invalid");
        }
        let next = match self {
            Self::Manual {} => return Ok(None),
            Self::Once { at_unix_seconds } => {
                return Ok((at_unix_seconds > now).then_some(at_unix_seconds))
            }
            Self::Interval { every_minutes } => {
                let step = u64::from(every_minutes) * 60;
                if anchor > now {
                    anchor
                } else {
                    anchor
                        .checked_add((now - anchor) / step * step)
                        .and_then(|t| t.checked_add(step))
                        .ok_or("Automation timestamp overflow")?
                }
            }
            Self::DailyUtc { hour, minute } => {
                let candidate =
                    now / 86400 * 86400 + u64::from(hour) * 3600 + u64::from(minute) * 60;
                if candidate > now {
                    candidate
                } else {
                    candidate + 86400
                }
            }
            Self::WeeklyUtc {
                weekday,
                hour,
                minute,
            } => {
                let day = now / 86400;
                let current_weekday = (day + 3) % 7; // 1970-01-01 was Thursday.
                let delta = (u64::from(weekday) + 7 - current_weekday) % 7;
                let candidate =
                    (day + delta) * 86400 + u64::from(hour) * 3600 + u64::from(minute) * 60;
                if candidate > now {
                    candidate
                } else {
                    candidate + 7 * 86400
                }
            }
        };
        if next > MAX_TIMESTAMP {
            return Err("Automation timestamp overflow");
        }
        Ok(Some(next))
    }
}

/// A planned module-defined action, never an executable or command string.
/// Plans are disabled because CORE has no action registry or executor.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomationPlan {
    id: String,
    name: String,
    module_id: String,
    action_id: String,
    trigger: AutomationTrigger,
    enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    declared_permissions: Vec<Permission>,
}

impl AutomationPlan {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        struct Fields {
            id: String,
            name: String,
            module_id: String,
            action_id: String,
            trigger: AutomationTrigger,
            enabled: bool,
            #[serde(default)]
            declared_permissions: Vec<Permission>,
        }
        let f: Fields = super::foundation::parse_object(bytes)?;
        if f.enabled {
            return Err("Automation execution is unavailable");
        }
        for (index, p) in f.declared_permissions.iter().enumerate() {
            if f.declared_permissions[..index].contains(p) {
                return Err("Automation permission is duplicated");
            }
        }
        let mut plan = Self::new(f.id, f.name, f.module_id, f.action_id, f.trigger)?;
        plan.declared_permissions = f.declared_permissions;
        Ok(plan)
    }

    pub fn new(
        id: String,
        name: String,
        module_id: String,
        action_id: String,
        trigger: AutomationTrigger,
    ) -> Result<Self, &'static str> {
        for identifier in [&id, &module_id, &action_id] {
            super::module_registry::validate_identifier(identifier)?;
        }
        if name.trim().is_empty()
            || name.len() > 128
            || super::module_registry::has_unsafe_display_characters(&name)
        {
            return Err(
                "Automation name must be nonblank, bounded, and contain no control or directional formatting characters",
            );
        }
        trigger.validate()?;
        Ok(Self {
            id,
            name,
            module_id,
            action_id,
            trigger,
            enabled: false,
            declared_permissions: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_schedule_preview_is_deterministic_and_strictly_future() {
        assert_eq!(AutomationTrigger::Manual {}.next_after(0, 0).unwrap(), None);
        assert_eq!(
            AutomationTrigger::Once {
                at_unix_seconds: 60
            }
            .next_after(60, 0)
            .unwrap(),
            None
        );
        assert_eq!(
            AutomationTrigger::Once {
                at_unix_seconds: 60
            }
            .next_after(59, 0)
            .unwrap(),
            Some(60)
        );
        assert_eq!(
            AutomationTrigger::Interval { every_minutes: 1 }
                .next_after(60, 0)
                .unwrap(),
            Some(120)
        );
        assert_eq!(
            AutomationTrigger::Interval { every_minutes: 1 }
                .next_after(10, 30)
                .unwrap(),
            Some(30)
        );
        assert_eq!(
            AutomationTrigger::DailyUtc { hour: 0, minute: 0 }
                .next_after(0, 0)
                .unwrap(),
            Some(86400)
        );
        assert_eq!(
            AutomationTrigger::WeeklyUtc {
                weekday: 0,
                hour: 0,
                minute: 0
            }
            .next_after(0, 0)
            .unwrap(),
            Some(4 * 86400)
        );
        assert_eq!(
            AutomationTrigger::WeeklyUtc {
                weekday: 3,
                hour: 0,
                minute: 0
            }
            .next_after(0, 0)
            .unwrap(),
            Some(7 * 86400)
        );
        for trigger in [
            AutomationTrigger::DailyUtc {
                hour: 24,
                minute: 0,
            },
            AutomationTrigger::DailyUtc {
                hour: 1,
                minute: 60,
            },
            AutomationTrigger::WeeklyUtc {
                weekday: 7,
                hour: 0,
                minute: 0,
            },
            AutomationTrigger::Once {
                at_unix_seconds: u64::MAX,
            },
        ] {
            assert!(trigger.next_after(0, 0).is_err());
        }
        assert!(AutomationTrigger::Interval { every_minutes: 1 }
            .next_after(MAX_TIMESTAMP, 0)
            .is_err());
    }
    #[test]
    fn serialized_plans_reject_execution_and_permission_ambiguity() {
        let bytes = serde_json::to_vec(&plan(AutomationTrigger::Manual {}).unwrap()).unwrap();
        assert!(AutomationPlan::parse(&bytes).is_ok());
        let valid = String::from_utf8(bytes).unwrap();
        for text in [
            valid.replace("false", "true"),
            valid.replace(
                "\"enabled\":false",
                "\"enabled\":false,\"declaredPermissions\":[\"network\",\"network\"]",
            ),
            valid.replace("\"enabled\":false", "\"enabled\":false,\"command\":\"run\""),
            valid.replace("\"id\":\"example\"", "\"id\":\"example\",\"id\":\"other\""),
            "[]".into(),
        ] {
            assert!(AutomationPlan::parse(text.as_bytes()).is_err(), "{text}");
        }
    }
    fn plan(trigger: AutomationTrigger) -> Result<AutomationPlan, &'static str> {
        AutomationPlan::new(
            "example".into(),
            "Example".into(),
            "vsa-asf".into(),
            "status".into(),
            trigger,
        )
    }

    #[test]
    fn valid_plans_are_inert_and_deterministic() {
        for trigger in [
            AutomationTrigger::Manual {},
            AutomationTrigger::Interval { every_minutes: 1 },
            AutomationTrigger::Interval {
                every_minutes: 10_080,
            },
        ] {
            let bytes = serde_json::to_vec(&plan(trigger).unwrap()).unwrap();
            assert_eq!(bytes, serde_json::to_vec(&plan(trigger).unwrap()).unwrap());
            let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(json["enabled"], false);
            assert_eq!(json.as_object().unwrap().len(), 6);
            assert_eq!(
                serde_json::from_value::<AutomationTrigger>(json["trigger"].clone()).unwrap(),
                trigger
            );
        }
    }

    #[test]
    fn malformed_schedule_and_action_metadata_are_rejected() {
        for every_minutes in [0, 10_081, u32::MAX] {
            assert!(plan(AutomationTrigger::Interval { every_minutes }).is_err());
        }
        for input in [
            r#"{"kind":"shell","command":"run"}"#,
            r#"{"kind":"manual","command":"run"}"#,
            r#"{"kind":"interval","everyMinutes":1,"everyMinutes":2}"#,
            r#"{"kind":"manual","kind":"interval","everyMinutes":1}"#,
            r#"{"kind":"interval","kind":"manual","everyMinutes":1}"#,
            r#"{"kind":"interval","everyMinutes":-1}"#,
            r#"{"kind":"interval","everyMinutes":1.5}"#,
            r#"{"kind":"interval"}"#,
            "[]",
        ] {
            assert!(
                serde_json::from_str::<AutomationTrigger>(input).is_err(),
                "{input}"
            );
        }
        for bad in [
            "../escape".into(),
            "UPPER".into(),
            "".into(),
            "x".repeat(65),
        ] {
            for index in 0..3 {
                let mut ids = ["example".to_owned(), "vsa-asf".into(), "status".into()];
                ids[index] = bad.clone();
                let [id, module, action] = ids;
                assert!(AutomationPlan::new(
                    id,
                    "Example".into(),
                    module,
                    action,
                    AutomationTrigger::Manual {}
                )
                .is_err());
            }
        }
        for name in [
            " ".into(),
            "bad\nname".into(),
            "bad\u{202e}name".into(),
            "é".repeat(65),
        ] {
            assert!(AutomationPlan::new(
                "example".into(),
                name,
                "vsa-asf".into(),
                "status".into(),
                AutomationTrigger::Manual {}
            )
            .is_err());
        }
    }
}

/// Bounded retry preview metadata. No retry is dispatched.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicy {
    max_attempts: u8,
    initial_delay_seconds: u32,
}
impl RetryPolicy {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        struct Fields {
            max_attempts: u8,
            initial_delay_seconds: u32,
        }
        let f: Fields = super::foundation::parse_object(bytes)?;
        if !(1..=5).contains(&f.max_attempts) || !(1..=3600).contains(&f.initial_delay_seconds) {
            return Err("Retry policy is invalid");
        }
        Ok(Self {
            max_attempts: f.max_attempts,
            initial_delay_seconds: f.initial_delay_seconds,
        })
    }
    pub fn delay_before_attempt(&self, attempt: u8) -> Result<Option<u32>, &'static str> {
        if attempt == 0 {
            return Err("Retry attempt is invalid");
        }
        if attempt == 1 || attempt > self.max_attempts {
            return Ok(None);
        }
        Ok(Some(
            (self.initial_delay_seconds * (1 << (attempt - 2))).min(3600),
        ))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkippedRunMetadata {
    plan_id: String,
    scheduled_at_unix_seconds: u64,
    state: &'static str,
}
impl SkippedRunMetadata {
    /// Preview of a skipped run only; never evidence that an action ran.
    pub fn new(plan_id: String, scheduled_at_unix_seconds: u64) -> Result<Self, &'static str> {
        super::module_registry::validate_identifier(&plan_id)?;
        if scheduled_at_unix_seconds > MAX_TIMESTAMP {
            return Err("Automation timestamp is invalid");
        }
        Ok(Self {
            plan_id,
            scheduled_at_unix_seconds,
            state: "skippedExecutionUnavailable",
        })
    }
}
#[derive(Debug, Default, Serialize)]
#[serde(transparent)]
pub struct RunHistoryPreview(std::collections::VecDeque<SkippedRunMetadata>);
impl RunHistoryPreview {
    pub fn push(&mut self, record: SkippedRunMetadata) {
        if self.0.len() == 100 {
            self.0.pop_front();
        }
        self.0.push_back(record);
    }
}

#[cfg(test)]
mod history_tests {
    use super::*;
    #[test]
    fn retry_bounds_and_backoff_are_deterministic() {
        let policy =
            RetryPolicy::parse(br#"{"maxAttempts":5,"initialDelaySeconds":1000}"#).unwrap();
        assert_eq!(policy.delay_before_attempt(1).unwrap(), None);
        assert_eq!(policy.delay_before_attempt(2).unwrap(), Some(1000));
        assert_eq!(policy.delay_before_attempt(3).unwrap(), Some(2000));
        assert_eq!(policy.delay_before_attempt(4).unwrap(), Some(3600));
        assert_eq!(policy.delay_before_attempt(6).unwrap(), None);
        assert!(policy.delay_before_attempt(0).is_err());
        for text in [
            r#"{"maxAttempts":0,"initialDelaySeconds":1}"#,
            r#"{"maxAttempts":6,"initialDelaySeconds":1}"#,
            r#"{"maxAttempts":1,"initialDelaySeconds":0}"#,
            r#"{"maxAttempts":1,"initialDelaySeconds":3601}"#,
            r#"{"maxAttempts":1,"maxAttempts":2,"initialDelaySeconds":1}"#,
            "[]",
        ] {
            assert!(RetryPolicy::parse(text.as_bytes()).is_err());
        }
    }
    #[test]
    fn history_is_bounded_inert_and_contains_no_arbitrary_error_text() {
        let mut history = RunHistoryPreview::default();
        for t in 0..101 {
            history.push(SkippedRunMetadata::new("example".into(), t).unwrap());
        }
        assert_eq!(history.0.len(), 100);
        assert_eq!(history.0.front().unwrap().scheduled_at_unix_seconds, 1);
        assert!(history
            .0
            .iter()
            .all(|r| r.state == "skippedExecutionUnavailable"));
        assert!(SkippedRunMetadata::new("../escape".into(), 0).is_err());
        assert!(SkippedRunMetadata::new("example".into(), u64::MAX).is_err());
    }
}
