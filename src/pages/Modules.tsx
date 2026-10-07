import { useLocalResource } from "../hooks/useLocalResource";
import { api } from "../services/api";

export function Modules() {
  const { data: modules, error, reload } = useLocalResource(api.getModules);

  return (
    <section aria-label="Module registry">
      <p>Roadmap metadata only. Installation and execution are not implemented yet.</p>
      <p>Permissions are a logical policy model, not an OS sandbox. Missing or undeclared permissions are denied.</p>
      {error ? (
        <div><p role="alert">Module registry unavailable.</p><button type="button" onClick={() => void reload()}>Retry modules</button></div>
      ) : modules === null ? (
        <p role="status">Loading modules…</p>
      ) : modules.length === 0 ? <p>No modules registered.</p> : modules.map(module => (
        <article className="panel module-card" key={module.id}>
          <div className="module-heading"><h2>{module.name}</h2><span className="development-badge">{module.lifecycle}</span></div>
          <p>{module.description}</p>
          <details>
            <summary>Lifecycle and permissions</summary>
            <p>
              Status: {module.lifecycle} · Installed: {module.lifecycle === "installed" || module.lifecycle === "enabled" ? "Yes" : "No"}
              {" · "}Enabled: {module.lifecycle === "enabled" ? "Yes" : "No"}
            </p>
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
