export interface GyroAndroidBridge {
  getSafeAreaInsets?: () => string;
  setSystemBarsTheme?: (bgHex: string, isDark: boolean) => void;
  openExternalUrl?: (url: string) => void;
  downloadAndInstallApk?: (url: string) => void;
  saveImageToGallery?: (filename: string, base64Png: string) => string;
  shareImage?: (filename: string, base64Png: string, title: string) => string;
  saveBackupJson?: (filename: string, content: string) => string;
  consumePendingSpinLink?: () => string | null;
  setPlaybackActive?: (
    playing: boolean,
    title: string,
    artist: string,
    coverUrl: string,
    hasTrack: boolean,
    positionMs: number,
    durationMs: number,
  ) => void;
}

export interface GyroAndroidWindow extends Window {
  __GYROTUBE_ANDROID__?: GyroAndroidBridge;
  __GYROTUBE_ANDROID_UPDATE__?: (status: string, pct: number, error: string) => void;
  __GYROTUBE_SPIN_LINK__?: (code: string) => void;
  __GYROTUBE_DISMISS_OVERLAY__?: () => boolean;
  __GYROTUBE_MEDIA_ACTION__?: (action: string) => void;
}

export function getAndroidWindow(): GyroAndroidWindow | null {
  if (typeof window === "undefined") return null;
  return window as unknown as GyroAndroidWindow;
}

export function getAndroidBridge(): GyroAndroidBridge | null {
  return getAndroidWindow()?.__GYROTUBE_ANDROID__ ?? null;
}
