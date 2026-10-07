import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Diagnostics = {
  version: string; platform: string; architecture: string; runtime: string; buildMode: string;
  localFirst: boolean; telemetryImplemented: boolean; accountRequired: boolean;
  configDirectory: string; dataDirectory: string; storageStatus: string; registryStatus: string;
  totalModules: number; enabledModules: number; allowedPermissions: number;
};
export function TrustCenter() {
  const [diagnostics, setDiagnostics] = useState<Diagnostics | null>(null);
  const [error, setError] = useState(false);
  useEffect(() => {
    invoke<Diagnostics>("get_diagnostics")
      .then(setDiagnostics)
      .catch(() => setError(true));
  }, []);

  if (error) return <p role="alert">Local diagnostics unavailable.</p>;
  if (!diagnostics) return <p role="status">Loading local diagnostics…</p>;
  const rows: [string, string][] = [
    ["Local first", diagnostics.localFirst ? "Implemented: local IPC and settings" : "Unavailable"],
    ["Telemetry collection", diagnostics.telemetryImplemented ? "Implemented" : "Not implemented; no collection"],
    ["Account required", diagnostics.accountRequired ? "Yes" : "No"],
    ["Version", diagnostics.version], ["Runtime", diagnostics.runtime],
    ["Platform", `${diagnostics.platform} / ${diagnostics.architecture}`], ["Build", diagnostics.buildMode],
    ["Config directory", diagnostics.configDirectory], ["Data directory (may not exist yet)", diagnostics.dataDirectory],
    ["Settings storage", diagnostics.storageStatus], ["Registry", diagnostics.registryStatus],
    ["Enabled / total modules", `${diagnostics.enabledModules} / ${diagnostics.totalModules}`],
    ["Allowed declared permissions", String(diagnostics.allowedPermissions)],
  ];
  return (
    <section className="panel">
      <h2>Local diagnostics</h2>
      <p>This information stays in this view. Directory paths can identify your local user; review before sharing a screenshot.</p>
      <dl className="diagnostics">
        {rows.map(([label, value]) => (
          <div key={label}>
            <dt>{label}</dt>
            <dd>{value}</dd>
          </div>
        ))}
      </dl>
      <h2>Permission foundation</h2>
      <p>Implemented: declared permission metadata and deny by default policy. No grants or module execution are exposed.</p>
      <p>Planned: reviewed module permissions and execution isolation.</p>
      <p>Not implemented yet: OS sandbox, Vault, encryption, updater, module installation, and log collection.</p>
    </section>
  );
}
