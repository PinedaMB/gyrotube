export const GENRE_PALETTE = [
  "#10b981",
  "#e66100",
  "#3584e4",
  "#9141ac",
  "#2ec27e",
  "#e11d48",
  "#c061cb",
  "#eab308",
  "#00b4d8",
  "#625df5",
] as const;

export const STATION_PALETTE = [
  "#e05263",
  "#10b981",
  "#9141ac",
  "#c061cb",
  "#e66100",
  "#3584e4",
] as const;

export const MOOD_KEYWORDS = [
  "chill",
  "christmas",
  "commute",
  "energize",
  "feel good",
  "focus",
  "gaming",
  "party",
  "romance",
  "sad",
  "sleep",
  "workout",
] as const;

export function isMoodGenre(name: string): boolean {
  const lower = name.toLowerCase();
  return MOOD_KEYWORDS.some((m) => lower.includes(m));
}

export function colorForGenre(name: string): string {
  let h = 2166136261;
  for (let i = 0; i < name.length; i++) {
    h ^= name.toLowerCase().charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  return GENRE_PALETTE[Math.abs(h) % GENRE_PALETTE.length];
}
