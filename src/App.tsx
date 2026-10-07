import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type CoreStatus = {
  name: string;
  version: string;
  runtime: string;
  mode: string;
  privacy: string;
  totalModules: number;
  enabledModules: number;
};

type ModuleDescriptor = { id: string; name: string; description: string; version: string | null; lifecycle: "planned" | "available" | "installed" | "enabled" };

const navigation = [
  "Dashboard",
  "Modules",
  "Automation",
  "Vault",
  "Trust Center",
  "Settings",
];

function App() {
  const [page, setPage] = useState("Dashboard");
  const [modules, setModules] = useState<ModuleDescriptor[] | null>(null);
  const [modulesError, setModulesError] = useState(false);
  const [coreStatus, setCoreStatus] = useState<CoreStatus | null>(null);
  const [coreError, setCoreError] = useState(false);

  useEffect(() => {
    invoke<ModuleDescriptor[]>("get_modules").then(setModules).catch(() => setModulesError(true));
    invoke<CoreStatus>("get_core_status")
      .then((status) => {
        setCoreStatus(status);
        setCoreError(false);
      })
      .catch(() => {
        setCoreError(true);
      });
  }, []);

  const isOnline = coreStatus !== null && !coreError;

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">V</div>

          <div>
            <div className="brand-name">{coreStatus?.name ?? "VSA CORE"}</div>
            <div className="brand-version">
              {coreStatus ? `v${coreStatus.version}` : "Version unavailable"}
            </div>
          </div>
        </div>

        <nav className="navigation">
          {navigation.map((item) => (
            <button
              className={`nav-item ${page === item ? "active" : ""}`}
              onClick={() => setPage(item)}
              aria-current={page === item ? "page" : undefined}
              key={item}
              type="button"
            >
              {item}
            </button>
          ))}
        </nav>

        <div className="sidebar-footer">
          <span className="status-dot" />
          {coreStatus ? `${coreStatus.mode} mode` : "Connecting..."}
        </div>
      </aside>

      <main className="main-content">
        <header className="topbar">
          <div>
            <p className="eyebrow">VSA SOFTWARE ECOSYSTEM</p>
            <h1>{page}</h1>
          </div>

          <div className="core-status">
            <span className="status-dot" />
            {coreError ? "Core unavailable" : isOnline ? "Core online" : "Connecting..."}
          </div>
        </header>

        {page === "Dashboard" && <>
        <section className="hero-card">
          <div>
            <p className="eyebrow">WELCOME TO</p>
            <h2>{coreStatus?.name ?? "VSA CORE"}</h2>
            <p className="hero-copy">
              Privacy-first infrastructure for VSA applications, modules,
              automation and self-hosted services.
            </p>
          </div>

          <div className="hero-badge">
            <span className="status-dot" />
            {coreStatus?.privacy ?? "Local First"}
          </div>
        </section>

        <section className="status-grid">
          <article className="status-card">
            <span className="card-label">CORE STATUS</span>
            <strong>{isOnline ? "Operational" : coreError ? "Unavailable" : "Starting"}</strong>
            <p>
              {isOnline
                ? "The local core status command responded."
                : coreError
                  ? "The local VSA CORE backend could not be reached."
                  : "Connecting to the local VSA CORE backend."}
            </p>
          </article>

          <article className="status-card">
            <span className="card-label">PRIVACY</span>
            <strong>Local First</strong>
            <p>No hidden telemetry. Your data stays local.</p>
          </article>

          <article className="status-card">
            <span className="card-label">MODULES</span>
            <strong>{coreStatus ? `${coreStatus.enabledModules} / ${coreStatus.totalModules} enabled` : "Unavailable"}</strong>
            <p>Roadmap metadata only; no module execution.</p>
          </article>
        </section>

        <section className="panel">
          <div className="panel-heading">
            <div>
              <p className="eyebrow">SYSTEM</p>
              <h3>VSA CORE foundation</h3>
            </div>

            <span className="development-badge">Early Development</span>
          </div>

          <div className="foundation-grid">
            <div>
              <span>Runtime</span>
              <strong>{coreStatus?.runtime ?? "Tauri 2 + Rust"}</strong>
            </div>

            <div>
              <span>Interface</span>
              <strong>React + TypeScript</strong>
            </div>

            <div>
              <span>Account</span>
              <strong>Not required</strong>
            </div>

            <div>
              <span>Data mode</span>
              <strong>{coreStatus?.privacy ?? "Local First"}</strong>
            </div>
          </div>
        </section>
        </>}
        {page === "Modules" && <section aria-label="Module registry">
          <p>Roadmap metadata only. Installation and execution are not implemented yet.</p>
          {modulesError ? <p role="alert">Module registry unavailable.</p> : modules === null ? <p role="status">Loading modules…</p> : modules.map(module => <article className="panel" key={module.id}>
            <h2>{module.name}</h2><p>{module.description}</p>
            <p>Status: {module.lifecycle} · Installed: {module.lifecycle === "installed" || module.lifecycle === "enabled" ? "Yes" : "No"} · Enabled: {module.lifecycle === "enabled" ? "Yes" : "No"}</p>
            {module.version && <p>Version: {module.version}</p>}
          </article>)}
        </section>}
        {page === "Trust Center" && <section className="panel"><h2>Current foundations</h2><p>Local first. No account required. No telemetry collection implemented.</p><p>Registry: {coreStatus ? `${coreStatus.enabledModules} / ${coreStatus.totalModules} enabled` : "Unavailable"}</p><p>Module execution, sandboxing, encryption, and secret storage: not implemented yet.</p></section>}
        {["Automation", "Vault", "Settings"].includes(page) && <section className="panel"><h2>{page}</h2><p>Not implemented yet.</p></section>}
      </main>
    </div>
  );
}

export default App;
