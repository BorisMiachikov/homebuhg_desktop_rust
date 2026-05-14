const NBSP = " ";
const RUB = "₽";

export function formatMoney(minor: number, currency = "RUB"): string {
  const abs = Math.abs(minor);
  const major = Math.trunc(abs / 100);
  const kop = abs % 100;
  const sign = minor < 0 ? "-" : "";
  const symbol = currency === "RUB" ? RUB : currency;
  return `${sign}${groupThousands(major)},${kop.toString().padStart(2, "0")}${NBSP}${symbol}`;
}

function groupThousands(n: number): string {
  const s = n.toString();
  let out = "";
  for (let i = 0; i < s.length; i++) {
    const rem = s.length - i;
    if (i > 0 && rem % 3 === 0) out += NBSP;
    out += s[i];
  }
  return out;
}

export function parseAmountToMinor(text: string): number | null {
  const cleaned = text.replace(/\s/g, "").replace(",", ".");
  if (!cleaned) return null;
  const num = Number(cleaned);
  if (!Number.isFinite(num) || num < 0) return null;
  return Math.round(num * 100);
}

export function formatDate(ms: number): string {
  if (!ms) return "";
  const d = new Date(ms);
  const day = d.getDate().toString().padStart(2, "0");
  const month = (d.getMonth() + 1).toString().padStart(2, "0");
  return `${day}.${month}.${d.getFullYear()}`;
}

export function formatDateTime(ms: number): string {
  if (!ms) return "";
  const d = new Date(ms);
  const date = formatDate(ms);
  const h = d.getHours().toString().padStart(2, "0");
  const m = d.getMinutes().toString().padStart(2, "0");
  return `${date} ${h}:${m}`;
}

export function todayStartMs(): number {
  const d = new Date();
  d.setHours(0, 0, 0, 0);
  return d.getTime();
}

export function toIsoDate(ms: number): string {
  const d = new Date(ms);
  const y = d.getFullYear();
  const m = (d.getMonth() + 1).toString().padStart(2, "0");
  const day = d.getDate().toString().padStart(2, "0");
  return `${y}-${m}-${day}`;
}

export function fromIsoDate(iso: string): number {
  const [y, m, d] = iso.split("-").map(Number);
  return new Date(y, (m || 1) - 1, d || 1).getTime();
}

export function colorToCss(color: number): string {
  const c = color >>> 0;
  const a = ((c >> 24) & 0xff) / 255;
  const r = (c >> 16) & 0xff;
  const g = (c >> 8) & 0xff;
  const b = c & 0xff;
  if (a >= 0.999) return `rgb(${r}, ${g}, ${b})`;
  return `rgba(${r}, ${g}, ${b}, ${a.toFixed(3)})`;
}
