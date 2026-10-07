import { useState } from "react";
import { useLocalResource } from "../hooks/useLocalResource";
import { api, moduleLifecycles, type ModuleLifecycle } from "../services/api";

export function Modules() {
  const [query, setQuery] = useState("");
  const [lifecycle, setLifecycle] = useState<ModuleLifecycle | "all">("all");
  const { data: modules, error, reload } = useLocalResource(api.getModules);

  const normalized = query.trim().toLowerCase();
  const visibleModules = (modules ?? []).filter(module =>
    (lifecycle === "all" || module.lifecycle === lifecycle) &&
    [module.id, module.name, module.description].some(value => value.toLowerCase().includes(normalized))
  );

  return (
    <section aria-label="Module registry">
      <p>Roadmap metadata only. Installation and execution are not implemented yet.</p>
      <p>Permissions are a logical policy model, not an OS sandbox. Missing or undeclared permissions are denied.</p>
      {modules !== null && !error && (
        <>
          <div className="module-filters">
            <label htmlFor="module-search">Find modules
              <input id="module-search" type="search" maxLength={128} value={query} onChange={event => setQuery(event.target.value)} placeholder="Name, ID, or description" />
            </label>
            <label htmlFor="module-lifecycle">Lifecycle
              <select id="module-lifecycle" value={lifecycle} onChange={event => setLifecycle(moduleLifecycles.find(value => value === event.target.value) ?? "all")}>
                <option value="all">All lifecycles</option>
                {moduleLifecycles.map(value => <option key={value} value={value}>{value}</option>)}
              </select>
            </label>
            <button type="button" disabled={!query && lifecycle === "all"} onClick={() => { setQuery(""); setLifecycle("all"); }}>Clear filters</button>
          </div>
          <p role="status">Showing {visibleModules.length} of {modules.length} modules.</p>
        </>
      )}
      {error ? (
        <div><p role="alert">Module registry unavailable.</p><button type="button" onClick={() => void reload()}>Retry modules</button></div>
      ) : modules === null ? (
        <p role="status">Loading modules…</p>
      ) : modules.length === 0 ? <p>No modules registered.</p> : visibleModules.length === 0 ? <p>No modules match these filters. Clear filters to see the registry.</p> : visibleModules.map(module => (
        <article className="panel module-card" key={module.id}>
          <div className="module-heading"><h2>{module.name}</h2><span className="development-badge">{module.lifecycle}</span></div>
          <p>{module.description}</p>
          <details>
            <summary>Lifecycle and permissions</summary>
            <p>
              Status: {module.lifecycle} · Installed: {module.lifecycle === "installed" || module.lifecycle === "enabled" ? "Yes" : "No"}
              {" · "}Enabled: {module.lifecycle === "enabled" ? "Yes" : "No"}
            </p>
            <p>Module ID: {module.id} · Runtime health: unavailable; no module runtime exists.</p>
            {module.version && <p>Version: {module.version}</p>}
            <h3>Declared permissions</h3>
            {module.declaredPermissions.length === 0 ? (
              <p>None declared. Requirements for planned products are not specified yet. Missing permissions are denied.</p>
            ) : (
              <ul>
                {module.declaredPermissions.map(permission => (
                  <li key={permission}>{permission}: {module.permissionPolicy[permission] ?? "deny"}</li>
                ))}
              </ul>
            )}
          </details>
        </article>
      ))}
    </section>
  );
}
