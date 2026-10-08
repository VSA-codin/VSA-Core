import { useEffect, useRef, useState } from "react";
import { useLocalResource } from "../hooks/useLocalResource";
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
      setRevealed(null);
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

  const diagnostics = revealed ?? snapshot;
  const rows: [string, string][] = diagnostics ? [
    ["Local First", diagnostics.localFirst ? "Implemented: local IPC and settings" : "Unavailable"],
    ["Telemetry collection", diagnostics.telemetryImplemented ? "Implemented" : "Not implemented; no collection"],
    ["Account required", diagnostics.accountRequired ? "Yes" : "No"],
    ["Version", diagnostics.version], ["Runtime", diagnostics.runtime],
    ["Platform", `${diagnostics.platform} / ${diagnostics.architecture}`], ["Build", diagnostics.buildMode],
    ["Config directory", showPaths ? diagnostics.configDirectory ?? "Unavailable" : "Hidden for privacy"], ["Data directory (may not exist yet)", showPaths ? diagnostics.dataDirectory ?? "Unavailable" : "Hidden for privacy"],
    ["Settings load state", ({ missing: "Missing · defaults in use", loaded: "Loaded", invalid: "Invalid · original preserved", unavailable: "Unavailable or unsafe" } as const)[diagnostics.settingsLoadState]], ["Settings storage", diagnostics.storageStatus], ["Registry", diagnostics.registryStatus],
    ["Enabled / total modules", `${diagnostics.enabledModules} / ${diagnostics.totalModules}`],
    ["Allowed declared permissions", String(diagnostics.allowedPermissions)],
  ] : [];
  return (
    <section className="workspace-page trust-page" aria-label="Trust Center">
      <p className="page-intro">A factual view of this foundation: what exists, what the local core reports, and where its boundaries end.</p>
      <h2>Implemented</h2>
      <div className="trust-facts">
        <div><strong>Local First</strong><p>Bundled interface and local settings. No account required.</p></div>
        <div><strong>No telemetry</strong><p>No analytics, tracking, advertising or report uploads.</p></div>
        <div><strong>Default deny policy</strong><p>Declared permission metadata; no module execution or OS sandbox.</p></div>
      </div>
      <section className="trust-section" aria-labelledby="diagnostics-title">
        <div className="panel-heading"><h2 id="diagnostics-title">Current status</h2><span className="development-badge">Local diagnostics</span></div>
        {error ? <div className="notice"><p role="alert">Local diagnostics unavailable.</p><button type="button" onClick={() => void reload()}>Retry diagnostics</button></div>
          : !diagnostics ? <p role="status">Loading local diagnostics…</p> : <>
        <p>Paths are hidden by default. Revealed paths may identify your user in screenshots; Support Report always excludes them.</p>
        <button type="button" aria-pressed={showPaths} disabled={pathsLoading} onClick={() => void togglePaths()}>
          {pathsLoading ? "Loading local paths…" : showPaths ? "Hide local paths" : "Show local paths"}
        </button>
        {pathsError && <p role="alert">Local paths could not be loaded. They remain hidden.</p>}
        <dl className="diagnostics" aria-label="Local core diagnostics">
          {rows.map(([label, value]) => <div key={label}><dt>{label}</dt><dd>{value}</dd></div>)}
        </dl>
        </>}
      </section>
      <section className="trust-section" aria-labelledby="not-implemented-title">
        <h2 id="not-implemented-title">Not implemented</h2>
        <p>Vault, encryption, OS sandboxing, module installation and execution, automation execution, updater and log collection.</p>
      </section>
      <section className="trust-section" aria-labelledby="review-gates-title">
        <h2 id="review-gates-title">Architecture review gates</h2>
        <dl className="diagnostics">
          <div><dt>Module catalog</dt><dd>Local preview contracts only. Installation, updates and rollback plans remain blocked.</dd></div>
          <div><dt>Signature verification</dt><dd>Metadata parsing only. No trust roots or cryptographic verification.</dd></div>
          <div><dt>CORE updates</dt><dd>Inert version/channel review state. No network downloads or installation.</dd></div>
          <div><dt>Network policy</dt><dd>Default-deny dry-run contracts. No firewall backend or network mutation.</dd></div>
          <div><dt>Windows publisher</dt><dd>Publisher metadata is not Authenticode signing. Unknown-publisher or SmartScreen warnings may appear.</dd></div>
        </dl>
      </section>
      <section className="trust-section" aria-labelledby="limitations-title">
        <h2 id="limitations-title">Known limitations</h2>
        <p>Permission decisions are logical metadata. They do not isolate code or restrict operating system access.</p>
        <p>Settings storage does not prevent attacks by processes running as the same user. Filesystem checks do not eliminate races, parent-directory links or every Windows reparse point. Concurrent app processes are not coordinated.</p>
        <p>Readability does not verify write access. File replacement does not guarantee recovery from a crash or power loss.</p>
      </section>
    </section>
  );
}
