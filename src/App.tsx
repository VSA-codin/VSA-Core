import { useEffect, useRef, useState } from "react";
import { api } from "./services/api";
import "./App.css";
import { useLocalResource } from "./hooks/useLocalResource";
import { Dashboard } from "./pages/Dashboard";
import { Modules } from "./pages/Modules";
import { TrustCenter } from "./pages/TrustCenter";
import { Settings } from "./pages/Settings";
import { NavIcon } from "./components/NavIcon";
import { SupportReport } from "./pages/SupportReport";
import type { AppSettings } from "./services/api";

const navigation = [
  "Dashboard",
  "Modules",
  "Automation",
  "Vault",
  "Trust Center",
  "Settings",
  "Support Report",
] as const;
type Page = typeof navigation[number];

function App() {
  const settingsRevision = useRef(0);
  const contentRef = useRef<HTMLElement>(null);
  const [settingsBusy, setSettingsBusy] = useState(false);
  const [settingsError, setSettingsError] = useState(false);
  const [settings, setSettings] = useState<AppSettings>({ compactLayout: false });
  const [page, setPage] = useState<Page>("Dashboard");
  const { data: coreStatus, error: coreError, reload: reloadCore } = useLocalResource(api.getCoreStatus);

  useEffect(() => {
    contentRef.current?.scrollTo({ top: 0 });
  }, [page]);

  useEffect(() => {
    let active = true;
    const revision = settingsRevision.current;
    api.getSettings()
      .then(value => { if (active && revision === settingsRevision.current) setSettings(value); })
      .catch(() => { if (active && revision === settingsRevision.current) setSettingsError(true); });
    return () => { active = false; };
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
              disabled={settingsBusy && page !== item}
              aria-current={page === item ? "page" : undefined}
              key={item}
              type="button"
            >
              <NavIcon name={item} /><span>{item}</span>
            </button>
          ))}
        </nav>

        <div className="sidebar-footer">
          {coreStatus && !coreError && <span className="status-dot" />}
          {coreError ? "Core unavailable" : coreStatus ? `${coreStatus.mode} mode` : "Connecting…"}
        </div>
      </aside>

      <main id="main-content" className="main-content" ref={contentRef} tabIndex={-1} aria-labelledby="page-title">
        <header className="topbar">
          <div>
            <p className="eyebrow">LOCAL WORKSPACE</p>
            <h1 id="page-title" aria-live="polite">{page}</h1>
          </div>

          <div className="core-status" role="status">
            {isOnline && <span className="status-dot" />}
            {coreError ? "Core unavailable" : isOnline ? "Core responding" : "Connecting…"}
          </div>
        </header>

        {settingsError && <p role="alert">Saved layout could not be loaded. Using default layout; see Settings for recovery.</p>}
        {coreError && <div><p role="alert">Core status unavailable.</p><button type="button" onClick={() => void reloadCore()}>Retry core status</button></div>}
        {page === "Dashboard" && <Dashboard coreStatus={coreStatus} coreError={coreError} />}
        {page === "Modules" && <Modules />}
        {page === "Trust Center" && <TrustCenter />}
        {["Automation", "Vault"].includes(page) && (
          <section className="panel boundary-panel">
            <div className="panel-heading"><h2>{page === "Automation" ? "Local workflows" : "Secret storage"}</h2><span className="development-badge">Not implemented</span></div>
            <p>{page === "Automation"
              ? "Future local workflows and scheduling will require explicit permissions. No automation runs today."
              : "Future secret storage requires a separately reviewed protection and recovery design. No secrets are stored or protected by this application today."}</p>
            <h3>{page === "Automation" ? "Metadata foundation" : "Design boundary"}</h3>
            {page === "Automation" ? <ul>
              <li>Validated manual and interval trigger contracts.</li>
              <li>Plans are disabled metadata in the experimental SDK.</li>
              <li>No timers, background jobs or action execution.</li>
            </ul> : <ul>
              <li>OS keychain and audited encryption require review.</li>
              <li>Lock, unlock, recovery and backup behavior remain undecided.</li>
              <li>Do not enter passwords, tokens or private keys into CORE.</li>
            </ul>}
          </section>
        )}
        {page === "Support Report" && <SupportReport />}
        {page === "Settings" && <Settings onBusyChange={setSettingsBusy} onChange={value => { ++settingsRevision.current; setSettings(value); setSettingsError(false); }} />}
      </main>
    </div>
  );
}

export default App;
