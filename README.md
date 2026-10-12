<p align="center">
  <img alt="Lunola Luma logo lockup" src="brand/lunola-luma/assets/lunola-luma-lockup.png" width="280">
</p>

<h1 align="center">Lunola Luma</h1>

<p align="center">
  <b>Professional Photo Editor</b> — Lunola Luma by Pujo Setyo.<br>
  Built on upstream PhotoCraft; its original Rust editor, tools and document formats are preserved.
</p>

<p align="center">
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="macOS · Windows · Linux · FreeBSD · Web" src="https://img.shields.io/badge/macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20FreeBSD%20%C2%B7%20Web-native-2f7bf5?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-3a3a3a?style=flat-square">
</p>

<p align="center">
  <a href="https://github.com/storytold/photocraft"><b>Upstream source: PhotoCraft by Storytold</b></a> ·
  <a href="NOTICE">Copyright and license notices</a>
</p>

<br>

<p align="center">
  <img src="brand/lunola-luma/evidence/after.png" alt="Lunola Luma start screen with the supplied LUNOLA and LUMA wordmark" width="100%">
  <br>
  <sub>Windows rebrand smoke-test capture. Editing behavior is unchanged from upstream PhotoCraft.</sub>
</p>

> [!NOTE]
> Lunola Luma is a display-identity rebrand of PhotoCraft, originally created by the ArtCraft Team and contributors.
> Upstream author attribution, source history, file formats and license notices are preserved.
> Only Windows was build- and GUI-smoke-tested for this rebrand; other platform builds and packages remain unverified.

<p align="center">
  <a href="#features">Features</a> ·
  <a href="#everything-in-the-box">Everything in the box</a> ·
  <a href="#psd-without-compromise">PSD</a> ·
  <a href="#built-for-agents">Agents</a> ·
  <a href="#under-the-hood">Under the hood</a> ·
  <a href="#get-started">Get started</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#lunola-creative-software">Lunola creative software</a>
</p>

<br>

<table>
  <tr>
    <td width="25%" valign="top">
      <h3>🎛️ Familiar by design</h3>
      The menus, shortcuts, panels and tools are where your hands expect them, from ⌘J to ⇧⌘D. If you know Photoshop, you already know Lunola Luma.
    </td>
    <td width="25%" valign="top">
      <h3>⚡ Native and fast</h3>
      A GPU compositor on wgpu (Metal, Vulkan, DX12, WebGPU), copy-on-write tiles and multithreaded filters. No Electron, no web view, no waiting.
    </td>
    <td width="25%" valign="top">
      <h3>🗂️ Real PSD files</h3>
      Open, edit and save layered Photoshop documents. Re-saving keeps the render of 307 of the 309 psd-tools test files.
    </td>
    <td width="25%" valign="top">
      <h3>🤖 Agent-ready</h3>
      Every action is a command, so you can drive the same engine from the UI, the CLI, a JSON control channel or an MCP server.
    </td>
  </tr>
</table>

<br>

## Features

The feature descriptions below reflect PhotoCraft functionality inherited by Lunola Luma. Archived images in `docs/images/` depict the upstream PhotoCraft interface and are not presented as Lunola Luma screenshots. Numeric feature and corpus figures are upstream documentation, not measurements from this Lunola Luma handoff. Current local verification is in [the smoke-test report](docs/windows-smoke-test.md).

