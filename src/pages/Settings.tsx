import { useEffect, useState } from "react";
import { api, type AppSettings } from "../services/api";

export function Settings({ onChange }: { onChange: (settings: AppSettings) => void }) {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [recoverable, setRecoverable] = useState(false);
  const [confirmRecovery, setConfirmRecovery] = useState(false);
  const [saved, setSaved] = useState(false);

  async function load() {
    setLoading(true);
    setSaved(false);
    setError("");
    setRecoverable(false);
    setConfirmRecovery(false);
    try {
      const result = await api.getSettings();
      setSettings(result);
      onChange(result);
    } catch {
      setError("Settings cannot be loaded. Existing files are preserved. Retry or use explicit recovery if available; access errors and unsafe files require manual review.");
      setRecoverable(await api.recoveryAvailable().catch(() => false));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => { void load(); }, []);

  async function save() {
    if (!settings) return;
    setSaving(true);
    setError("");
    setSaved(false);
    try {
      const result = await api.updateSettings(settings);
      setSettings(result);
      onChange(result);
      setSaved(true);
    } catch {
      setError("Settings could not be saved. Check directory access or an interrupted temporary write. Existing configuration is preserved.");
    } finally {
      setSaving(false);
    }
  }

  async function recover() {
    setSaving(true);
    setError("");
    try {
      const result = await api.recoverSettings();
      setSettings(result);
      onChange(result);
      setRecoverable(false);
      setConfirmRecovery(false);
      setSaved(true);
    } catch {
      setError("Recovery failed. Original settings are preserved. A backup or interrupted temporary file may require manual review before retrying.");
    } finally {
      setSaving(false);
    }
  }

  return (
    <section className="panel">
      <h2>Local settings</h2>
      <p>Stored on this device. No secrets or telemetry configuration.</p>
      {error && <p role="alert">{error}</p>}
      {!settings && error && <button type="button" disabled={saving || loading} onClick={load}>Retry loading settings</button>}
      {!settings && recoverable && (
        <div>
          <p>Recovery saves the original bytes as settings.json.corrupt.bak in the application config directory, then resets compact layout to Off. Existing backups are never overwritten.</p>
          <button type="button" disabled={saving || loading} aria-expanded={confirmRecovery} onClick={() => setConfirmRecovery(value => !value)}>Review settings recovery</button>
          {confirmRecovery && (
            <div role="group" aria-label="Confirm settings recovery">
              <p>Back up the corrupted settings and reset to defaults?</p>
              <button type="button" disabled={saving} onClick={() => void recover()}>{saving ? "Recovering…" : "Back up and reset settings"}</button>{" "}
              <button type="button" disabled={saving} onClick={() => setConfirmRecovery(false)}>Cancel</button>
            </div>
          )}
        </div>
      )}
      {loading && <p role="status">Loading settings…</p>}
      {settings && (
        <>
          <label className={`setting-toggle ${saving ? "disabled" : ""}`}>
            <span className="setting-toggle-copy">
              <strong>Compact layout</strong>
              <small id="compact-description">Use tighter spacing across the VSA CORE interface.</small>
            </span>

            <span className="setting-toggle-action">
              <span className="setting-state">
                {settings.compactLayout ? "On" : "Off"}
              </span>

              <input
                className="switch-input"
                type="checkbox"
                checked={settings.compactLayout}
                disabled={saving}
                aria-label="Compact layout"
                aria-describedby="compact-description"
                onChange={event => {
                  setSettings({ compactLayout: event.target.checked });
                  setSaved(false);
                }}
              />

              <span className="switch-track" aria-hidden="true">
                <span className="switch-thumb" />
              </span>
            </span>
          </label>
          <p>
            <button type="button" disabled={saving} onClick={save}>
              {saving ? "Saving…" : "Save settings"}
            </button>
            {" "}<button type="button" disabled={saving} onClick={() => { setSettings({ compactLayout: false }); setSaved(false); }}>Reset preference</button>
          </p>
          <p>Reset changes the preference above. Save settings to persist it.</p>
        </>
      )}
      {saved && <p role="status">Settings saved locally.</p>}
    </section>
  );
}
