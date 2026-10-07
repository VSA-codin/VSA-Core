import { useEffect, useRef, useState } from "react";
import { api } from "../services/api";

export function SupportReport() {
  const [report, setReport] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const request = useRef(0);
  useEffect(() => () => { ++request.current; }, []);

  async function preview() {
    const current = ++request.current;
    setLoading(true);
    setError(false);
    setReport(null);
    try {
      const result = await api.getSupportReport();
      if (current === request.current) setReport(result);
    } catch {
      if (current === request.current) setError(true);
    } finally {
      if (current === request.current) setLoading(false);
    }
  }

  return (
    <section className="panel" aria-labelledby="support-report-title">
      <div className="panel-heading"><h2 id="support-report-title">Support Report</h2><span className="development-badge">Local only</span></div>
      <p>Includes version, platform, architecture, runtime, build mode, local-first/account/telemetry facts, settings read status, module counts, and permission limitations. Paths and file contents are excluded, even when paths are revealed in Trust Center. Nothing is uploaded or saved automatically.</p>
      <button type="button" disabled={loading} onClick={() => void preview()}>
        {loading ? "Preparing report…" : report ? "Refresh report preview" : "Preview sanitized report"}
      </button>
      {loading && <p role="status">Reading local diagnostics…</p>}
      {error && <p role="alert">Support report unavailable. Retry the preview.</p>}
      {report && (
        <>
          <p role="status">Report ready. Review the text before sharing it manually.</p>
          <label htmlFor="support-report">Sanitized report text</label>
          <textarea id="support-report" className="support-report" readOnly value={report} rows={18} spellCheck={false} />
          <button type="button" onClick={() => setReport(null)}>Hide report preview</button>
        </>
      )}
    </section>
  );
}
