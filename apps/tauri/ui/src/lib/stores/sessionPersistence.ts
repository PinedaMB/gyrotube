import { api, isTauriRuntime } from "../api";
import type { SessionState, ThemeMode } from "../types";

const CHART_COUNTRY_KEY = "gyrotube.chartCountry";
const THEME_MODE_KEY = "gyrotube.themeMode";
const NAV_COLLAPSED_KEY = "gyrotube.navCollapsed";
const SIDEBAR_COLLAPSED_KEY = "gyrotube.sidebarCollapsed";

export function readNavCollapsed(): boolean {
  if (typeof localStorage === "undefined") return false;
  try {
    return localStorage.getItem(NAV_COLLAPSED_KEY) === "1";
  } catch {
    return false;
  }
}

export function writeNavCollapsed(collapsed: boolean): void {
  if (typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(NAV_COLLAPSED_KEY, collapsed ? "1" : "0");
  } catch {
    // ignore storage errors
  }
}

export function readInitialSidebarOpen(): boolean {
  if (typeof window !== "undefined" && window.innerWidth < 1024) return false;
  if (typeof localStorage === "undefined") return true;
  try {
    return localStorage.getItem(SIDEBAR_COLLAPSED_KEY) !== "1";
  } catch {
    return true;
  }
}

export function writeSidebarCollapsed(collapsed: boolean): void {
  if (typeof window !== "undefined" && window.innerWidth < 1024) return;
  if (typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(SIDEBAR_COLLAPSED_KEY, collapsed ? "1" : "0");
  } catch {
    // ignore storage errors
  }
}

export function readThemeMode(): ThemeMode {
  if (typeof localStorage === "undefined") return "system";
  try {
    const saved = localStorage.getItem(THEME_MODE_KEY);
    if (saved === "light" || saved === "dark" || saved === "system") return saved;
  } catch {
    // ignore storage errors
  }
  return "system";
}

let systemThemeMediaBound = false;
let lastAppliedThemeMode: ThemeMode = "system";

function syncAndroidSystemBars(mode: ThemeMode): void {
  if (typeof window === "undefined") return;
  const bridge = (
    window as unknown as {
      __GYROTUBE_ANDROID__?: {
        setSystemBarsTheme?: (bgHex: string, isDark: boolean) => void;
      };
    }
  ).__GYROTUBE_ANDROID__;
  if (!bridge?.setSystemBarsTheme) return;
  const prefersDark =
    typeof window.matchMedia === "function" &&
    window.matchMedia("(prefers-color-scheme: dark)").matches;
  const isDark = mode === "dark" || (mode === "system" && prefersDark);
  bridge.setSystemBarsTheme(isDark ? "#1e1c26" : "#ffffff", isDark);
}

export function applyThemeToDocument(mode: ThemeMode): void {
  lastAppliedThemeMode = mode;
  if (typeof localStorage !== "undefined") {
    try {
      localStorage.setItem(THEME_MODE_KEY, mode);
    } catch {
      // ignore storage errors
    }
  }
  if (typeof document === "undefined") return;
  const root = document.documentElement;
  if (mode === "system") {
    root.removeAttribute("data-theme");
    root.style.colorScheme = "";
  } else {
    root.setAttribute("data-theme", mode);
    root.style.colorScheme = mode;
  }
  syncAndroidSystemBars(mode);
  if (!systemThemeMediaBound && typeof window !== "undefined" && typeof window.matchMedia === "function") {
    systemThemeMediaBound = true;
    window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
      if (lastAppliedThemeMode === "system") {
        syncAndroidSystemBars("system");
      }
    });
  }
}

export function readChartCountry(): string | null {
  return typeof localStorage !== "undefined" ? localStorage.getItem(CHART_COUNTRY_KEY) : null;
}

export function writeChartCountry(country: string | null): string | null {
  const clean = country && country.trim() ? country.trim().toUpperCase() : null;
  if (typeof localStorage !== "undefined") {
    if (clean) localStorage.setItem(CHART_COUNTRY_KEY, clean);
    else localStorage.removeItem(CHART_COUNTRY_KEY);
  }
  return clean;
}

export class SessionPersister {
  private timer: number | null = null;

  public schedule(buildSession: () => SessionState, delayMs = 350): void {
    if (!isTauriRuntime()) return;
    if (this.timer !== null) {
      window.clearTimeout(this.timer);
    }
    this.timer = window.setTimeout(() => {
      this.timer = null;
      void api.saveSession(buildSession()).catch(() => {});
    }, delayMs);
  }
}
