import type { ActiveView, CollectionSubView, MainTab } from "./types";

export interface RouteOverlays {
  playerOpen?: boolean;
  partyOpen?: boolean;
  spinOpen?: boolean;
}

export interface RouteState extends RouteOverlays {
  view: ActiveView;
  tab: MainTab;
  depth: number;
}

export function viewToHash(view: ActiveView, overlays: RouteOverlays = {}): string {
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
  } else if (view.kind === "collection") {
    if (view.sub?.kind === "favorites") {
      path = "#/collection/favorites";
    } else if (view.sub?.kind === "playlist") {
      path = `#/collection/playlist/${encodeURIComponent(view.sub.id)}`;
    } else {
      path = "#/collection";
    }
  } else {
    path = `#/${view.kind}`;
  }
  if (overlays.playerOpen) params.set("player", "1");
  if (overlays.partyOpen) params.set("party", "1");
  if (overlays.spinOpen) params.set("spin", "1");
  const qs = params.toString();
  return `${path}${qs ? `?${qs}` : ""}`;
}

export function hashToView(rawHash: string): {
  view: ActiveView;
  tab: MainTab;
  playerOpen: boolean;
  partyOpen: boolean;
  spinOpen: boolean;
} {
  const cleaned = (rawHash || "#/home").replace(/^#\/?/, "");
  const [pathPart, queryPart] = cleaned.split("?");
  const segments = pathPart.split("/").filter(Boolean);
  const params = new URLSearchParams(queryPart ?? "");
  const head = segments[0] ?? "home";
  const playerOpen = params.get("player") === "1";
  const partyOpen = params.get("party") === "1";
  const spinOpen = params.get("spin") === "1";

  if (head === "explore") {
    return { view: { kind: "explore" }, tab: "explore", playerOpen, partyOpen, spinOpen };
  }
  if (head === "search") {
    const q = params.get("q")?.trim();
    return {
      view: q ? { kind: "search", query: q } : { kind: "search" },
      tab: "explore",
      playerOpen,
      partyOpen,
      spinOpen,
    };
  }
  if (head === "collection") {
    let sub: CollectionSubView | undefined;
    if (segments[1] === "favorites") {
      sub = { kind: "favorites" };
    } else if (segments[1] === "playlist" && segments[2]) {
      sub = { kind: "playlist", id: decodeURIComponent(segments[2]) };
    }
    return {
      view: sub ? { kind: "collection", sub } : { kind: "collection" },
      tab: "collection",
      playerOpen,
      partyOpen,
      spinOpen,
    };
  }
  if (head === "settings") {
    return { view: { kind: "settings" }, tab: "home", playerOpen, partyOpen, spinOpen };
  }
  if (head === "artist" && segments[1]) {
    const artistId = decodeURIComponent(segments[1]);
    const artistName = params.get("name") || "Artist";
    return {
      view: { kind: "artist", artistId, artistName },
      tab: "explore",
      playerOpen,
      partyOpen,
      spinOpen,
    };
  }
  if (head === "album" && segments[1]) {
    const id = decodeURIComponent(segments[1]);
    const title = params.get("title") || "Album";
    const isPlaylist = params.get("pl") === "1";
    const coverUrl = params.get("cover");
    return {
      view: { kind: "album", id, title, coverUrl, isPlaylist },
      tab: "home",
      playerOpen,
      partyOpen,
      spinOpen,
    };
  }
  return { view: { kind: "home" }, tab: "home", playerOpen, partyOpen, spinOpen };
}

function isSameCollectionSub(a?: CollectionSubView, b?: CollectionSubView): boolean {
  if (!a && !b) return true;
  if (!a || !b || a.kind !== b.kind) return false;
  if (a.kind === "playlist" && b.kind === "playlist") return a.id === b.id;
  return true;
}

export function isSameView(a: ActiveView, b: ActiveView): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === "search" && b.kind === "search") return (a.query ?? "") === (b.query ?? "");
  if (a.kind === "collection" && b.kind === "collection") {
    return isSameCollectionSub(a.sub, b.sub);
  }
  if (a.kind === "artist" && b.kind === "artist") return a.artistId === b.artistId;
  if (a.kind === "album" && b.kind === "album") return a.id === b.id;
  return true;
}
