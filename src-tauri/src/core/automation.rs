//! Inert workflow contracts. No timers, persistence, dispatch, or execution.
use serde::{Deserialize, Serialize};

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
}

impl AutomationTrigger {
    pub fn validate(self) -> Result<(), &'static str> {
        if let Self::Interval { every_minutes } = self {
            if !(1..=10_080).contains(&every_minutes) {
                return Err("Automation interval must be between 1 minute and 7 days");
            }
        }
        Ok(())
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
}

impl AutomationPlan {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
