import type { Track } from "../types";
import { device } from "./device.svelte";

export interface TrackContextPayload {
  kind: "track";
  track: Track;
  onPlay?: () => void;
  onRemove?: () => void;
  removeLabel?: string;
}

export interface AppContextPayload {
  kind: "app";
}

export type ContextPayload = TrackContextPayload | AppContextPayload;

class ContextMenuStore {
  open = $state(false);
  x = $state(0);
  y = $state(0);
  payload = $state<ContextPayload | null>(null);

  private place(clientX: number, clientY: number, menuW: number, menuH: number) {
    const pad = 10;
    const vw = typeof window !== "undefined" ? window.innerWidth : 1200;
    const vh = typeof window !== "undefined" ? window.innerHeight : 800;
    this.x = Math.max(pad, Math.min(clientX, vw - menuW - pad));
    this.y = Math.max(pad, Math.min(clientY, vh - menuH - pad));
  }

  public openTrackMenu(
    e: MouseEvent,
    track: Track,
    opts: { onPlay?: () => void; onRemove?: () => void; removeLabel?: string } = {},
  ): void {
    e.preventDefault();
    e.stopPropagation();
    this.payload = {
      kind: "track",
      track,
      onPlay: opts.onPlay,
      onRemove: opts.onRemove,
      removeLabel: opts.removeLabel,
    };
    this.place(e.clientX, e.clientY, 248, 360);
    this.open = true;
  }

  public openTrackSheet(
    track: Track,
    opts: { onPlay?: () => void; onRemove?: () => void; removeLabel?: string } = {},
  ): void {
    this.payload = {
      kind: "track",
      track,
      onPlay: opts.onPlay,
      onRemove: opts.onRemove,
      removeLabel: opts.removeLabel,
    };
    this.open = true;
  }

  public openAppMenu(e: MouseEvent): void {
    const target = e.target as HTMLElement | null;
    // Allow native Copy/Paste context menu inside text inputs
    if (
      target &&
      (target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.isContentEditable)
    ) {
      return;
    }
    e.preventDefault();
    // On touch/mobile devices, block the native WebView menu without opening
    // an accidental general menu while scrolling or resting a finger.
    if (device.isTouch || device.isMobile) {
      return;
    }
    this.payload = { kind: "app" };
    this.place(e.clientX, e.clientY, 224, 290);
    this.open = true;
  }

  public close(): void {
    this.open = false;
  }
}

export const contextMenu = new ContextMenuStore();
