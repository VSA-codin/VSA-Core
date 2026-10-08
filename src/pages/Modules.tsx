import { useState } from "react";
import { useLocalResource } from "../hooks/useLocalResource";
import { api, type Permission } from "../services/api";
import { LifecycleFilter, lifecycleLabels, type LifecycleSelection } from "../components/LifecycleFilter";

const permissionLabels: Record<Permission, string> = {
  network: "Network access", "filesystem.read": "Read files", "filesystem.write": "Write files",
  "process.execute": "Run processes", notifications: "Notifications", "secrets.read": "Read secrets", clipboard: "Clipboard access",
};

export function Modules() {
  const [query, setQuery] = useState("");
  const [lifecycle, setLifecycle] = useState<LifecycleSelection>("all");
  const { data: modules, error, reload } = useLocalResource(api.getModules);
  const normalized = query.trim().toLowerCase();
  const visibleModules = (modules ?? []).filter(module =>
    (lifecycle === "all" || module.lifecycle === lifecycle) &&
    [module.id, module.name, module.description].some(value => value.toLowerCase().includes(normalized))
  );

  return (
    <section aria-label="Module registry">
      <p className="page-intro">Explore the VSA roadmap. Modules describe planned products; installation and execution are not implemented.</p>
      {modules !== null && !error && <>
        <div className="module-filters">
          <label className="search-filter" htmlFor="module-search">Find modules
            <input id="module-search" type="search" maxLength={128} value={query} onChange={event => setQuery(event.target.value)} placeholder="Search name, ID or description" />
          </label>
          <LifecycleFilter value={lifecycle} onChange={setLifecycle} />
          <button type="button" disabled={!query && lifecycle === "all"} onClick={() => { setQuery(""); setLifecycle("all"); }}>Clear filters</button>
        </div>
        <p className="result-count" role="status" aria-live="polite">{visibleModules.length} of {modules.length} modules</p>
      </>}
      {error ? <div className="notice"><p role="alert">Module registry unavailable.</p><button type="button" onClick={() => void reload()}>Retry modules</button></div>
        : modules === null ? <p className="empty-state" role="status">Loading modules…</p>
        : modules.length === 0 ? <p className="empty-state">No modules registered.</p>
        : visibleModules.length === 0 ? <p className="empty-state">No matching modules. Clear filters to see the registry.</p>
        : <div className="module-list">{visibleModules.map(module => (
          <article className="module-card" key={module.id}>
            <div className="module-heading"><h2>{module.name}</h2><span className={`lifecycle-badge lifecycle-${module.lifecycle}`}>{lifecycleLabels[module.lifecycle]}</span></div>
            <p className="module-description">{module.description}</p>
            <details>
              <summary>Module details</summary>
              <div className="module-details">
                <dl className="metadata">
                  <div><dt>Module ID</dt><dd><code>{module.id}</code></dd></div>
                  <div><dt>Version</dt><dd>{module.version ?? "Not released"}</dd></div>
                  <div><dt>CORE API</dt><dd>{module.requiredCoreApi ?? "Not specified"}</dd></div>
                  <div><dt>Publisher ID</dt><dd>{module.publisherId ?? "Not attested"}</dd></div>
                  <div><dt>Trust</dt><dd>Unsigned metadata · verification unavailable</dd></div>
                  <div><dt>Runtime</dt><dd>Not implemented</dd></div>
                </dl>
                <div className="permission-section">
                  <h3>Permission declarations</h3>
                  {module.declaredPermissions.length === 0
                    ? <p>No permissions declared. Planned requirements have not been reviewed.</p>
                    : <ul className="permission-list">{module.declaredPermissions.map(permission => (
                      <li key={permission}><span>{permissionLabels[permission]}</span><span>{module.permissionPolicy[permission] === "allow" ? "Allowed by policy" : "Denied by policy"}</span></li>
                    ))}</ul>}
                  <p className="muted">Missing permissions are denied. Policy metadata does not provide an OS sandbox.</p>
                </div>
              </div>
            </details>
          </article>
        ))}</div>}
    </section>
  );
}
