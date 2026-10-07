import type { CoreStatus } from "../services/api";

export function Dashboard({ coreStatus, coreError }: { coreStatus: CoreStatus | null; coreError: boolean }) {
  const isOnline = coreStatus !== null && !coreError;
  return (
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
          {coreStatus?.privacy ?? "Local First"}
        </div>
      </section>

      <section className="status-grid">
        <article className="status-card">
          <span className="card-label">CORE STATUS</span>
          <strong>{isOnline ? "Responding" : coreError ? "Unavailable" : "Starting"}</strong>
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
          <p>Telemetry collection is not implemented. Local settings stay on this device.</p>
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
            <h2>VSA CORE foundation</h2>
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
  );
}
