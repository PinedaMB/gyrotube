export type ParsedYtmLink =
  | { kind: "track"; id: string }
  | { kind: "playlist"; id: string }
  | { kind: "album"; id: string }
  | { kind: "artist"; id: string };

const VIDEO_ID_RE = /^[A-Za-z0-9_-]{11}$/;

export function parseYtmUrl(raw: string): ParsedYtmLink | null {
  const trimmed = raw.trim();
  if (!trimmed || (!trimmed.includes("youtube.com") && !trimmed.includes("youtu.be"))) {
    return null;
  }
  let url: URL;
  try {
    url = new URL(trimmed.startsWith("http") ? trimmed : `https://${trimmed}`);
  } catch {
    return null;
  }

  const host = url.hostname.toLowerCase();
  if (
    host !== "youtu.be" &&
    !host.endsWith(".youtube.com") &&
    host !== "youtube.com"
  ) {
    return null;
  }

  // Short youtu.be/<id>
  if (host === "youtu.be") {
    const seg = url.pathname.split("/").filter(Boolean)[0];
    if (seg && VIDEO_ID_RE.test(seg)) {
      return { kind: "track", id: seg };
    }
    return null;
  }

  const path = url.pathname;
  const listParam = url.searchParams.get("list")?.trim();
  const vParam = url.searchParams.get("v")?.trim();

  // Explicit playlist page
  if (path.startsWith("/playlist") && listParam) {
    return { kind: "playlist", id: listParam };
  }

  // Browse URLs (album MPREb_... or artist UC... or playlist VL...)
  if (path.startsWith("/browse/")) {
    const bid = path.slice("/browse/".length).split("/")[0]?.trim();
    if (bid) {
      if (bid.startsWith("MPREb_") || bid.startsWith("OLAK")) {
        return { kind: "album", id: bid };
      }
      if (bid.startsWith("UC")) {
        return { kind: "artist", id: bid };
      }
      if (bid.startsWith("VL") || bid.startsWith("PL") || bid.startsWith("RDCLAK")) {
        return { kind: "playlist", id: bid.replace(/^VL/, "") };
      }
    }
  }

  if (path.startsWith("/channel/")) {
    const cid = path.slice("/channel/".length).split("/")[0]?.trim();
    if (cid && cid.startsWith("UC")) {
      return { kind: "artist", id: cid };
    }
  }

  // Watch URL: prioritize track if `v` is present
  if (vParam && VIDEO_ID_RE.test(vParam)) {
    return { kind: "track", id: vParam };
  }

  if (listParam) {
    return { kind: "playlist", id: listParam };
  }

  return null;
}
