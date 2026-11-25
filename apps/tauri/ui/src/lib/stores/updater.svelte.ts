import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { api, isTauriRuntime } from "../api";
import { t } from "../i18n.svelte";
import { player } from "./player.svelte";

export type UpdateStatus = "idle" | "checking" | "up_to_date" | "available" | "installing" | "error";

class UpdaterStore {
  public currentVersion = $state("1.0.0");
  public latestVersion = $state<string | null>(null);
  public status = $state<UpdateStatus>("idle");
  public canAutoInstall = $state(false);
  public releaseUrl = $state("https://github.com/PinedaMB/gyrotube/releases/latest");
  public downloadUrl = $state<string | null>(null);
  public progressPct = $state<number | null>(null);
  public errorMessage = $state<string | null>(null);

  private initialized = false;
  private unlistenProgress: UnlistenFn | null = null;

  public get updateAvailable(): boolean {
    return this.status === "available" || this.status === "installing";
  }

  public init(): void {
    if (this.initialized || !isTauriRuntime()) return;
    this.initialized = true;

    void api
      .getAppVersion()
      .then((v) => {
        if (v) this.currentVersion = v;
      })
      .catch(() => {});

    // Silent background check 4s after launch if online
    window.setTimeout(() => {
      if (!player.isOffline && this.status === "idle") {
        void this.check(true);
      }
    }, 4000);
  }

  public async check(silent = false): Promise<void> {
    if (!isTauriRuntime() || this.status === "checking" || this.status === "installing") return;
    this.status = "checking";
    this.errorMessage = null;
    try {
      const res = await Promise.race([
        api.checkForUpdates(),
        new Promise<never>((_, reject) =>
          window.setTimeout(() => reject(new Error("Update check timed out")), 15000),
        ),
      ]);
      this.currentVersion = res.current_version || this.currentVersion;
      this.latestVersion = res.latest_version;
      this.canAutoInstall = res.can_auto_install;
      this.releaseUrl = res.release_url;
      this.downloadUrl = res.download_url;
      if (res.update_available && res.latest_version) {
        this.status = "available";
        if (silent) {
          player.showToast(t("updater.toast_available", { version: res.latest_version }));
        }
      } else {
        this.status = "up_to_date";
      }
    } catch (err) {
      if (silent) {
        this.status = "idle";
      } else {
        this.status = "error";
        this.errorMessage = err instanceof Error ? err.message : String(err);
      }
    }
  }

  public async installOrDownload(): Promise<void> {
    if (!isTauriRuntime()) return;
    const win =
      typeof window !== "undefined"
        ? (window as unknown as {
            __GYROTUBE_ANDROID__?: {
              openExternalUrl?: (url: string) => void;
              downloadAndInstallApk?: (url: string) => void;
            };
            __GYROTUBE_ANDROID_UPDATE__?: (status: string, pct: number, error: string) => void;
          })
        : null;

    const androidBridge = win?.__GYROTUBE_ANDROID__;
    if (androidBridge?.downloadAndInstallApk && this.downloadUrl) {
      this.status = "installing";
      this.progressPct = 0;
      this.errorMessage = null;
      win!.__GYROTUBE_ANDROID_UPDATE__ = (status: string, pct: number, error: string) => {
        if (status === "progress") {
          this.status = "installing";
          this.progressPct = pct >= 0 ? Math.min(100, pct) : null;
        } else if (status === "ready") {
          this.progressPct = 100;
          this.status = "available";
        } else if (status === "error") {
          this.status = "error";
          this.errorMessage = error || "Update failed";
          player.showToast(t("updater.error"));
        }
      };
      androidBridge.downloadAndInstallApk(this.downloadUrl);
      return;
    }

    if (!this.canAutoInstall) {
      const target = this.downloadUrl || this.releaseUrl;
      if (androidBridge?.openExternalUrl) {
        androidBridge.openExternalUrl(target);
        return;
      }
      void api.openExternalUrl(target).catch(() => {
        window.open(target, "_blank", "noopener,noreferrer");
      });
      return;
    }

    this.status = "installing";
    this.progressPct = 0;
    this.errorMessage = null;
    try {
      this.unlistenProgress = await listen<{ downloaded: number; total: number | null }>(
        "updater:progress",
        (ev) => {
          const { downloaded, total } = ev.payload;
          if (total && total > 0) {
            this.progressPct = Math.min(100, Math.round((downloaded / total) * 100));
          } else {
            this.progressPct = null;
          }
        },
      );
      await api.installUpdate();
    } catch (err) {
      this.status = "error";
      this.errorMessage = err instanceof Error ? err.message : String(err);
      player.showToast(t("updater.error"));
    } finally {
      this.unlistenProgress?.();
      this.unlistenProgress = null;
    }
  }
}

export const updater = new UpdaterStore();
