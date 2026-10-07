import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type CoreStatus = {
  name: string;
  version: string;
  runtime: string;
  mode: string;
  privacy: string;
};

const navigation = [
  "Dashboard",
  "Modules",
  "Automation",
  "Vault",
  "Trust Center",
  "Settings",
];

function App() {
  const [coreStatus, setCoreStatus] = useState<CoreStatus | null>(null);
  const [coreError, setCoreError] = useState(false);

  useEffect(() => {
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
              v{coreStatus?.version ?? "0.1.0"}
            </div>
          </div>
        </div>

        <nav className="navigation">
          {navigation.map((item, index) => (
            <button
              className={`nav-item ${index === 0 ? "active" : ""}`}
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
            <h1>Dashboard</h1>
          </div>

          <div className="core-status">
            <span className="status-dot" />
            {coreError ? "Core unavailable" : isOnline ? "Core online" : "Connecting..."}
          </div>
        </header>

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
                ? "All local core services are available."
                : coreError
                  ? "The local VSA CORE backend could not be reached."
                  : "Connecting to the local VSA CORE backend."}
            </p>
          </article>

          <article className="status-card">
            <span className="card-label">PRIVACY</span>
            <strong>Protected</strong>
            <p>No hidden telemetry. Your data stays local.</p>
          </article>

          <article className="status-card">
            <span className="card-label">MODULES</span>
            <strong>0 enabled</strong>
            <p>Module infrastructure will be added next.</p>
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
      </main>
    </div>
  );
}

export default App;
