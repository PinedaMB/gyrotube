import en from "./i18n/en.json";
import es from "./i18n/es.json";
import { player } from "./stores/player.svelte";

export type TranslationKey = keyof typeof en;
export type ResolvedLocale = "en" | "es";

const catalogs: Record<ResolvedLocale, Record<TranslationKey, string>> = {
  en,
  es,
};

export function resolveLocale(): ResolvedLocale {
  if (player.language === "Es") return "es";
  if (player.language === "En") return "en";
  if (typeof navigator !== "undefined") {
    const lang = (navigator.language || "en").toLowerCase();
    if (lang.startsWith("es")) return "es";
  }
  return "en";
}

/**
 * Reactive translation helper for Svelte 5 templates.
 * Automatically re-evaluates whenever `player.language` changes.
 */
export function t(key: TranslationKey, params?: Record<string, string | number>): string {
  const locale = resolveLocale();
  let text = catalogs[locale][key] ?? catalogs.en[key] ?? key;
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replaceAll(`{${k}}`, String(v));
    }
  }
  return text;
}
