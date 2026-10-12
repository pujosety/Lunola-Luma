# Lunola Luma app icons

The platform icons are rendered from the official user-supplied `brand/lunola-luma/references/official-lunola-luma-app-icon.png`. The source's RGB colors, rounded-square shape, transparent canvas and safe margins are retained; only isolated alpha≤8 specks outside a one-pixel silhouette halo are cleared. Resizing into required OS formats is the only other transformation. The official product lockup is copied byte-for-byte to `brand/lunola-luma/assets/lunola-luma-lockup.png`.

The `photocraft*` filenames, Linux stock icon ID and macOS resource name intentionally remain unchanged because package manifests, MIME associations and runtime code refer to them. Their artwork is Lunola Luma. Stable machine names do not change the user-visible product name.

## Generated files

- `photocraft.svg`, `photocraft-small.svg`: SVG wrappers embedding the supplied transparent app icon; used by macOS artwork and the Linux scalable entry.
- `lunola-luma.ico`: official product-named Windows ICO; `photocraft.ico` is a byte-identical compatibility path embedded by the existing build.
- `photocraft-1024.png` / `lunola-luma-1024.png`: 1024 px macOS icon source.
- `lunola-luma.icns`: product-named macOS icon; `photocraft.icns` is a byte-identical compatibility resource used by the current bundle script.
- `photocraft.ico`: Windows icon with 16, 20, 24, 32, 40, 48, 64, 128 and 256 px entries.
- `hicolor/<size>/apps/ai.storyteller.photocraft.png`: PNG theme icons at 16, 24, 32, 48, 64, 128, 256 and 512 px.
- `hicolor/scalable/apps/ai.storyteller.photocraft.svg`: scalable Linux theme icon.

Runtime window icon handling remains in `apps/photocraft/src/app_icon.rs`; the MSI uses the same ICO. The macOS `CFBundleIconFile` remains `PhotoCraft.icns` to match the stable resource filename.

## Regenerate

Run `packaging/icons.sh` or `python brand/lunola-luma/generate_assets.py` from any directory. Both use the same deterministic generator and require Python 3 with Pillow.

The official Lunola Luma app icon and product lockup are project-owner supplied for this rebrand and its associated builds; they are not covered by the upstream source-code license. See [`brand/lunola-luma/LICENSE-brand.txt`](../../brand/lunola-luma/LICENSE-brand.txt). The replaced upstream PhotoCraft icon's original license notice is retained at `brand/lunola-luma/references/photocraft-icon-LICENSE.txt` for attribution history.
