import { t } from "../i18n.svelte";
import type { Track } from "../types";

export function formatPlayerErrorMessage(raw: string, isOffline: boolean): string {
  const msg = raw.trim();
  const lower = msg.toLowerCase();

  if (
    isOffline ||
    lower.includes("dns error") ||
    lower.includes("failed to lookup address") ||
    lower.includes("error sending request") ||
    lower.includes("network error") ||
    lower.includes("connection refused") ||
    lower.includes("connect error")
  ) {
    return t("error.no_internet");
  }
  if (lower.includes("timed out") || lower.includes("timeout")) {
    return t("error.timeout");
  }
  if (lower.includes("http 403") || lower.includes("http 410") || lower.includes("unavailable")) {
    return t("error.track_unavailable");
  }
  if (lower.includes("not supported") || lower.includes("audio error (code")) {
    return t("error.audio_playback");
  }
  return msg.length > 140 ? `${msg.slice(0, 137)}…` : msg;
}

export function createYtmTrackStub(videoId: string): Track {
  return {
    id: videoId,
    title: "YouTube Music",
    artist: "",
    artist_id: null,
    album: null,
    album_id: null,
    duration_secs: null,
    cover_url: `https://i.ytimg.com/vi/${videoId}/hqdefault.jpg`,
    cover_large_url: `https://i.ytimg.com/vi/${videoId}/hqdefault.jpg`,
    is_video: false,
  };
}
