export type LyricsCardThemeId = "cover" | "black" | "emerald" | "midnight" | "sunset" | "paper";
export type LyricsCardAspect = "portrait" | "square";

export interface LyricsCardOptions {
  lines: string[];
  title: string;
  artist: string;
  coverUrl: string | null;
  ambientRgb: [number, number, number] | null;
  theme: LyricsCardThemeId;
  aspect: LyricsCardAspect;
}

interface Palette {
  bgStart: string;
  bgMid: string;
  bgEnd: string;
  glowTop: string;
  glowBottom: string;
  text: string;
  muted: string;
  badgeBg: string;
  accent: string;
  vinylCore: string;
  isLight: boolean;
}

function rgbToHsl(r: number, g: number, b: number): [number, number, number] {
  const rn = r / 255;
  const gn = g / 255;
  const bn = b / 255;
  const max = Math.max(rn, gn, bn);
  const min = Math.min(rn, gn, bn);
  const l = (max + min) / 2;
  let h = 0;
  let s = 0;
  if (max !== min) {
    const d = max - min;
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    switch (max) {
      case rn:
        h = (gn - bn) / d + (gn < bn ? 6 : 0);
        break;
      case gn:
        h = (bn - rn) / d + 2;
        break;
      case bn:
        h = (rn - gn) / d + 4;
        break;
    }
    h *= 60;
  }
  return [h, s * 100, l * 100];
}

export function coverSwatchGradient(rgb: [number, number, number] | null): string {
  if (!rgb) return "linear-gradient(135deg, #10b981, #0f172a)";
  const [h, s] = rgbToHsl(rgb[0], rgb[1], rgb[2]);
  const sat = Math.max(46, Math.min(72, s + 22));
  return `linear-gradient(135deg, hsl(${Math.round(h)} ${Math.round(sat)}% 46%), hsl(${Math.round((h + 14) % 360)} ${Math.round(sat)}% 18%))`;
}

function paletteFor(theme: LyricsCardThemeId, rgb: [number, number, number] | null): Palette {
  if (theme === "cover" && rgb) {
    const [rawH, rawS] = rgbToHsl(rgb[0], rgb[1], rgb[2]);
    // Boost saturation so low-chroma album covers still look rich and editorial
    const h = Math.round(rawH);
    const hShift = (h + 16) % 360;
    const sat = Math.round(Math.max(42, Math.min(70, rawS + 20)));
    return {
      bgStart: `hsl(${h} ${sat}% 34%)`,
      bgMid: `hsl(${h} ${Math.max(36, sat - 6)}% 22%)`,
      bgEnd: `hsl(${hShift} ${Math.max(32, sat - 10)}% 12%)`,
      glowTop: `hsla(${h} ${Math.min(85, sat + 14)}% 58% / 0.36)`,
      glowBottom: `hsla(${hShift} ${Math.min(80, sat + 10)}% 46% / 0.24)`,
      text: "#ffffff",
      muted: "rgba(255, 255, 255, 0.78)",
      badgeBg: "rgba(255, 255, 255, 0.14)",
      accent: `hsl(${h} ${Math.min(90, sat + 22)}% 68%)`,
      vinylCore: `hsl(${hShift} ${Math.max(30, sat - 10)}% 13%)`,
      isLight: false,
    };
  }
  switch (theme) {
    case "black":
      return {
        bgStart: "#141416",
        bgMid: "#0a0a0c",
        bgEnd: "#040405",
        glowTop: "rgba(255, 255, 255, 0.07)",
        glowBottom: "rgba(255, 255, 255, 0.03)",
        text: "#ffffff",
        muted: "rgba(255, 255, 255, 0.68)",
        badgeBg: "rgba(255, 255, 255, 0.10)",
        accent: "#34d399",
        vinylCore: "#050506",
        isLight: false,
      };
    case "emerald":
    case "cover":
      return {
        bgStart: "#0d6e4f",
        bgMid: "#084431",
        bgEnd: "#041c15",
        glowTop: "rgba(52, 211, 153, 0.34)",
        glowBottom: "rgba(16, 185, 129, 0.22)",
        text: "#ffffff",
        muted: "rgba(236, 253, 245, 0.78)",
        badgeBg: "rgba(255, 255, 255, 0.14)",
        accent: "#34d399",
        vinylCore: "#052219",
        isLight: false,
      };
    case "midnight":
      return {
        bgStart: "#312e81",
        bgMid: "#1e1b4b",
        bgEnd: "#090a18",
        glowTop: "rgba(129, 140, 248, 0.35)",
        glowBottom: "rgba(99, 102, 241, 0.22)",
        text: "#ffffff",
        muted: "rgba(224, 231, 255, 0.78)",
        badgeBg: "rgba(255, 255, 255, 0.14)",
        accent: "#a5b4fc",
        vinylCore: "#0d0e22",
        isLight: false,
      };
    case "sunset":
      return {
        bgStart: "#c2410c",
        bgMid: "#7e2246",
        bgEnd: "#2e0845",
        glowTop: "rgba(251, 146, 60, 0.34)",
        glowBottom: "rgba(236, 72, 153, 0.24)",
        text: "#ffffff",
        muted: "rgba(255, 237, 213, 0.80)",
        badgeBg: "rgba(255, 255, 255, 0.15)",
        accent: "#fdba74",
        vinylCore: "#340a48",
        isLight: false,
      };
    case "paper":
      return {
        bgStart: "#ffffff",
        bgMid: "#f5f5f4",
        bgEnd: "#e7e5e4",
        glowTop: "rgba(16, 185, 129, 0.14)",
        glowBottom: "rgba(52, 211, 153, 0.10)",
        text: "#18181b",
        muted: "rgba(24, 24, 27, 0.62)",
        badgeBg: "rgba(24, 24, 27, 0.08)",
        accent: "#059669",
        vinylCore: "#f5f5f4",
        isLight: true,
      };
  }
}

