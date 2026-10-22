"""
Genera icon.ico multi-risoluzione per Gideon AI.
Stile: sfondo scuro, G verde neon, angoli HUD, testo AI.
"""
from PIL import Image, ImageDraw, ImageFont
import math
import os

def draw_icon(size):
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    s = size
    cx, cy = s // 2, s // 2

    # --- Sfondo arrotondato ---
    radius = int(s * 0.19)
    bg_dark = (7, 9, 11, 255)
    bg_mid  = (13, 19, 26, 255)
    draw.rounded_rectangle([0, 0, s - 1, s - 1], radius=radius, fill=bg_mid)

    # --- Anello esterno tratteggiato ---
    ring_r = int(s * 0.39)
    ring_w = max(1, int(s * 0.006))
    for deg in range(0, 360, 18):
        rad = math.radians(deg)
        rad2 = math.radians(deg + 10)
        x1 = cx + ring_r * math.cos(rad)
        y1 = cy + ring_r * math.sin(rad)
        x2 = cx + ring_r * math.cos(rad2)
        y2 = cy + ring_r * math.sin(rad2)
        draw.line([(x1, y1), (x2, y2)], fill=(0, 255, 102, 60), width=ring_w)

    # --- Lettera G ---
    green      = (0, 255, 102, 255)
    green_dim  = (0, 204, 82, 180)
    green_glow = (0, 255, 102, 40)

    font_size_g  = int(s * 0.58)
    font_size_ai = int(s * 0.13)

    # Prova font monospace di sistema
    font_g  = None
    font_ai = None
    for fname in ["consolab.ttf", "consola.ttf", "cour.ttf", "arial.ttf", "segoeui.ttf"]:
        try:
            font_g  = ImageFont.truetype(f"C:/Windows/Fonts/{fname}", font_size_g)
            font_ai = ImageFont.truetype(f"C:/Windows/Fonts/{fname}", font_size_ai)
            break
        except:
            pass
    if font_g is None:
        font_g  = ImageFont.load_default()
        font_ai = ImageFont.load_default()

    # Glow layer per G
    for offset in range(4, 0, -1):
        alpha = 30 + offset * 8
        glow_col = (0, 255, 102, alpha)
        bbox = draw.textbbox((0, 0), "G", font=font_g)
        tw = bbox[2] - bbox[0]
        th = bbox[3] - bbox[1]
        tx = cx - tw // 2 - bbox[0]
        ty = int(cy - th * 0.58) - bbox[1]
        draw.text((tx - offset, ty - offset), "G", font=font_g, fill=glow_col)
        draw.text((tx + offset, ty + offset), "G", font=font_g, fill=glow_col)

    # G principale
    bbox = draw.textbbox((0, 0), "G", font=font_g)
    tw = bbox[2] - bbox[0]
    th = bbox[3] - bbox[1]
    tx = cx - tw // 2 - bbox[0]
    ty = int(cy - th * 0.58) - bbox[1]
    draw.text((tx, ty), "G", font=font_g, fill=green)

    # Barra orizzontale sotto G
    bar_y  = int(cy + th * 0.44)
    bar_x1 = int(cx - s * 0.22)
    bar_x2 = int(cx + s * 0.22)
    bar_h  = max(1, int(s * 0.012))
    draw.rounded_rectangle([bar_x1, bar_y, bar_x2, bar_y + bar_h],
                            radius=bar_h, fill=(0, 255, 102, 160))

    # Testo "AI" sotto la barra
    if size >= 48:
        bbox_ai = draw.textbbox((0, 0), "AI", font=font_ai)
        aw = bbox_ai[2] - bbox_ai[0]
        ax = cx - aw // 2 - bbox_ai[0]
        ay = bar_y + bar_h + int(s * 0.03)
        draw.text((ax, ay), "AI", font=font_ai, fill=green_dim)

    # --- Angoli HUD ---
    if size >= 32:
        pad   = int(s * 0.09)
        arm   = int(s * 0.085)
        thick = max(1, int(s * 0.025))
        corners = [
            (pad, pad, pad + arm, pad, pad, pad + arm),          # TL
            (s - pad, pad, s - pad - arm, pad, s - pad, pad + arm),  # TR
            (pad, s - pad, pad + arm, s - pad, pad, s - pad - arm),  # BL
            (s - pad, s - pad, s - pad - arm, s - pad, s - pad, s - pad - arm),  # BR
        ]
        for (x0, y0, x1, y1, x2, y2) in corners:
            draw.line([(x0, y0), (x1, y1)], fill=(0, 255, 102, 140), width=thick)
            draw.line([(x0, y0), (x2, y2)], fill=(0, 255, 102, 140), width=thick)

    return img


def main():
    sizes = [16, 24, 32, 48, 64, 128, 256]
    frames = [draw_icon(s) for s in sizes]

    out_path = os.path.join(os.path.dirname(__file__), "icon.ico")
    frames[0].save(
        out_path,
        format="ICO",
        append_images=frames[1:],
        sizes=[(s, s) for s in sizes]
    )
    print(f"[OK] icon.ico creato in: {out_path}")
    print(f"     Risoluzioni: {sizes}")


if __name__ == "__main__":
    main()
