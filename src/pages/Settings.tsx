import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export type AppSettings = { compactLayout: boolean };

export function Settings({ onChange }: { onChange: (settings: AppSettings) => void }) {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [error, setError] = useState("");
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  async function load() {
    setError("");
    try {
      setSettings(await invoke<AppSettings>("get_settings"));
    } catch {
      setError("Settings cannot be loaded. Existing files are preserved. Back up settings.json before repairing it in the application config directory, then retry.");
    }
  }

  useEffect(() => { void load(); }, []);

  async function save() {
    if (!settings) return;
    setSaving(true);
    setError("");
    setSaved(false);
    try {
      const result = await invoke<AppSettings>("update_settings", { settings });
      setSettings(result);
      onChange(result);
      setSaved(true);
    } catch {
      setError("Settings could not be saved. Check directory access or an interrupted temporary write. Existing configuration is preserved.");
    } finally {
      setSaving(false);
    }
  }

  return (
    <section className="panel">
      <h2>Local settings</h2>
      <p>Stored on this device. No secrets or telemetry configuration.</p>
      {error && <p role="alert">{error}</p>}
      {!settings && error && <button type="button" onClick={load}>Retry loading settings</button>}
      {!settings && !error && <p role="status">Loading settings…</p>}
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
          </p>
        </>
      )}
      {saved && <p role="status">Settings saved locally.</p>}
    </section>
  );
}
