import { useEffect, useRef, useState } from "react";
import { api, type CoreStatus } from "./services/api";
import "./App.css";
import { Modules } from "./pages/Modules";
import { TrustCenter } from "./pages/TrustCenter";
import { Settings } from "./pages/Settings";
import type { AppSettings } from "./services/api";

const navigation = [
  "Dashboard",
  "Modules",
  "Automation",
  "Vault",
  "Trust Center",
  "Settings",
];

function App() {
  const contentRef = useRef<HTMLElement>(null);
  const [settingsError, setSettingsError] = useState(false);
  const [settings, setSettings] = useState<AppSettings>({ compactLayout: false });
  const [page, setPage] = useState("Dashboard");
  const [coreStatus, setCoreStatus] = useState<CoreStatus | null>(null);
  const [coreError, setCoreError] = useState(false);

  useEffect(() => {
    contentRef.current?.scrollTo({ top: 0 });
  }, [page]);

  useEffect(() => {
    api.getSettings()
      .then(setSettings)
      .catch(() => setSettingsError(true));
    api.getCoreStatus()
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
    <div className={`app-shell ${settings.compactLayout ? "compact" : ""}`}>
      <a className="skip-link" href="#main-content">Skip to content</a>
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

        <nav className="navigation" aria-label="Main navigation">
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
          {coreStatus && !coreError && <span className="status-dot" />}
          {coreError ? "Core unavailable" : coreStatus ? `${coreStatus.mode} mode` : "Connecting..."}
        </div>
      </aside>

      <main id="main-content" className="main-content" ref={contentRef} tabIndex={-1} aria-labelledby="page-title">
        <header className="topbar">
          <div>
            <p className="eyebrow">VSA SOFTWARE ECOSYSTEM</p>
            <h1 id="page-title">{page}</h1>
          </div>

          <div className="core-status">
            {isOnline && <span className="status-dot" />}
            {coreError ? "Core unavailable" : isOnline ? "Core online" : "Connecting..."}
          </div>
        </header>

        {settingsError && <p role="alert">Saved layout could not be loaded. Using default layout; see Settings for recovery.</p>}
        {page === "Dashboard" && (
          <>
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
          </>
        )}
        {page === "Modules" && <Modules />}
        {page === "Trust Center" && <TrustCenter />}
        {["Automation", "Vault"].includes(page) && (
          <section className="panel">
            <h2>{page}</h2>
            <p>Not implemented yet.</p>
          </section>
        )}
        {page === "Settings" && <Settings onChange={value => { setSettings(value); setSettingsError(false); }} />}
      </main>
    </div>
  );
}

export default App;
