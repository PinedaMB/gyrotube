import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { api, isTauriRuntime } from "../api";
import type { Track } from "../types";

export interface SpinRemoteSyncPayload {
  track: Track | null;
  position_secs: number;
  is_playing: boolean;
  sent_at_ms: number;
  allow_controls: boolean;
  source_name: string;
  host_name: string;
  queue?: Track[];
}

export interface NativeEventHandlers {
  onPlay: () => void;
  onPause: () => void;
  onTogglePlay: () => void;
  onStop: () => void;
  onNext: () => void;
  onPrev: () => void;
  onSeekTo: (secs: number) => void;
  onSeekBy: (offsetSecs: number) => void;
  onSetShuffle: (shuffle: boolean) => void;
  onSetRepeat: (repeat: boolean) => void;
  onSetVolume: (volume: number) => void;
  onMoveQueue: (from: number, to: number) => void;
  onEnqueueTrack: (track: Track, guestName: string) => void;
  onRemoteSync: (payload: SpinRemoteSyncPayload | null, error?: string | null) => void;
}

let activeUnlisteners: UnlistenFn[] = [];
let bindGen = 0;

export async function bindTauriPlayerEvents(handlers: NativeEventHandlers): Promise<void> {
  if (!isTauriRuntime()) return;
  const gen = ++bindGen;
  for (const unlisten of activeUnlisteners) {
    unlisten();
  }
  activeUnlisteners = [];

  try {
    const unlisteners = await Promise.all([
      listen<string>("tray:action", (event) => {
        if (event.payload === "toggle_play") handlers.onTogglePlay();
        else if (event.payload === "next") handlers.onNext();
        else if (event.payload === "prev") handlers.onPrev();
      }),
      listen<
        | { kind: "toggle_play_pause" }
        | { kind: "next_track" }
        | { kind: "prev_track" }
        | { kind: "move_track"; from: number; to: number }
        | { kind: "add_track"; video_id: string; guest_name: string; track?: Track | null }
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
          if (cmd.track) {
            handlers.onEnqueueTrack(cmd.track, cmd.guest_name);
            return;
          }
          void api
            .searchAll(cmd.video_id)
            .then((res) => {
              const match = res.tracks.find((t) => t.id === cmd.video_id) ?? res.tracks[0];
              if (match) handlers.onEnqueueTrack(match, cmd.guest_name);
            })
            .catch(() => {
              // Ignore lookup failure if fallback search fails
            });
        }
      }),
      listen<{
        target: string;
        payload?: SpinRemoteSyncPayload | null;
        error?: string | null;
      }>("spin:remote-sync", (event) => {
        handlers.onRemoteSync(event.payload.payload ?? null, event.payload.error);
      }),
      listen<
        | { kind: "play" }
        | { kind: "pause" }
        | { kind: "play_pause" }
        | { kind: "stop" }
        | { kind: "next" }
        | { kind: "previous" }
        | { kind: "seek_to"; position_secs: number }
        | { kind: "seek_by"; offset_secs: number }
        | { kind: "set_shuffle"; shuffle: boolean }
        | { kind: "set_repeat"; repeat: boolean }
        | { kind: "set_volume"; volume: number }
      >("mpris:command", (event) => {
        const cmd = event.payload;
        if (cmd.kind === "play") handlers.onPlay();
        else if (cmd.kind === "pause") handlers.onPause();
        else if (cmd.kind === "play_pause") handlers.onTogglePlay();
        else if (cmd.kind === "stop") handlers.onStop();
        else if (cmd.kind === "next") handlers.onNext();
        else if (cmd.kind === "previous") handlers.onPrev();
        else if (cmd.kind === "seek_to") handlers.onSeekTo(cmd.position_secs);
        else if (cmd.kind === "seek_by") handlers.onSeekBy(cmd.offset_secs);
        else if (cmd.kind === "set_shuffle") handlers.onSetShuffle(cmd.shuffle);
        else if (cmd.kind === "set_repeat") handlers.onSetRepeat(cmd.repeat);
        else if (cmd.kind === "set_volume") handlers.onSetVolume(cmd.volume);
      }),
    ]);

    if (gen !== bindGen) {
      for (const unlisten of unlisteners) unlisten();
      return;
    }
    activeUnlisteners = unlisteners;
  } catch {
    // Non-fatal outside Tauri event bridge
  }
}