<table>
  <tr>
    <td width="50%" valign="top">
      <h3>Edit without regret</h3>
      Adjustment layers keep every edit live. Stack Levels, Curves, Vibrance, Hue/Saturation and a dozen more, mask them to an area, reorder them, or turn them off, and your original pixels never change.
      <br><br>
      <b>16 adjustment layers</b> that also apply directly to pixels, including Curves with per-channel editing, Levels with a live histogram, Black &amp; White, Channel Mixer, Gradient Map, Photo Filter, Selective Color and Color Lookup (.cube, .3dl, .look). Plus Shadows/Highlights, Replace Color, Match Color, HDR Toning, Desaturate and Equalize.
    </td>
    <td width="50%" valign="top">
      <h3>Styles that sell the shot</h3>
      Drop Shadow, Inner Shadow, Outer and Inner Glow, Bevel &amp; Emboss, Satin, Stroke, and Color, Gradient and Pattern Overlay, live on any layer, including type. Patterns come from a library (built-ins, Edit › Define Pattern, <code>.pat</code> import/export) and PSD <code>Patt</code> blocks.
      <br><br>
      Copy and paste styles between layers, hide all effects at once, and open styles straight from your PSDs, rendered to match Photoshop.
    </td>
  </tr>
  <tr>
    <td width="50%" valign="top">
      <h3>Selections that understand your image</h3>
      Marquees, lassos and the Magic Wand for precision; Quick Selection, Object Selection and Select Subject when you want the computer to do the tracing; Select and Mask to refine hair-fine edges.
      <br><br>
      Feather, expand, contract, smooth, grow, reselect. Turn any selection into a layer mask, a vector path or a shape. Smart selection runs on your machine, with no cloud and no account.
    </td>
    <td width="50%" valign="top">
      <h3>Type that sets beautifully</h3>
      Point and paragraph text, edited right on the canvas, with full Character and Paragraph controls: font, weight, size, leading, tracking, alignment and colour. The <a href="docs/color-picker.md">full Type Color Picker</a> also supports screen sampling with a pixel loupe.
      <br><br>
      Type layers stay editable, take layer styles, and round-trip through PSD.
    </td>
  </tr>
  <tr>
    <td width="50%" valign="top">
      <h3>Pixel-perfect vectors</h3>
      Rectangle, Ellipse, Triangle, Polygon, Line and the Pen, Freeform Pen and Curvature Pen tools, with resolution-independent shape layers, gradient fills, and dashed, aligned strokes.
      <br><br>
      Combine shapes (unite, subtract, intersect, exclude), keep paths in the Paths panel, use them as vector masks, or stroke and fill them. 116 of 116 shape layers in our PSD corpus match Photoshop's pixels.
    </td>
    <td width="50%" valign="top">
      <h3>See it before you commit</h3>
      Every filter dialog previews live on the canvas, through your selection. Blurs (Gaussian, Box, Motion, Radial, Surface, Smart, Lens, Shape, and the Blur Gallery: Tilt-Shift, Iris, Field, Spin, Path), sharpening, Reduce Noise, distortions (Twirl, Wave, Ripple, Displace, Shear, Zig Zag…), Pixelate, Stylize (Oil Paint, Wind, Extrude…), Render (Clouds, Fibers, Lens Flare, Lighting Effects) and more.
      <br><br>
      Run filters on a smart object and they stay editable: change, hide, reorder or mask them at any time.
      <br><br>
      Large-radius blurs use running-sum box passes across all cores: a radius-180 Gaussian on 3.6 MP takes under a second.
    </td>
  </tr>
  <tr>
    <td width="50%" valign="top">
      <h3>Shape it any way you like</h3>
      Free Transform with scale, rotate, skew, distort and perspective; exact 90° and 180° rotations and flips; Transform Again. Layers, type, shapes, masks and selections all transform together.
      <br><br>
      Full history, Toggle Last State and the History Brush mean every step can be undone, even one brush stroke at a time.
    </td>
    <td width="50%" valign="top">
      <h3>Ship it anywhere</h3>
      Export As with format, quality, transparency and scale, plus a preview and an instant file-size estimate. Quick Export to PNG in one click.
      <br><br>
      Choose a dark Pro theme, the airy Studio themes, or a Classic look.
    </td>
  </tr>
</table>

<br>

## Everything in the box

