import { api, isTauriRuntime } from "../api";
import type { SessionState, ThemeMode } from "../types";

const CHART_COUNTRY_KEY = "gyrotube.chartCountry";
const THEME_MODE_KEY = "gyrotube.themeMode";

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

export function applyThemeToDocument(mode: ThemeMode): void {
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
