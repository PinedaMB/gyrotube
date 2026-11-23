import type { ActiveView, MainTab } from "./types";

export interface RouteState {
  view: ActiveView;
  tab: MainTab;
  depth: number;
  playerOpen?: boolean;
}

export function viewToHash(view: ActiveView, playerOpen = false): string {
  const params = new URLSearchParams();
  let path: string;
  if (view.kind === "artist") {
    if (view.artistName) params.set("name", view.artistName);
    path = `#/artist/${encodeURIComponent(view.artistId)}`;
  } else if (view.kind === "album") {
    if (view.title) params.set("title", view.title);
    if (view.isPlaylist) params.set("pl", "1");
    if (view.coverUrl) params.set("cover", view.coverUrl);
    path = `#/album/${encodeURIComponent(view.id)}`;
  } else if (view.kind === "search") {
    if (view.query) params.set("q", view.query);
    path = "#/search";
  } else {
    path = `#/${view.kind}`;
  }
  if (playerOpen) params.set("player", "1");
  const qs = params.toString();
  return `${path}${qs ? `?${qs}` : ""}`;
}

export function hashToView(rawHash: string): {
  view: ActiveView;
  tab: MainTab;
  playerOpen: boolean;
} {
  const cleaned = (rawHash || "#/home").replace(/^#\/?/, "");
  const [pathPart, queryPart] = cleaned.split("?");
  const segments = pathPart.split("/").filter(Boolean);
  const params = new URLSearchParams(queryPart ?? "");
  const head = segments[0] ?? "home";
  const playerOpen = params.get("player") === "1";

  if (head === "explore") {
    return { view: { kind: "explore" }, tab: "explore", playerOpen };
  }
  if (head === "search") {
    const q = params.get("q")?.trim();
    return {
      view: q ? { kind: "search", query: q } : { kind: "search" },
      tab: "explore",
      playerOpen,
    };
  }
  if (head === "collection") {
    return { view: { kind: "collection" }, tab: "collection", playerOpen };
  }
  if (head === "artist" && segments[1]) {
    const artistId = decodeURIComponent(segments[1]);
    const artistName = params.get("name") || "Artist";
    return { view: { kind: "artist", artistId, artistName }, tab: "explore", playerOpen };
  }
  if (head === "album" && segments[1]) {
    const id = decodeURIComponent(segments[1]);
    const title = params.get("title") || "Album";
    const isPlaylist = params.get("pl") === "1";
    const coverUrl = params.get("cover");
    return { view: { kind: "album", id, title, coverUrl, isPlaylist }, tab: "home", playerOpen };
  }
  return { view: { kind: "home" }, tab: "home", playerOpen };
}

export function isSameView(a: ActiveView, b: ActiveView): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === "search" && b.kind === "search") return (a.query ?? "") === (b.query ?? "");
  if (a.kind === "artist" && b.kind === "artist") return a.artistId === b.artistId;
  if (a.kind === "album" && b.kind === "album") return a.id === b.id;
  return true;
}
