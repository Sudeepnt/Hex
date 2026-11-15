#!/usr/bin/env python3
"""Generate assets/theme.json, Utter's colours, from assets/base.json.

Change the three values below and run `python3 tools/brand.py`:
  ACCENT_HUE      the brand colour (OKLCH hue, degrees): 305 violet, 265 blue,
                  185 teal, 150 green, 30 coral.
  NEUTRAL_HUE     the tint of greys and backgrounds (75 warm paper, 250 cool slate).
  NEUTRAL_CHROMA  how strongly greys are tinted (0 = pure grey, 0.012 = soft).
The accent family (every blue, plus the "ember" brand scale) moves to
ACCENT_HUE keeping its lightness and saturation, so contrast is unchanged.
"""
import json, math, pathlib, re

ACCENT_HUE = 305.0
NEUTRAL_HUE = 75.0
NEUTRAL_CHROMA = 0.012

ROOT = pathlib.Path(__file__).resolve().parent.parent
HEX = re.compile(r"^#([0-9a-fA-F]{6})([0-9a-fA-F]{2})?$")

def to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4

def to_srgb(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055

def rgb_to_oklch(r, g, b):
    r, g, b = to_linear(r), to_linear(g), to_linear(b)
    l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b) ** (1 / 3)
    m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b) ** (1 / 3)
    s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b) ** (1 / 3)
    L = 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s
    A = 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s
    B = 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s
    return L, math.hypot(A, B), math.degrees(math.atan2(B, A)) % 360

def oklch_to_rgb(L, C, H):
    A, B = C * math.cos(math.radians(H)), C * math.sin(math.radians(H))
    l = (L + 0.3963377774 * A + 0.2158037573 * B) ** 3
    m = (L - 0.1055613458 * A - 0.0638541728 * B) ** 3
    s = (L - 0.0894841775 * A - 1.2914855480 * B) ** 3
    return (
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    )

def in_gamut(rgb):
    return all(-1e-4 <= c <= 1 + 1e-4 for c in rgb)

def fit(L, C, H):
    """The colour at L, H with the most chroma up to C that sRGB can show."""
    lo, hi = 0.0, C
    if in_gamut(oklch_to_rgb(L, C, H)):
        return oklch_to_rgb(L, C, H)
    for _ in range(24):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if in_gamut(oklch_to_rgb(L, mid, H)) else (lo, mid)
    return oklch_to_rgb(L, lo, H)

def hexcolour(rgb, alpha):
    r, g, b = (round(max(0.0, min(1.0, to_srgb(max(0.0, c)))) * 255) for c in rgb)
    return f"#{r:02x}{g:02x}{b:02x}{alpha}"

def recolour(key, value):
    m = HEX.match(value)
    if not m:
        return value
    alpha = (m.group(2) or "ff").lower()
    r, g, b = (int(m.group(1)[i:i + 2], 16) / 255 for i in (0, 2, 4))
    L, C, H = rgb_to_oklch(r, g, b)
    if key.startswith("ember-") or (C > 0.035 and 235 <= H <= 290):
        shift = ACCENT_HUE - (265.0 if not key.startswith("ember-") else H)
        return hexcolour(fit(L, C, (H + shift) % 360), alpha)
    if C < 0.02:
        # Tint greys: strongest mid-scale, faint at pure white and black.
        tint = NEUTRAL_CHROMA * math.sin(math.pi * min(max(L, 0), 1)) ** 0.7
        tint = max(tint, NEUTRAL_CHROMA * 0.35) if L > 0.97 else tint
        return hexcolour(fit(L, tint, NEUTRAL_HUE), alpha)
    return value

doc = json.loads((ROOT / "assets/base.json").read_text())
doc["name"] = "Utter"
for mode in ("light", "dark"):
    doc[mode] = {k: recolour(k, v) if isinstance(v, str) else v for k, v in doc[mode].items()}
(ROOT / "assets/theme.json").write_text(json.dumps(doc, indent=2, sort_keys=False) + "\n")
print("wrote assets/theme.json")
