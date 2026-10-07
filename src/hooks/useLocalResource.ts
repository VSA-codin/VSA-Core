import { useCallback, useEffect, useRef, useState } from "react";

// For read-only IPC: a retry supersedes older requests, and unmount discards results.
// Loader identity must be stable (the shared API methods satisfy this).
export function useLocalResource<T>(loader: () => Promise<T>) {
  const request = useRef(0);
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  const reload = useCallback(async () => {
    const current = ++request.current;
    setLoading(true);
    setError(false);
    setData(null);
    try {
      const result = await loader();
      if (current === request.current) setData(result);
    } catch {
      if (current === request.current) setError(true);
    } finally {
      if (current === request.current) setLoading(false);
    }
  }, [loader]);

  useEffect(() => {
    void reload();
    return () => { ++request.current; };
  }, [reload]);

  return { data, loading, error, reload };
}
