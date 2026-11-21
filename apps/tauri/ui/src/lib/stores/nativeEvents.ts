import { listen } from "@tauri-apps/api/event";
import { api, isTauriRuntime } from "../api";
import type { Track } from "../types";

export interface SpinRemoteSyncPayload {
  track: Track | null;
  position_secs: number;
  is_playing: boolean;
  sent_at_ms: number;
  source_name: string;
  host_name: string;
}

export interface NativeEventHandlers {
  onTogglePlay: () => void;
  onNext: () => void;
  onPrev: () => void;
  onMoveQueue: (from: number, to: number) => void;
  onEnqueueTrack: (track: Track, guestName: string) => void;
  onRemoteSync: (payload: SpinRemoteSyncPayload | null, error?: string | null) => void;
}

export async function bindTauriPlayerEvents(handlers: NativeEventHandlers): Promise<void> {
  if (!isTauriRuntime()) return;
  try {
    await listen<string>("tray:action", (event) => {
      if (event.payload === "toggle_play") handlers.onTogglePlay();
      else if (event.payload === "next") handlers.onNext();
      else if (event.payload === "prev") handlers.onPrev();
    });

    await listen<
      | { kind: "toggle_play_pause" }
      | { kind: "next_track" }
      | { kind: "prev_track" }
      | { kind: "move_track"; from: number; to: number }
      | { kind: "add_track"; video_id: string; guest_name: string }
    >("spin:command", (event) => {
      const cmd = event.payload;
      if (cmd.kind === "toggle_play_pause") {
        handlers.onTogglePlay();
      } else if (cmd.kind === "next_track") {
        handlers.onNext();
      } else if (cmd.kind === "prev_track") {
        handlers.onPrev();
      } else if (cmd.kind === "move_track") {
        handlers.onMoveQueue(cmd.from, cmd.to);
      } else if (cmd.kind === "add_track") {
        void api.searchAll(cmd.video_id).then((res) => {
          const match = res.tracks.find((t) => t.id === cmd.video_id) ?? res.tracks[0];
          if (match) handlers.onEnqueueTrack(match, cmd.guest_name);
        });
      }
    });

    await listen<{
      target: string;
      payload?: SpinRemoteSyncPayload | null;
      error?: string | null;
    }>("spin:remote-sync", (event) => {
      handlers.onRemoteSync(event.payload.payload ?? null, event.payload.error);
    });
  } catch {
    // Non-fatal outside Tauri event bridge
  }
}
