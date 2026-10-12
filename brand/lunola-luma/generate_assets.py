"""Generate platform icon containers from the owner-supplied Lunola Luma PNG.

The source icon is not redrawn, recoloured, or given effects. The generator only clears
near-transparent stray pixels (alpha <= 8), then resamples the original RGBA artwork into
platform-required sizes. The supplied product lockup is copied byte-for-byte.

Run from any directory:
    python brand/lunola-luma/generate_assets.py
"""
from __future__ import annotations

import base64
import io
import shutil
import struct
from pathlib import Path

from PIL import Image, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
REF = Path(__file__).resolve().parent / "references"
OUT = Path(__file__).resolve().parent / "assets"
APP = ROOT / "assets" / "app-icon"
SOURCE_ICON = REF / "official-lunola-luma-app-icon.png"
SOURCE_LOCKUP = REF / "official-lunola-luma-lockup.png"
HICOLOR_SIZES = (16, 24, 32, 48, 64, 128, 256, 512)
ICO_SIZES = (16, 20, 24, 32, 40, 48, 64, 128, 256)
ICNS_SPECS = (("icp4", 16), ("icp5", 32), ("icp6", 64), ("ic07", 128), ("ic08", 256), ("ic09", 512), ("ic10", 1024))


def clean_source_icon() -> Image.Image:
    image = Image.open(SOURCE_ICON).convert("RGBA")
    if image.width != image.height:
        raise ValueError(f"the approved app icon must be square, got {image.size}")
    alpha = image.getchannel("A")
    if alpha.getbbox() is None:
        raise ValueError("the approved app icon is fully transparent")

    # Keep low-alpha antialiasing that touches the visible silhouette. Clear only isolated
    # alpha<=8 specks outside a one-pixel halo around the source's alpha>8 mark.
    support = alpha.point(lambda value: 255 if value > 8 else 0).filter(ImageFilter.MaxFilter(3))
    clean_alpha = Image.composite(alpha, Image.new("L", alpha.size, 0), support)
    image.putalpha(clean_alpha)
    if image.getpixel((0, 0))[3] != 0 or image.getpixel((image.width - 1, image.height - 1))[3] != 0:
        raise ValueError("the approved icon must retain transparent outer corners")
    return image


def resized(image: Image.Image, size: int) -> Image.Image:
    if image.width == size and image.height == size:
        return image.copy()
    return image.resize((size, size), Image.Resampling.LANCZOS)


def png_bytes(image: Image.Image) -> bytes:
    buffer = io.BytesIO()
    image.save(buffer, format="PNG", optimize=True)
    return buffer.getvalue()


def make_svg(image: Image.Image, path: Path) -> None:
    encoded = base64.b64encode(png_bytes(image)).decode("ascii")
    svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" role="img" aria-label="Lunola Luma">
  <title>Lunola Luma</title>
  <image x="0" y="0" width="512" height="512" preserveAspectRatio="xMidYMid meet" href="data:image/png;base64,{encoded}"/>
</svg>\n'''
    path.write_text(svg, encoding="utf-8")


def make_icns(image: Image.Image, path: Path) -> None:
    chunks = []
    for code, size in ICNS_SPECS:
        png = png_bytes(resized(image, size))
        chunks.append(code.encode("ascii") + struct.pack(">I", len(png) + 8) + png)
    body = b"".join(chunks)
    path.write_bytes(b"icns" + struct.pack(">I", len(body) + 8) + body)


def main() -> None:
    if not SOURCE_ICON.is_file() or not SOURCE_LOCKUP.is_file():
        raise FileNotFoundError("copy the two official Lunola Luma PNGs into brand/lunola-luma/references first")
    OUT.mkdir(parents=True, exist_ok=True)
    APP.mkdir(parents=True, exist_ok=True)

    icon = clean_source_icon()
    lockup = Image.open(SOURCE_LOCKUP).convert("RGBA")
    if lockup.getchannel("A").getbbox() is None:
        raise ValueError("the supplied product lockup is fully transparent")

    # Keep the full-resolution source-derived icon and the supplied lockup available to the app.
    icon.save(OUT / "lunola-luma-app-icon.png", optimize=True)
    shutil.copyfile(SOURCE_LOCKUP, OUT / "lunola-luma-lockup.png")

    # Windows executable icon. Keep the legacy resource path for build compatibility and also
    # publish the requested product-named ICO; both files have identical bytes.
    ico = resized(icon, 1024)
    ico.save(APP / "photocraft.ico", format="ICO", sizes=[(size, size) for size in ICO_SIZES])
    shutil.copyfile(APP / "photocraft.ico", APP / "lunola-luma.ico")

    # macOS app icon; preserve the supplied transparent margins in every ICNS representation.
    icon_1024 = resized(icon, 1024)
    icon_1024.save(APP / "photocraft-1024.png", optimize=True)
    icon_1024.save(APP / "lunola-luma-1024.png", optimize=True)
    make_icns(icon, APP / "photocraft.icns")
    shutil.copyfile(APP / "photocraft.icns", APP / "lunola-luma.icns")

    # Linux theme icon sizes (the desktop ID and filenames remain stable for upgrades/associations).
    for size in HICOLOR_SIZES:
        folder = APP / "hicolor" / f"{size}x{size}" / "apps"
        folder.mkdir(parents=True, exist_ok=True)
        resized(icon, size).save(folder / "ai.storyteller.photocraft.png", optimize=True)

    # Scalable entries embed the exact RGBA icon rather than a traced/redrawn approximation.
    make_svg(icon, APP / "photocraft.svg")
    make_svg(icon, APP / "photocraft-small.svg")
    make_svg(icon, APP / "hicolor" / "scalable" / "apps" / "ai.storyteller.photocraft.svg")

    # The old monochrome generator outputs are no longer used; they must not be mistaken for marks.
    for retired in ("brand-mark-128.png", "four-point-star.png", "lunola-master-wordmark.png", "lunola-luma-lockup-dark.png"):
        (OUT / retired).unlink(missing_ok=True)
    print(f"Generated transparent Lunola Luma icons from {SOURCE_ICON.name}; lockup copied byte-for-byte.")
    print(f"Windows ICO sizes: {', '.join(map(str, ICO_SIZES))} px")
    print(f"Linux PNG sizes: {', '.join(map(str, HICOLOR_SIZES))} px")
    print("macOS ICNS sizes: 16, 32, 64, 128, 256, 512, 1024 px")


if __name__ == "__main__":
    main()
