# Automation metadata and scheduling previews

Status: implemented inert SDK contracts; no runtime scheduling or action execution.

Plans accept strict bounded object JSON: `id`, `name`, `moduleId`, `actionId`, `trigger`, required `enabled: false`, optional unique known `declaredPermissions`. IDs are restricted lowercase ASCII identities, names are bounded/nonblank, unknown/duplicate fields fail. There is no command, script, path, URL, secret or arbitrary argument field. Parsing does not prove the referenced module/action exists or grant permissions.

| Trigger kind | Fields / validation |
| --- | --- |
| manual | No extra fields; next-run preview is None |
| interval | `everyMinutes`, integer 1–10,080; explicit caller-supplied anchor |
| once | `atUnixSeconds`, integer 0–253402300799 (end of year 9999) |
| dailyUtc | `hour` 0–23, `minute` 0–59 |
| weeklyUtc | `weekday` 0–6 (Monday–Sunday), hour/minute as above |

`next_after(now, anchor)` uses only caller-supplied Unix seconds, returns a strictly later candidate or None, checks bounds, skips past one-time runs, and never reads a clock or starts a timer. Interval schedules preserve the explicit anchor and advance beyond missed slots; calendar previews use UTC and Gregorian epoch-day arithmetic. Leap seconds, local wall time, IANA timezones and DST are not supported. UTC is in the trigger identity; local zones must not be silently interpreted as UTC. Review local timezone/DST gaps/folds, missed-run policy and monotonic/runtime sleep behavior before an engine exists.

Separate RetryPolicy preview has 1–5 maximum attempts and 1–3600 initial delay seconds. Attempt 1 has no delay; later delays double and cap at 3600. Attempts beyond the maximum have no retry. This contract neither retries nor grants permission.

RunHistoryPreview retains at most 100 caller-constructed skipped-run metadata records in memory, evicting the oldest insertion. Records have validated plan IDs, bounded scheduled timestamps and the fixed state `skippedExecutionUnavailable`. They are previews, not observed run history. No arbitrary exception text, module result, timestamp collection or persisted log exists.

Before dispatch: verify module identity/installation/compatibility, action registration, declaration and explicit resource-scoped user grants; keep new plans disabled. Define cancellation, overlap, concurrent processes, sleep/restart, recovery, per-profile isolation, retries for idempotent actions, bounded actual history and headless broker parity. No shell/process/module execution, OS task registration, networking or external side effect is implemented.
