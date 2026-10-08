// Locally bundled, decorative line icons; labels supply accessible names.
const paths: Record<string, string> = {
  Dashboard: "M3 3h7v7H3z M14 3h7v7h-7z M3 14h7v7H3z M14 14h7v7h-7z",
  Modules: "M12 3l9 5-9 5-9-5z M3 12l9 5 9-5 M3 16l9 5 9-5",
  Automation: "M13 2L4 14h7l-1 8 10-13h-7z",
  Vault: "M5 10h14v11H5z M8 10V6a4 4 0 018 0v4 M12 14v3",
  "Trust Center": "M12 2l8 3v6c0 5-4 9-8 11-4-2-8-6-8-11V5z M8 12l3 3 5-6",
  Settings: "M4 6h16 M4 12h16 M4 18h16 M8 3v6 M16 9v6 M10 15v6",
  "Support Report": "M6 2h9l4 4v16H6z M14 2v5h5 M9 12h7 M9 16h7",
};
export function NavIcon({ name }: { name: string }) {
  return <svg className="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true"><path d={paths[name]} /></svg>;
}
