#!/usr/bin/env python3
"""Generate the 1200x630 social/OG card used by meta og:image and JSON-LD.

Run manually when branding changes:

    python site/scripts/gen-og-image.py

Output: site/docs/public/og-image.png
"""
from __future__ import annotations

import os
from PIL import Image, ImageDraw, ImageFont

W, H = 1200, 630
OUT = os.path.join(os.path.dirname(__file__), "..", "docs", "public", "og-image.png")

BG = (9, 14, 24)
BRAND_1 = (15, 118, 110)
BRAND_2 = (13, 148, 136)
BRAND_3 = (3, 105, 161)
WHITE = (240, 248, 250)
MUTED = (150, 172, 180)

FONT_DIR = r"C:\Windows\Fonts"
BOLD = os.path.join(FONT_DIR, "segoeuib.ttf")
SEMI = os.path.join(FONT_DIR, "seguisb.ttf")
REG = os.path.join(FONT_DIR, "segoeui.ttf")
MONO = os.path.join(FONT_DIR, "consola.ttf")


def font(path: str, size: int) -> ImageFont.FreeTypeFont:
    for candidate in (path, BOLD, REG):
        if os.path.exists(candidate):
            return ImageFont.truetype(candidate, size)
    return ImageFont.load_default()


def lerp(a, b, t):
    return tuple(round(a[i] + (b[i] - a[i]) * t) for i in range(3))


def diagonal_gradient(size, c1, c2, c3):
    """Diagonal three-stop gradient."""
    w, h = size
    img = Image.new("RGB", size)
    px = img.load()
    for y in range(h):
        for x in range(w):
            t = (x / max(w - 1, 1) + y / max(h - 1, 1)) / 2
            px[x, y] = lerp(c1, c2, t * 2) if t < 0.5 else lerp(c2, c3, (t - 0.5) * 2)
    return img


img = Image.new("RGB", (W, H), BG)
draw = ImageDraw.Draw(img)

# Soft radial glow from the top-right, built by compositing blurred ellipses.
glow = Image.new("RGB", (W, H), BG)
gdraw = ImageDraw.Draw(glow)
gdraw.ellipse((W - 620, -260, W + 260, 420), fill=lerp(BG, BRAND_2, 0.30))
glow = glow.filter(__import__("PIL.ImageFilter", fromlist=["ImageFilter"]).GaussianBlur(150))
img = Image.blend(img, glow, 0.85)
draw = ImageDraw.Draw(img)

# --- Code card on the right ------------------------------------------------
card = (700, 120, 1130, 470)
draw.rounded_rectangle(card, radius=22, fill=(13, 20, 34), outline=(38, 54, 74), width=2)
draw.rounded_rectangle((700, 120, 1130, 168), radius=22, fill=(18, 27, 44))
draw.rectangle((700, 146, 1130, 168), fill=(18, 27, 44))
for i, color in enumerate([(255, 95, 86), (255, 189, 46), (39, 201, 63)]):
    cx = 728 + i * 24
    draw.ellipse((cx - 6, 138, cx + 6, 150), fill=color)

code_font = font(MONO, 23)
lines = [
    ("func main() {", (126, 231, 219)),
    ('    print("Hello, SYX")', (214, 226, 235)),
    ("}", (126, 231, 219)),
    ("", WHITE),
    ("let g = Greeter(name: \"SYX\")", (214, 226, 235)),
    ("g.greet()", (214, 226, 235)),
]
for i, (line, color) in enumerate(lines):
    draw.text((730, 205 + i * 40), line, font=code_font, fill=color)

# --- Brand lockup on the left ---------------------------------------------
badge = (80, 96, 176, 192)
draw.rounded_rectangle(badge, radius=26, fill=BRAND_2)
mark_font = font(BOLD, 76)
mark = "S"
bbox = draw.textbbox((0, 0), mark, font=mark_font)
draw.text(
    (badge[0] + (badge[2] - badge[0] - (bbox[2] - bbox[0])) / 2 - bbox[0],
     badge[1] + (badge[3] - badge[1] - (bbox[3] - bbox[1])) / 2 - bbox[1]),
    mark, font=mark_font, fill=WHITE,
)

title_font = font(BOLD, 108)
draw.text((196, 96), "SYX", font=title_font, fill=WHITE)

tag_font = font(SEMI, 40)
draw.text((82, 250), "Swift-inspired. Rust-powered.", font=tag_font, fill=(198, 226, 232))

body_font = font(REG, 29)
draw.text((82, 322), "A modern programming language with a", font=body_font, fill=MUTED)
draw.text((82, 364), "VM, LLVM native AOT, and Wasm backends.", font=body_font, fill=MUTED)
draw.text((82, 406), "Source files end in .sy", font=body_font, fill=MUTED)

pill_font = font(SEMI, 25)
x = 82
for label in ("VM", "LLVM", "Wasm", "VS Code + LSP"):
    tw = draw.textlength(label, font=pill_font)
    draw.rounded_rectangle((x, 470, x + tw + 36, 520), radius=25, fill=(20, 33, 50), outline=(46, 66, 90), width=1)
    draw.text((x + 18, 481), label, font=pill_font, fill=(150, 226, 220))
    x += tw + 52

# Bottom brand rule
draw.rectangle((0, H - 8, W, H), fill=BRAND_2)

img.save(OUT, "PNG", optimize=True)
print("wrote", os.path.normpath(OUT), img.size)
