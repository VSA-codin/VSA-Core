import { useEffect, useRef, useState } from "react";
import { useLocalResource } from "../hooks/useLocalResource";
import { SupportReport } from "./SupportReport";
import { api, type Diagnostics } from "../services/api";

export function TrustCenter() {
  const [showPaths, setShowPaths] = useState(false);
  const { data: snapshot, error, reload } = useLocalResource(api.getDiagnostics);
  const [revealed, setRevealed] = useState<Diagnostics | null>(null);
  const request = useRef(0);
  const [pathsLoading, setPathsLoading] = useState(false);
  const [pathsError, setPathsError] = useState(false);
  useEffect(() => () => { ++request.current; }, []);

  async function togglePaths() {
    const current = ++request.current;
    setPathsError(false);
    if (showPaths) {
      setShowPaths(false);
      setRevealed(value => value && ({ ...value, configDirectory: null, dataDirectory: null }));
      return;
    }
    setPathsLoading(true);
    try {
      const result = await api.getDiagnostics(true);
      if (current !== request.current) return;
      setRevealed(result);
      setShowPaths(true);
    } catch {
      if (current === request.current) setPathsError(true);
    } finally {
      if (current === request.current) setPathsLoading(false);
    }
  }

  if (error) return <div><p role="alert">Local diagnostics unavailable.</p><button type="button" onClick={() => void reload()}>Retry diagnostics</button></div>;
  const diagnostics = revealed ?? snapshot;
  if (!diagnostics) return <p role="status">Loading local diagnostics…</p>;
  const rows: [string, string][] = [
    ["Local first", diagnostics.localFirst ? "Implemented: local IPC and settings" : "Unavailable"],
    ["Telemetry collection", diagnostics.telemetryImplemented ? "Implemented" : "Not implemented; no collection"],
    ["Account required", diagnostics.accountRequired ? "Yes" : "No"],
    ["Version", diagnostics.version], ["Runtime", diagnostics.runtime],
    ["Platform", `${diagnostics.platform} / ${diagnostics.architecture}`], ["Build", diagnostics.buildMode],
    ["Config directory", showPaths ? diagnostics.configDirectory ?? "Unavailable" : "Hidden for privacy"], ["Data directory (may not exist yet)", showPaths ? diagnostics.dataDirectory ?? "Unavailable" : "Hidden for privacy"],
    ["Settings load state", diagnostics.settingsLoadState], ["Settings storage", diagnostics.storageStatus], ["Registry", diagnostics.registryStatus],
    ["Enabled / total modules", `${diagnostics.enabledModules} / ${diagnostics.totalModules}`],
    ["Allowed declared permissions", String(diagnostics.allowedPermissions)],
  ];
  return (
    <section className="panel">
      <h2>Implemented</h2>
      <h3>Local diagnostics</h3>
      <p>Diagnostics are generated locally. Directory paths can identify your local user; review before sharing a screenshot.</p>
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
      <SupportReport />
      <h3>Permission foundation</h3>
      <p>Implemented: declared permission metadata and deny by default policy. No grants or module execution are exposed. This logical policy model is not an OS sandbox.</p>
      <h2>Not implemented yet</h2>
      <p>OS sandbox, Vault, encryption, updater, module installation, execution isolation, and log collection.</p>
      <h2>Current limitations</h2>
      <p>Permission decisions describe metadata only; no module execution exists to enforce them. Storage readability does not verify write access. Local paths can identify users when revealed. Same-user filesystem races and cross-process settings coordination are not prevented.</p>
    </section>
  );
}
