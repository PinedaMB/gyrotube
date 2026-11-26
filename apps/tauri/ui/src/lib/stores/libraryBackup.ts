import type { Artist, Track, UserPlaylist } from "../types";

export interface LibrarySnapshot {
  favorites: Track[];
  userPlaylists: UserPlaylist[];
  followedArtists: Artist[];
}

export interface LibraryImportDiff {
  newFavorites: Track[];
  newPlaylists: UserPlaylist[];
  newArtists: Artist[];
}

const SAFE_COVER_DATA_RE = /^data:image\/(?:jpeg|png|webp);base64,[A-Za-z0-9+/=]+$/;

export function serializeLibraryBackup(snapshot: LibrarySnapshot): string {
  return JSON.stringify(
    {
      version: 1,
      exported_at: new Date().toISOString(),
      favorites: snapshot.favorites,
      user_playlists: snapshot.userPlaylists,
      followed_artists: snapshot.followedArtists,
    },
    null,
    2,
  );
}

export function parseLibraryBackupImport(
  raw: string,
  existing: LibrarySnapshot,
): LibraryImportDiff {
  const parsed = JSON.parse(raw) as Record<string, unknown>;
  if (!parsed || typeof parsed !== "object") {
    throw new Error("Invalid backup");
  }

  const newFavorites: Track[] = [];
  if (Array.isArray(parsed.favorites)) {
    const existingIds = new Set(existing.favorites.map((t) => t.id));
    for (const item of parsed.favorites) {
      if (
        item &&
        typeof item === "object" &&
        typeof (item as Track).id === "string" &&
        typeof (item as Track).title === "string" &&
        typeof (item as Track).artist === "string" &&
        !existingIds.has((item as Track).id)
      ) {
        existingIds.add((item as Track).id);
        newFavorites.push(item as Track);
      }
    }
  }

  const newPlaylists: UserPlaylist[] = [];
  if (Array.isArray(parsed.user_playlists)) {
    const existingIds = new Set(existing.userPlaylists.map((p) => p.id));
    for (const item of parsed.user_playlists) {
      if (
        item &&
        typeof item === "object" &&
        typeof (item as UserPlaylist).id === "string" &&
        typeof (item as UserPlaylist).name === "string" &&
        Array.isArray((item as UserPlaylist).tracks) &&
        !existingIds.has((item as UserPlaylist).id)
      ) {
        const pl = item as UserPlaylist;
        const rawCover = typeof pl.cover_path === "string" ? pl.cover_path.trim() : null;
        const safeCover =
          rawCover && (SAFE_COVER_DATA_RE.test(rawCover) || rawCover.startsWith("https://"))
            ? rawCover
            : null;
        existingIds.add(pl.id);
        newPlaylists.push({ ...pl, cover_path: safeCover });
      }
    }
  }

  const newArtists: Artist[] = [];
  if (Array.isArray(parsed.followed_artists)) {
    const existingIds = new Set(existing.followedArtists.map((a) => a.id));
    for (const item of parsed.followed_artists) {
      if (
        item &&
        typeof item === "object" &&
        typeof (item as Artist).id === "string" &&
        typeof (item as Artist).name === "string" &&
        !existingIds.has((item as Artist).id)
      ) {
        existingIds.add((item as Artist).id);
        newArtists.push(item as Artist);
      }
    }
  }

  return { newFavorites, newPlaylists, newArtists };
}
