/**
 * Reactive device & viewport store for Svelte 5.
 * Lets any component adapt interactions cleanly between desktop (mouse)
 * and mobile/Android (touch, compact viewports) without repeating media queries.
 */
class DeviceStore {
  isMobile = $state(false);
  isCompact = $state(false);
  isTouch = $state(false);
  isAndroid = $state(false);

  constructor() {
    if (typeof window === "undefined") return;

    const ua = navigator.userAgent || "";
    const win = window as unknown as { __GYROTUBE_ANDROID__?: unknown };
    this.isAndroid = Boolean(win.__GYROTUBE_ANDROID__) || /Android/i.test(ua);

    const mqMobile = window.matchMedia("(max-width: 767px)");
    const mqCompact = window.matchMedia("(max-width: 1023px)");
    const mqTouch = window.matchMedia("(pointer: coarse)");

    const sync = () => {
      this.isMobile = mqMobile.matches;
      this.isCompact = mqCompact.matches;
      this.isTouch = mqTouch.matches || this.isAndroid;
    };

    sync();
    mqMobile.addEventListener("change", sync);
    mqCompact.addEventListener("change", sync);
    mqTouch.addEventListener("change", sync);
  }
}

export const device = new DeviceStore();
