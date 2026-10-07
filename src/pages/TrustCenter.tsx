import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Diagnostics = {
  version: string; platform: string; architecture: string; runtime: string; buildMode: string;
  localFirst: boolean; telemetryImplemented: boolean; accountRequired: boolean;
  configDirectory: string | null; dataDirectory: string | null; storageStatus: string; registryStatus: string;
  totalModules: number; enabledModules: number; allowedPermissions: number;
};
export function TrustCenter() {
  const [showPaths, setShowPaths] = useState(false);
  const [diagnostics, setDiagnostics] = useState<Diagnostics | null>(null);
  const [pathsLoading, setPathsLoading] = useState(false);
  const [pathsError, setPathsError] = useState(false);
  const [error, setError] = useState(false);
  useEffect(() => {
    invoke<Diagnostics>("get_diagnostics")
      .then(setDiagnostics)
      .catch(() => setError(true));
  }, []);

  async function togglePaths() {
    setPathsError(false);
    if (showPaths) {
      setShowPaths(false);
      setDiagnostics(value => value && ({ ...value, configDirectory: null, dataDirectory: null }));
      return;
    }
    setPathsLoading(true);
    try {
      setDiagnostics(await invoke<Diagnostics>("get_diagnostics", { includePaths: true }));
      setShowPaths(true);
    } catch {
      setPathsError(true);
    } finally {
      setPathsLoading(false);
    }
  }

  if (error) return <p role="alert">Local diagnostics unavailable.</p>;
  if (!diagnostics) return <p role="status">Loading local diagnostics…</p>;
  const rows: [string, string][] = [
    ["Local first", diagnostics.localFirst ? "Implemented: local IPC and settings" : "Unavailable"],
    ["Telemetry collection", diagnostics.telemetryImplemented ? "Implemented" : "Not implemented; no collection"],
    ["Account required", diagnostics.accountRequired ? "Yes" : "No"],
    ["Version", diagnostics.version], ["Runtime", diagnostics.runtime],
    ["Platform", `${diagnostics.platform} / ${diagnostics.architecture}`], ["Build", diagnostics.buildMode],
    ["Config directory", showPaths ? diagnostics.configDirectory ?? "Unavailable" : "Hidden for privacy"], ["Data directory (may not exist yet)", showPaths ? diagnostics.dataDirectory ?? "Unavailable" : "Hidden for privacy"],
    ["Settings storage", diagnostics.storageStatus], ["Registry", diagnostics.registryStatus],
    ["Enabled / total modules", `${diagnostics.enabledModules} / ${diagnostics.totalModules}`],
    ["Allowed declared permissions", String(diagnostics.allowedPermissions)],
  ];
  return (
    <section className="panel">
      <h2>Local diagnostics</h2>
      <p>This information stays in this view. Directory paths can identify your local user; review before sharing a screenshot.</p>
      <button type="button" aria-pressed={showPaths} disabled={pathsLoading} onClick={() => void togglePaths()}>
        {pathsLoading ? "Loading local paths…" : showPaths ? "Hide local paths" : "Show local paths"}
      </button>
      {pathsError && <p role="alert">Local paths could not be loaded. They remain hidden.</p>}
      <dl className="diagnostics">
        {rows.map(([label, value]) => (
          <div key={label}>
            <dt>{label}</dt>
            <dd>{value}</dd>
          </div>
        ))}
      </dl>
      <h2>Permission foundation</h2>
      <p>Implemented: declared permission metadata and deny by default policy. No grants or module execution are exposed. This logical policy model is not an OS sandbox.</p>
      <p>Planned: reviewed module permissions and execution isolation.</p>
      <p>Not implemented yet: OS sandbox, Vault, encryption, updater, module installation, and log collection.</p>
    </section>
  );
}
