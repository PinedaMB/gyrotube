import type { ExternalTrackQuery } from "./types";

function parseCsvRows(text: string): string[][] {
  const clean = text.replace(/^\uFEFF/, "");
  const rows: string[][] = [];
  let row: string[] = [];
  let field = "";
  let inQuotes = false;

  for (let i = 0; i < clean.length; i++) {
    const ch = clean[i];
    if (inQuotes) {
      if (ch === '"') {
        if (clean[i + 1] === '"') {
          field += '"';
          i += 1;
        } else {
          inQuotes = false;
        }
      } else {
        field += ch;
      }
    } else if (ch === '"') {
      inQuotes = true;
    } else if (ch === ",") {
      row.push(field.trim());
      field = "";
    } else if (ch === "\r" || ch === "\n") {
      if (ch === "\r" && clean[i + 1] === "\n") i += 1;
      row.push(field.trim());
      field = "";
      if (row.some((cell) => cell.length > 0)) {
        rows.push(row);
      }
      row = [];
    } else {
      field += ch;
    }
  }

  if (field.length > 0 || row.length > 0) {
    row.push(field.trim());
    if (row.some((cell) => cell.length > 0)) {
      rows.push(row);
    }
  }

  return rows;
}

function parseDurationCell(raw: string | undefined, isMsHeader: boolean): number | null {
  if (!raw) return null;
  const trimmed = raw.trim();
  if (!trimmed) return null;
  if (trimmed.includes(":")) {
    const parts = trimmed.split(":").map((p) => Number(p));
    if (parts.length === 2 && parts.every((n) => Number.isFinite(n))) {
      return Math.max(0, Math.round(parts[0] * 60 + parts[1]));
    }
  }
  const num = Number(trimmed);
  if (!Number.isFinite(num) || num <= 0) return null;
  if (isMsHeader || num > 3600) {
    return Math.round(num / 1000);
  }
  return Math.round(num);
}

export function parseSpotifyCsv(text: string): ExternalTrackQuery[] {
  const rows = parseCsvRows(text);
  if (rows.length < 2) return [];

  const headers = rows[0].map((h) => h.toLowerCase().trim());
  const titleIdx = headers.findIndex((h) =>
    ["track name", "title", "song", "song name", "name"].includes(h),
  );
  const artistIdx = headers.findIndex((h) =>
    ["artist name(s)", "artist", "artist name", "artists"].includes(h),
  );
  const durIdx = headers.findIndex((h) =>
    ["duration (ms)", "track duration (ms)", "duration", "duration_ms", "duration_secs"].includes(h),
  );

  if (titleIdx === -1) return [];
  const isMsHeader = durIdx !== -1 && headers[durIdx].includes("ms");

  const tracks: ExternalTrackQuery[] = [];
  for (let i = 1; i < rows.length; i++) {
    const cols = rows[i];
    const title = (cols[titleIdx] ?? "").trim();
    if (!title) continue;
    const rawArtist = artistIdx !== -1 ? (cols[artistIdx] ?? "").trim() : "";
    // Exportify separates multiple artists with ";"
    const artist = rawArtist.replace(/\s*;\s*/g, ", ");
    const duration_secs =
      durIdx !== -1 ? parseDurationCell(cols[durIdx], isMsHeader) : null;
    tracks.push({ title, artist, duration_secs });
  }

  return tracks;
}
