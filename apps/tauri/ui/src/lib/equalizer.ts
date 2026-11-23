export const EQ_FREQUENCIES = [60, 230, 910, 3600, 14000] as const;
export const EQ_LABELS = ["60 Hz", "230 Hz", "910 Hz", "3.6 kHz", "14 kHz"] as const;

export type EqPresetId =
  | "flat"
  | "bass"
  | "vocal"
  | "rock"
  | "pop"
  | "electronic"
  | "acoustic"
  | "custom";

export const EQ_PRESETS: Record<Exclude<EqPresetId, "custom">, [number, number, number, number, number]> = {
  flat: [0, 0, 0, 0, 0],
  bass: [6, 4, 0, -1, 1],
  vocal: [-2, 0, 4, 3, 1],
  rock: [4, 2, -1, 3, 4],
  pop: [-1, 2, 4, 2, -1],
  electronic: [5, 3, 0, 2, 5],
  acoustic: [2, 1, 2, 3, 2],
};

export interface EqSettings {
  enabled: boolean;
  normalize: boolean;
  preset: EqPresetId;
  bands: [number, number, number, number, number];
}

const STORAGE_KEY = "gyrotube.eq_settings_v1";

export const DEFAULT_EQ_SETTINGS: EqSettings = {
  enabled: false,
  normalize: false,
  preset: "flat",
  bands: [0, 0, 0, 0, 0],
};

export function loadEqSettings(): EqSettings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...DEFAULT_EQ_SETTINGS, bands: [...DEFAULT_EQ_SETTINGS.bands] };
    const parsed = JSON.parse(raw) as Partial<EqSettings>;
    const bands =
      Array.isArray(parsed.bands) && parsed.bands.length === 5
        ? (parsed.bands.map((v) => Math.max(-12, Math.min(12, Number(v) || 0))) as [
            number,
            number,
            number,
            number,
            number,
          ])
        : ([0, 0, 0, 0, 0] as [number, number, number, number, number]);
    return {
      enabled: Boolean(parsed.enabled),
      normalize: Boolean(parsed.normalize),
      preset: (parsed.preset as EqPresetId) ?? "flat",
      bands,
    };
  } catch {
    return { ...DEFAULT_EQ_SETTINGS, bands: [...DEFAULT_EQ_SETTINGS.bands] };
  }
}

export function saveEqSettings(settings: EqSettings): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch {
    // ignore storage quota errors
  }
}