export const LYRICS_CARD_THEMES: { id: LyricsCardThemeId; swatch: string }[] = [
  { id: "cover", swatch: "linear-gradient(135deg, #34d399, #6366f1)" },
  { id: "black", swatch: "linear-gradient(135deg, #27272a, #050506)" },
  { id: "emerald", swatch: "linear-gradient(135deg, #10b981, #064e3b)" },
  { id: "midnight", swatch: "linear-gradient(135deg, #4f46e5, #0f172a)" },
  { id: "sunset", swatch: "linear-gradient(135deg, #f97316, #581c87)" },
  { id: "paper", swatch: "linear-gradient(135deg, #ffffff, #d6d3d1)" },
];

function drawRoundedRect(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  w: number,
  h: number,
  r: number,
): void {
  ctx.beginPath();
  ctx.moveTo(x + r, y);
  ctx.arcTo(x + w, y, x + w, y + h, r);
  ctx.arcTo(x + w, y + h, x, y + h, r);
  ctx.arcTo(x, y + h, x, y, r);
  ctx.arcTo(x, y, x + w, y, r);
  ctx.closePath();
}

let cachedLogoPromise: Promise<HTMLImageElement | null> | null = null;

function loadAppLogo(): Promise<HTMLImageElement | null> {
  if (!cachedLogoPromise) {
    cachedLogoPromise = loadCorsImage("/gyrotube.svg");
  }
  return cachedLogoPromise;
}

function loadCorsImage(url: string): Promise<HTMLImageElement | null> {
  return new Promise((resolve) => {
    const img = new Image();
    img.crossOrigin = "anonymous";
    img.referrerPolicy = "no-referrer";
    const timer = window.setTimeout(() => resolve(null), 4000);
    img.onload = () => {
      window.clearTimeout(timer);
      resolve(img);
    };
    img.onerror = () => {
      window.clearTimeout(timer);
      resolve(null);
    };
    img.src = url;
  });
}

function wrapLines(
  ctx: CanvasRenderingContext2D,
  paragraphs: string[],
  maxWidth: number,
): string[] {
  const out: string[] = [];
  for (const para of paragraphs) {
    const words = para.trim().split(/\s+/).filter(Boolean);
    if (words.length === 0) continue;
    let current = words[0];
    for (let i = 1; i < words.length; i++) {
      const candidate = `${current} ${words[i]}`;
      if (ctx.measureText(candidate).width <= maxWidth) {
        current = candidate;
      } else {
        out.push(current);
        current = words[i];
      }
    }
    out.push(current);
  }
  return out;
}

function truncateText(ctx: CanvasRenderingContext2D, text: string, maxWidth: number): string {
  if (ctx.measureText(text).width <= maxWidth) return text;
  let trimmed = text;
  while (trimmed.length > 1 && ctx.measureText(`${trimmed}…`).width > maxWidth) {
    trimmed = trimmed.slice(0, -1);
  }
  return `${trimmed}…`;
}

let activeRenderSeq = 0;