<table>
  <tr>
    <td width="33%" valign="top">
      <h4>🧰 67 tools</h4>
      Move · Artboard · Rectangular and Elliptical Marquee · Single Row Marquee · Single Column Marquee · Lasso · Polygonal Lasso · Magnetic Lasso · Object Selection · Quick Selection · Magic Wand · Crop · Perspective Crop · Slice · Slice Select · Frame · Eyedropper · Color Sampler · Ruler · Note · Count · Remove · Spot Healing Brush · Healing Brush · Patch · Content-Aware Move · Red Eye · Brush · Pencil · Mixer Brush · Color Replacement · Clone Stamp · Pattern Stamp · History Brush · Eraser · Background Eraser · Magic Eraser · Gradient · Paint Bucket · Blur · Sharpen · Smudge · Dodge · Burn · Sponge · Pen · Freeform Pen · Curvature Pen · Add Anchor Point · Delete Anchor Point · Convert Point · Horizontal and Vertical Type · Horizontal and Vertical Type Mask · Path Selection · Direct Selection · Rectangle · Ellipse · Triangle · Polygon · Line · Custom Shape · Hand · Rotate View · Zoom
    </td>
    <td width="33%" valign="top">
      <h4>🖌️ A real brush engine</h4>
      Shape Dynamics, Scattering, Texture, Dual Brush, Color Dynamics, Transfer, Brush Pose, Wet Edges, Build-up and Smoothing (including Pulled String), driven by pen pressure, tilt, rotation and direction. Brush presets, Define Brush from Selection, and deterministic, replayable strokes.
    </td>
    <td width="33%" valign="top">
      <h4>🗃️ Layers, done properly</h4>
      Groups, clipping masks, pixel and vector masks, fill layers (solid, gradient and pattern), adjustment layers, live smart objects with smart filters and lossless transforms and warps, multi-layer selection with align, distribute and link, alpha channels and Quick Mask, 27 Photoshop blend modes plus six Paint.NET variants, opacity and fill, locks, colour labels, layer filters, merge, flatten, rasterize, Layer via Copy/Cut, Paste Into.
    </td>
  </tr>
  <tr>
    <td width="33%" valign="top">
      <h4>🎨 Any colour, any depth</h4>
      RGB, Grayscale, CMYK and Lab documents at 8, 16 and 32 bits per channel. Bit depth and colour model are runtime data, so every tool works at every depth.
      <br><br>
      Real ICC colour management in pure Rust: embedded profiles, Assign and Convert to Profile with all four rendering intents and black point compensation, soft proofing (⌘Y) and Gamut Warning (⇧⌘Y) on the GPU.
    </td>
    <td width="33%" valign="top">
      <h4>🗂️ Formats</h4>
      PSD and PSB, layered TIFF (Photoshop's layer data in the TIFF, read and written in either byte order), <a href="docs/pdn.md">Paint.NET PDN3 (read, with editable layers and all 14 blend modes)</a>, <a href="docs/ora.md">OpenRaster (read and write, with layers, groups and blend modes, for Krita, MyPaint and GIMP)</a>, SVG (opens as shape layers, places as a vector Smart Object), plus flat PNG, JPEG, TIFF, WebP (lossy and lossless), GIF, BMP, TGA, ICO, QOI, PNM, OpenEXR, Radiance HDR and FITS (astronomy images: 8/16-bit and 32-bit float, gray or RGB, header kept), and the native <code>.pcraft</code> format. Flat formats retain the depths they support: PNG keeps 8/16-bit samples, TIFF keeps 8/16-bit and 32-bit float samples, and OpenEXR keeps half/float samples; JPEG, WebP and the other 8-bit formats reduce higher-depth samples with an export warning. See the <a href="crates/codecs/README.md#capability-matrix-default-build">codec capability matrix</a> for each format. HEIC photos from iPhone and Mac are read-only (included in official builds; optional <code>--features heif</code> for source builds). AVIF is write-only with the optional <code>avif</code> build feature; AVIF decoding and JPEG XL are unavailable. Affinity documents (<code>.af</code>, <code>.afdesign</code>, <code>.afphoto</code>, <code>.afpub</code>) open natively, read only, with artboards, layers, vectors, text, images and masks; effects, adjustments and other unsupported parts are listed in a warning, and Affinity export is unavailable.
    </td>
    <td width="33%" valign="top">
      <h4>🪄 The everyday essentials</h4>
      Auto Tone, Contrast and Color · Equalize · Image and Canvas Size · Crop and Trim · Reveal All · Edit › Fill and Stroke · Copy Merged · Paste in Place · guides, rulers, grid and snapping · Actions record and replay · a command palette (⌘F).
    </td>
  </tr>
</table>

<br>

## PSD without compromise

The upstream PhotoCraft project's PSD support is a standalone crate written from Adobe's public specification and tested against a corpus of real-world files. This handoff did not rerun those corpus tests.

- **Faithful round trips:** the upstream project reports matching renders for 307 of 309 psd-tools files and 169 of 170 mixed ag-psd/psd-tools files (`crates/io/tests/corpus.rs`; fetch the psd-tools set with `cargo xtask corpus --psd-tools`). These figures were not rerun for this rebrand. Unsupported raw blocks, descriptors and extras are carried over instead of being dropped. A re-saved file is not byte-identical to its source; the document model rewrites image resources, layer records and the composite. The standalone `photocraft-psd` crate, parsing and writing without the document model, reproduces every parseable corpus file byte for byte (`crates/psd/tests/corpus.rs`).
- **Pixels that match:** a composite oracle compares our render with Photoshop's own merged image, covering gradient interpolation (Classic, Perceptual and Linear), layer effects, shape strokes, clipping and fill opacity.
- **Large documents:** PSB, 16 and 32-bit files, and CMYK and Lab documents open natively.

## Built for agents

Every menu item, tool and dialog runs a command from one registry of 500+ commands. The UI, the CLI, the JSON control channel and the MCP server all call the same commands, so anything you can click, a script or an AI agent can do too.

```sh
# Headless: open, edit, save
photocraft-cli run wave.psd \
  --cmd filter.sharpen.smartSharpen     --params '{"amount":80}' \
  --cmd layer.newAdjustmentLayer.curves --params '{"points":[[0,0],[64,48],[192,212],[255,255]]}' \
  --out wave-final.png

# Apply one action list to a folder of images
photocraft-cli batch --actions grade.json --in ./raw --out ./graded

# Every subcommand explains itself
photocraft-cli batch --help

# Let an agent drive it over MCP (headless, or bridged to the running app)
photocraft-cli mcp
```

The desktop app also offers an authenticated, loopback-only control channel (`photocraft --control`) for inspecting UI state, driving tools with pointer events, and taking offscreen screenshots. Every image in this README was rendered that way. See [`docs/control-protocol.md`](docs/control-protocol.md).

## Under the hood

- **Engine first:** a pure-data document model and a command engine, with a thin egui UI on top. Layering is enforced at build time across 24 crates.
- **Two compositors:** a CPU compositor serves as the reference oracle, and a wgpu compositor puts the canvas on the GPU. They are tested against each other.
- **Copy-on-write tiles:** 256² sparse tiles make undo cheap and huge canvases light, and effect maps are cached per layer state.
- **Runs in the browser:** the whole engine and UI compile to WebAssembly.
- **Clean-room:** implemented from public specs and observed behaviour only. No proprietary code, shaders or assets.
- **Tested:** more than 1,700 tests, including PSD round trips, synthetic generators, compositor oracles and multi-depth checks.

## Get started

```sh
git clone https://github.com/storytold/photocraft
cd photocraft
cargo run --release -p photocraft -- image.psd   # the desktop app
cargo test --workspace                           # the test suite
```

Japanese fonts for the UI and Type tool come from [craft-fonts](https://github.com/storytold/craft-fonts), an optional build input (desktop release builds always include it). Without it Lunola Luma uses your system's CJK fonts:

```sh
git clone https://github.com/storytold/craft-fonts ../craft-fonts
CRAFT_FONTS_DIR="$PWD/../craft-fonts" cargo run --release -p photocraft
```

New contributors and AI agents: start with [`AGENTS.md`](AGENTS.md), then [`docs/`](docs/).

Installers for macOS, Windows, Linux, FreeBSD and the web are attached to each [GitHub release](https://github.com/storytold/photocraft/releases). On Linux you can pick an AppImage, a `.deb`, an `.rpm`, a tarball or a Flatpak bundle. The bundle needs the freedesktop runtime from [Flathub](https://flathub.org/setup), which `flatpak` offers to install along with it:

```sh
flatpak install --user photocraft-<version>-linux-x86_64.flatpak   # or -linux-aarch64
flatpak run ai.storyteller.photocraft
```

The AppImage needs no install: the first run registers its launcher icon and menu entry in `~/.local/share` so the dock shows Lunola Luma's icon on Wayland. Set `PHOTOCRAFT_NO_DESKTOP_INTEGRATION=1` to skip that, and see [`docs/releasing.md`](docs/releasing.md) › Linux to undo it.

On a Wayland session, files dropped on the window don't open yet: winit 0.30, the windowing library under egui, has no Wayland drag and drop (#386). Use File › Open, or copy the image in your file manager and paste it with Ctrl+V. To drag and drop, start Lunola Luma under XWayland: `WAYLAND_DISPLAY= photocraft`, `WAYLAND_DISPLAY= ./photocraft-<version>-linux-x86_64.AppImage`, or `flatpak run --nosocket=wayland --socket=x11 ai.storyteller.photocraft`. On Wayland, Lunola Luma shows the command for your install in a notice until you dismiss it.

On macOS, the command-line tool comes as `photocraft-cli-<version>-macos-universal.zip`. The binary is signed with the same Developer ID as the app and notarized by Apple. A bare binary can't carry a stapled notarization ticket the way the DMG does, so the first time you run it macOS checks the notarization online. You can confirm it yourself:

```sh
ditto -x -k photocraft-cli-<version>-macos-universal.zip .
spctl --assess --type install -vv photocraft-cli-<version>-macos-universal/photocraft-cli
# ... accepted, source=Notarized Developer ID
```

On FreeBSD 14 (x86_64), the release has a tarball laid out like `/usr/local`. Install the runtime libraries, then unpack it there:

```sh
pkg install libxkbcommon wayland libX11 libXcursor libXrandr libXi libxcb mesa-libs vulkan-loader gtk3 fontconfig freetype2 alsa-lib
tar -xzf photocraft-<version>-freebsd-x86_64.tar.gz --strip-components 1 -C /usr/local
photocraft
```

Maintainers: [`docs/releasing.md`](docs/releasing.md) explains how releases are built, signed and published.

> [!IMPORTANT]
> **Upstream status:** the original PhotoCraft project is in early alpha, and we want to be straight about where it stands: much of Photoshop's feature surface exists in some form, but **it is not yet a Photoshop replacement for daily professional work**. The biggest gaps are AI/generative features, 15 missing tools, depth in typography and pro workflows, and plug-in compatibility. Every Photoshop menu item is wired to a command ([`docs/parity-checklist.md`](docs/parity-checklist.md)), but that measures wiring, not behaviour. The honest, dimension-by-dimension picture and where we're going next are in the [roadmap](ROADMAP.md) and [parity assessment](docs/target-app-parity.md). Expect rough edges, and please file issues (include your OS, document size, layer count and a screenshot).

## Documentation

Start with the [documentation index](docs/README.md) or the [getting-started guide](docs/getting-started.md). The [Lunola Luma documentation book](book/) contains the deeper architecture, development, automation, format and security references. The [brand record](brand/lunola-luma/README.md), [Windows smoke-test report](docs/windows-smoke-test.md), and [`releases/`](releases/README.md) describe the rebrand evidence and current release status.

To self-host the browser version with Docker, see [Docker web hosting](packaging/web/README.md#docker).

## Security

Security architecture, threat modeling, parser hardening, fuzzing, and vulnerability reporting are covered in the [security documentation](book/src/security/) and the repository [security policy](SECURITY.md).

## Upstream test corpora

The upstream PhotoCraft project tests against real files: Photoshop-authored oracle PSDs in
[photocraft-corpus](https://github.com/storytold/photocraft-corpus) plus the psd-tools, ag-psd and PngSuite sets, pinned and
sha256-verified. This rebrand preserves the corpus harness but did not fetch or run these sets. Fetch them with `cargo xtask corpus --all` and run the tests with
`cargo xtask test-corpus` (details in [docs/development.md](docs/development.md#test-corpora)).

## Downloads

This rebrand branch does not publish installers yet. The links below point to **upstream PhotoCraft releases**, which retain the original PhotoCraft branding and are not Lunola Luma builds. The original artifact filenames are unchanged to preserve packaging compatibility.

**Upstream releases:** [latest](https://github.com/storytold/photocraft/releases/latest) · [all releases](https://github.com/storytold/photocraft/releases). `<ver>` is the upstream version; `SHA256SUMS.txt` lists checksums.

### Upstream Windows artifacts

| Build | Installer | Portable |
|---|---|---|
| x64 (64-bit Intel/AMD) | `photocraft-<ver>-windows-x64.msi` | `photocraft-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon and other ARM PCs) | `photocraft-<ver>-windows-arm64.msi` | `photocraft-<ver>-windows-arm64-portable.zip` |
| x86 (32-bit) | `photocraft-<ver>-windows-x86.msi` | `photocraft-<ver>-windows-x86-portable.zip` |

Installers and executables are code-signed.

### Upstream macOS artifacts

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `photocraft-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `photocraft-cli-<ver>-macos-universal.zip` | Signed and notarized |

### Upstream Linux artifacts

| Format | x86_64 | aarch64 (ARM64) | Notes |
|---|---|---|---|
| AppImage | `photocraft-<ver>-linux-x86_64.AppImage` | `photocraft-<ver>-linux-aarch64.AppImage` | Runs anywhere; updates itself with [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`.zsync` files) |
| Flatpak | `photocraft-<ver>-linux-x86_64.flatpak` | `photocraft-<ver>-linux-aarch64.flatpak` | Sandboxed; `flatpak install --user <file>` |
| Debian/Ubuntu | `photocraft-<ver>-linux-x86_64.deb` | `photocraft-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `photocraft-<ver>-linux-x86_64.rpm` | `photocraft-<ver>-linux-aarch64.rpm` | |
| Tarball | `photocraft-<ver>-linux-x86_64.tar.gz` | `photocraft-<ver>-linux-aarch64.tar.gz` | Unpack anywhere |

RISC-V (riscv64): `photocraft-<ver>-linux-riscv64.tar.gz` only, built against Ubuntu 24.04 (needs glibc 2.39+).

### Upstream FreeBSD artifacts

| Build | File |
|---|---|
| x86_64 | `photocraft-<ver>-freebsd-x86_64.tar.gz` |

### Upstream Web artifacts (WebAssembly)

| Build | File | Notes |
|---|---|---|
| Static site | `photocraft-web-<ver>.zip` | Runs in a modern browser; host it on any static server |

## Rebrand and attribution

Lunola Luma is a display-identity rebrand by Pujo Setyo based on upstream PhotoCraft. The original Rust editor, contributor history, document formats, licenses and legal notices are preserved.

---

## License and credits

The upstream source code remains dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
The Lunola Luma marks are project-owner supplied and used under separate permission; see [their license notice](brand/lunola-luma/LICENSE-brand.txt).
Copyright (c) 2026 ArtCraft Team and the PhotoCraft contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

Every artwork shown is in the public domain (Wikimedia Commons, NASA, U.S. National Archives); sources are listed in [`docs/images/SOURCES.md`](docs/images/SOURCES.md).

The original ArtCraft wordmark and logo assets are trademarks of the ArtCraft Team and are not bundled in this rebrand. Their trademark notice is preserved in [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt); Lunola Luma uses only the owner-supplied marks.

<sub>Adobe, Photoshop, Illustrator, Premiere Pro, Lightroom, Acrobat, After Effects and InDesign are trademarks or registered trademarks of Adobe Inc. in the United States and/or other countries. Lunola Luma is an independent, open-source rebrand based on PhotoCraft and is not affiliated with, sponsored by or endorsed by Adobe Inc.; these names are used only to describe the workflows it is compatible with.</sub>

<p align="center">
  Based on <a href="https://github.com/storytold/photocraft">PhotoCraft by Storytold</a>; original contributors retain their copyright and attribution.
</p>

## Upstream PhotoCraft star history

[![Upstream PhotoCraft Star History Chart](https://api.star-history.com/svg?repos=storytold/photocraft&type=Date&legend=top-left)](https://www.star-history.com/?repos=storytold%2Fphotocraft&type=date&legend=top-left)
