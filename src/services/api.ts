import { invoke } from "@tauri-apps/api/core";

export type AppSettings = { compactLayout: boolean };

export type CoreStatus = {
  name: string;
  version: string;
  runtime: string;
  mode: string;
  privacy: string;
  totalModules: number;
  enabledModules: number;
};

export type Permission = "network" | "filesystem.read" | "filesystem.write" | "process.execute" | "notifications" | "secrets.read" | "clipboard";

export type ModuleDescriptor = {
  id: string;
  name: string;
  description: string;
  version: string | null;
  lifecycle: "planned" | "available" | "installed" | "enabled";
  declaredPermissions: Permission[];
  permissionPolicy: Partial<Record<Permission, "allow" | "deny">>;
};

export type Diagnostics = {
  version: string; platform: string; architecture: string; runtime: string; buildMode: string;
  localFirst: boolean; telemetryImplemented: boolean; accountRequired: boolean;
  configDirectory: string | null; dataDirectory: string | null; settingsLoadState: "missing" | "loaded" | "invalid" | "unavailable"; storageStatus: string; registryStatus: string;
  totalModules: number; enabledModules: number; allowedPermissions: number;
};

export const api = {
  getSupportReport: () => invoke<string>("get_support_report"),
  getCoreStatus: () => invoke<CoreStatus>("get_core_status"),
  getModules: () => invoke<ModuleDescriptor[]>("get_modules"),
  getSettings: () => invoke<AppSettings>("get_settings"),
  updateSettings: (settings: AppSettings) => invoke<AppSettings>("update_settings", { settings }),
  recoveryAvailable: () => invoke<boolean>("settings_recovery_available"),
  recoverSettings: () => invoke<AppSettings>("recover_settings"),
  getDiagnostics: (includePaths = false) => invoke<Diagnostics>("get_diagnostics", { includePaths }),
};
