import { useEffect, useRef, useState } from "react";
import { api, type AppSettings } from "../services/api";

export function Settings({ onChange, onBusyChange }: {
  onChange: (settings: AppSettings) => void;
  onBusyChange: (busy: boolean) => void;
}) {
  const request = useRef(0);
  const [dirty, setDirty] = useState(false);
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [recoverable, setRecoverable] = useState(false);
  const [confirmRecovery, setConfirmRecovery] = useState(false);
  const [saved, setSaved] = useState(false);

  async function load() {
    const current = ++request.current;
    setLoading(true);
    setSaved(false);
    setError("");
    setRecoverable(false);
    setConfirmRecovery(false);
    try {
      const result = await api.getSettings();
      if (current !== request.current) return;
      setSettings(result);
      setDirty(false);
      onChange(result);
    } catch {
      if (current !== request.current) return;
      setError("Settings cannot be loaded. Existing files are preserved. Retry or use explicit recovery if available; access errors and unsafe files require manual review.");
      const available = await api.recoveryAvailable().catch(() => false);
      if (current === request.current) setRecoverable(available);
    } finally {
      if (current === request.current) setLoading(false);
    }
  }

  useEffect(() => {
    void load();
    return () => { ++request.current; onBusyChange(false); };
  }, [onBusyChange]);

  async function save() {
    if (!settings || saving) return;
    const current = ++request.current;
    setSaving(true);
    onBusyChange(true);
    setError("");
    setSaved(false);
    try {
      const result = await api.updateSettings(settings);
      if (current !== request.current) return;
      setSettings(result);
      setDirty(false);
      onChange(result);
      setSaved(true);
    } catch {
      if (current !== request.current) return;
      setError("Settings could not be saved. Check directory access or an interrupted temporary write. Existing configuration is preserved.");
    } finally {
      if (current === request.current) { setSaving(false); onBusyChange(false); }
    }
  }

  async function recover() {
    if (saving) return;
    const current = ++request.current;
    setSaving(true);
    onBusyChange(true);
    setError("");
    try {
      const result = await api.recoverSettings();
      if (current !== request.current) return;
      setSettings(result);
      setDirty(false);
      onChange(result);
      setRecoverable(false);
      setConfirmRecovery(false);
      setSaved(true);
    } catch {
      if (current !== request.current) return;
      setError("Recovery did not complete. Review the current settings, recovery backup and any interrupted temporary file before retrying. Existing backups are never overwritten.");
    } finally {
      if (current === request.current) { setSaving(false); onBusyChange(false); }
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
      {saving && <p role="status">Writing local settings. Navigation resumes when the operation finishes.</p>}
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
                  const next = { compactLayout: event.target.checked };
                  setSettings(next);
                  setDirty(true);
                  setSaved(false);
                }}
              />

              <span className="switch-track" aria-hidden="true">
                <span className="switch-thumb" />
              </span>
            </span>
          </label>
          <p>
            <button type="button" disabled={saving || !dirty} onClick={save}>
              {saving ? "Saving…" : "Save settings"}
            </button>
            {" "}<button type="button" disabled={saving} onClick={() => { const next = { compactLayout: false }; setSettings(next); setDirty(true); setSaved(false); }}>Reset preference</button>
          </p>
          <p>{dirty ? "Preference has unsaved changes. Save settings to apply it and keep it after restart." : "Save applies the preference across the interface and keeps it after restart."}</p>
        </>
      )}
      {saved && <p role="status">Settings saved locally.</p>}
    </section>
  );
}