export async function renderLyricsCard(
  canvas: HTMLCanvasElement,
  opts: LyricsCardOptions,
): Promise<void> {
  const seq = ++activeRenderSeq;
  const width = 1080;
  const height = opts.aspect === "portrait" ? 1350 : 1080;

  const pal = paletteFor(opts.theme, opts.ambientRgb);
  if (typeof document !== "undefined" && document.fonts?.ready) {
    await document.fonts.ready.catch(() => {});
  }
  // Load external images BEFORE mutating canvas so concurrent renders never interleave
  const [coverImg, logoImg] = await Promise.all([
    opts.coverUrl ? loadCorsImage(opts.coverUrl) : Promise.resolve(null),
    loadAppLogo(),
  ]);
  if (seq !== activeRenderSeq) return;

  canvas.width = width;
  canvas.height = height;

  const ctx = canvas.getContext("2d", { alpha: true });
  if (!ctx) return;

  const computedLyricsFont =
    typeof document !== "undefined"
      ? getComputedStyle(document.documentElement).getPropertyValue("--lyrics-font").trim()
      : "";
  const fontFamily =
    computedLyricsFont ||
    '"Plus Jakarta Sans Variable", "Plus Jakarta Sans", system-ui, -apple-system, sans-serif';

  // Full-bleed opaque card (no transparent corners that apps/viewers turn into solid boxes)
  ctx.clearRect(0, 0, width, height);

  // Rich 3-stop diagonal background gradient
  const bg = ctx.createLinearGradient(0, 0, width * 0.42, height);
  bg.addColorStop(0, pal.bgStart);
  bg.addColorStop(0.52, pal.bgMid);
  bg.addColorStop(1, pal.bgEnd);
  ctx.fillStyle = bg;
  ctx.fillRect(0, 0, width, height);

  // Atmospheric dual radial glows (editorial depth)
  const glowTop = ctx.createRadialGradient(
    width * 0.84,
    height * 0.12,
    24,
    width * 0.84,
    height * 0.12,
    width * 0.82,
  );
  glowTop.addColorStop(0, pal.glowTop);
  glowTop.addColorStop(1, "rgba(0,0,0,0)");
  ctx.fillStyle = glowTop;
  ctx.fillRect(0, 0, width, height);

  const glowBot = ctx.createRadialGradient(
    width * 0.14,
    height * 0.88,
    24,
    width * 0.14,
    height * 0.88,
    width * 0.68,
  );
  glowBot.addColorStop(0, pal.glowBottom);
  glowBot.addColorStop(1, "rgba(0,0,0,0)");
  ctx.fillStyle = glowBot;
  ctx.fillRect(0, 0, width, height);

  // Subtle top sheen for glass-like studio finish
  const sheen = ctx.createLinearGradient(0, 0, 0, height * 0.36);
  sheen.addColorStop(0, pal.isLight ? "rgba(255,255,255,0.45)" : "rgba(255,255,255,0.06)");
  sheen.addColorStop(1, "rgba(255,255,255,0)");
  ctx.fillStyle = sheen;
  ctx.fillRect(0, 0, width, height * 0.36);

  // Header: cover + vertically centered title & artist
  const pad = 88;
  const coverSize = 140;
  const headerY = pad;

  ctx.save();
  ctx.shadowColor = pal.isLight ? "rgba(15, 23, 42, 0.18)" : "rgba(0, 0, 0, 0.45)";
  ctx.shadowBlur = 36;
  ctx.shadowOffsetY = 14;
  drawRoundedRect(ctx, pad, headerY, coverSize, coverSize, 26);
  ctx.fillStyle = pal.badgeBg;
  ctx.fill();
  ctx.restore();

  if (coverImg) {
    ctx.save();
    drawRoundedRect(ctx, pad, headerY, coverSize, coverSize, 26);
    ctx.clip();
    const s = Math.min(coverImg.naturalWidth, coverImg.naturalHeight);
    const sx = (coverImg.naturalWidth - s) / 2;
    const sy = (coverImg.naturalHeight - s) / 2;
    ctx.drawImage(coverImg, sx, sy, s, s, pad, headerY, coverSize, coverSize);
    ctx.restore();

    // Subtle crisp border ring around cover art
    ctx.save();
    drawRoundedRect(ctx, pad + 1, headerY + 1, coverSize - 2, coverSize - 2, 25);
    ctx.strokeStyle = pal.isLight ? "rgba(24, 24, 27, 0.10)" : "rgba(255, 255, 255, 0.20)";
    ctx.lineWidth = 2;
    ctx.stroke();
    ctx.restore();
  } else {
    ctx.fillStyle = pal.accent;
    ctx.font = `800 56px ${fontFamily}`;
    ctx.textBaseline = "middle";
    ctx.textAlign = "center";
    ctx.fillText("♪", pad + coverSize / 2, headerY + coverSize / 2 + 2);
    ctx.textAlign = "left";
  }

  const metaX = pad + coverSize + 34;
  const metaMaxW = width - metaX - pad;
  const headerCenterY = headerY + coverSize / 2;

  ctx.textBaseline = "middle";
  ctx.fillStyle = pal.text;
  ctx.font = `800 42px ${fontFamily}`;
  ctx.fillText(truncateText(ctx, opts.title || "GyroTube", metaMaxW), metaX, headerCenterY - 22);

  ctx.fillStyle = pal.muted;
  ctx.font = `600 32px ${fontFamily}`;
  ctx.fillText(truncateText(ctx, opts.artist || "", metaMaxW), metaX, headerCenterY + 26);

  // Lyrics block in the center — bold hero typography
  const contentTop = headerY + coverSize + 74;
  const footerLineY = height - 144;
  const contentBottom = footerLineY - 46;
  const availH = contentBottom - contentTop;
  const maxTextW = width - pad * 2;

  const cleanLines = opts.lines.map((l) => l.trim()).filter(Boolean);
  const maxFontSize = cleanLines.length <= 2 ? 78 : cleanLines.length === 3 ? 68 : 58;
  let fontSize = maxFontSize;
  let wrapped: string[] = [];
  let lineHeight = fontSize * 1.34;

  for (let size = maxFontSize; size >= 36; size -= 2) {
    ctx.font = `800 ${size}px ${fontFamily}`;
    const candidate = wrapLines(ctx, cleanLines, maxTextW);
    const lh = size * 1.36;
    if (candidate.length * lh <= availH - 16) {
      fontSize = size;
      wrapped = candidate;
      lineHeight = lh;
      break;
    }
    fontSize = size;
    wrapped = candidate;
    lineHeight = lh;
  }

  const totalTextH = wrapped.length * lineHeight;
  let startY = contentTop + Math.max(0, (availH - totalTextH) / 2) + lineHeight * 0.5;

  // Soft decorative quote mark behind the first lyric line
  ctx.save();
  ctx.fillStyle = pal.isLight ? "rgba(24, 24, 27, 0.06)" : "rgba(255, 255, 255, 0.08)";
  ctx.font = `900 180px Georgia, serif`;
  ctx.textBaseline = "top";
  ctx.fillText("“", pad - 18, Math.max(contentTop - 36, startY - lineHeight * 0.5 - 92));
  ctx.restore();

  ctx.save();
  ctx.textBaseline = "middle";
  ctx.fillStyle = pal.text;
  ctx.font = `800 ${fontSize}px ${fontFamily}`;
  if ("letterSpacing" in ctx) {
    (ctx as CanvasRenderingContext2D & { letterSpacing: string }).letterSpacing = "-0.02em";
  }
  if (!pal.isLight) {
    ctx.shadowColor = "rgba(0, 0, 0, 0.28)";
    ctx.shadowBlur = 20;
    ctx.shadowOffsetY = 5;
  }
  for (const line of wrapped) {
    if (startY > contentBottom) break;
    ctx.fillText(line, pad, startY);
    startY += lineHeight;
  }
  ctx.restore();

  // Subtle hairline separator above footer
  ctx.save();
  ctx.strokeStyle = pal.isLight ? "rgba(24, 24, 27, 0.09)" : "rgba(255, 255, 255, 0.13)";
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(pad, footerLineY);
  ctx.lineTo(width - pad, footerLineY);
  ctx.stroke();
  ctx.restore();

  // Footer: real GyroTube app logo + wordmark
  const footerY = height - 74;
  const logoSize = 44;
  if (logoImg) {
    ctx.drawImage(logoImg, pad, footerY - logoSize / 2, logoSize, logoSize);
  } else {
    ctx.beginPath();
    ctx.arc(pad + 18, footerY, 16, 0, Math.PI * 2);
    ctx.fillStyle = pal.accent;
    ctx.fill();

    ctx.beginPath();
    ctx.arc(pad + 18, footerY, 5.5, 0, Math.PI * 2);
    ctx.fillStyle = pal.vinylCore;
    ctx.fill();
  }

  ctx.textBaseline = "middle";
  ctx.fillStyle = pal.text;
  ctx.globalAlpha = 0.88;
  ctx.font = `800 31px ${fontFamily}`;
  ctx.fillText("GyroTube", pad + (logoImg ? logoSize + 15 : 46), footerY + 1);
  ctx.globalAlpha = 1;
}

export function canvasToPngBlob(canvas: HTMLCanvasElement): Promise<Blob | null> {
  return new Promise((resolve) => canvas.toBlob((b) => resolve(b), "image/png"));
}

export function lyricsCardFilename(title: string): string {
  const slug = title
    .toLowerCase()
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 40);
  return `gyrotube-lyrics-${slug || "card"}.png`;
}
