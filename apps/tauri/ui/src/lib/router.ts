import type { ActiveView, MainTab } from "./types";

export interface RouteState {
  view: ActiveView;
  tab: MainTab;
  depth: number;
}

export function viewToHash(view: ActiveView): string {
  if (view.kind === "artist") {
    const params = new URLSearchParams();
    if (view.artistName) params.set("name", view.artistName);
    const qs = params.toString();
    return `#/artist/${encodeURIComponent(view.artistId)}${qs ? `?${qs}` : ""}`;
  }
  if (view.kind === "album") {
    const params = new URLSearchParams();
    if (view.title) params.set("title", view.title);
    if (view.isPlaylist) params.set("pl", "1");
    if (view.coverUrl) params.set("cover", view.coverUrl);
    const qs = params.toString();
    return `#/album/${encodeURIComponent(view.id)}${qs ? `?${qs}` : ""}`;
  }
  return `#/${view.kind}`;
}

export function hashToView(rawHash: string): { view: ActiveView; tab: MainTab } {
  const cleaned = (rawHash || "#/home").replace(/^#\/?/, "");
  const [pathPart, queryPart] = cleaned.split("?");
  const segments = pathPart.split("/").filter(Boolean);
  const params = new URLSearchParams(queryPart ?? "");
  const head = segments[0] ?? "home";

  if (head === "explore") {
    return { view: { kind: "explore" }, tab: "explore" };
  }
  if (head === "collection") {
    return { view: { kind: "collection" }, tab: "collection" };
  }
  if (head === "artist" && segments[1]) {
    const artistId = decodeURIComponent(segments[1]);
    const artistName = params.get("name") || "Artist";
    return { view: { kind: "artist", artistId, artistName }, tab: "explore" };
  }
  if (head === "album" && segments[1]) {
    const id = decodeURIComponent(segments[1]);
    const title = params.get("title") || "Album";
    const isPlaylist = params.get("pl") === "1";
    const coverUrl = params.get("cover");
    return { view: { kind: "album", id, title, coverUrl, isPlaylist }, tab: "home" };
  }
  return { view: { kind: "home" }, tab: "home" };
}

export function isSameView(a: ActiveView, b: ActiveView): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === "artist" && b.kind === "artist") return a.artistId === b.artistId;
  if (a.kind === "album" && b.kind === "album") return a.id === b.id;
  return true;
}
