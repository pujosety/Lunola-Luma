<p align="center">
  <img alt="Lunola Luma — by Pujo Setyo" src="brand/lunola-luma/assets/lunola-luma-lockup.png" width="320">
</p>

<h1 align="center">Lunola Luma</h1>

<p align="center">
  <strong>Make the image. Keep every layer.</strong><br>
  A professional photo editor for thoughtful, detailed image work.<br>
  <em>By Pujo Setyo</em>
</p>

<p align="center">
  <a href="https://github.com/pujosety/Lunola-Luma/releases/tag/v0.8.2"><strong>⬇ Get Lunola Luma v0.8.2 Preview</strong></a>
  &nbsp;·&nbsp;
  <a href="docs/getting-started.md">Getting started</a>
  &nbsp;·&nbsp;
  <a href="docs/">Documentation</a>
  &nbsp;·&nbsp;
  <a href="ROADMAP.md">Roadmap</a>
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/native-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="Prerelease version 0.8.2" src="https://img.shields.io/badge/prerelease-0.8.2-e8b44c?style=flat-square">
  <img alt="License: MIT or Apache 2.0" src="https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-465b73?style=flat-square">
</p>

<p align="center">
  <img src="brand/lunola-luma/evidence/after.png" alt="Lunola Luma running on Windows" width="100%">
</p>

> **Download:** Lunola Luma v0.8.2 is a published prerelease. It includes unsigned Windows x64 packages, Linux x86_64/aarch64 and Flatpak builds; the release notes list omitted platforms and verification failures.

## An editor built around the way you work

Lunola Luma brings a native, GPU-accelerated canvas together with the precision tools of a layer-based editor. Make a change, refine it, revisit it later—without flattening your ideas along the way.

| ✦ Shape the image | ✦ Keep edits flexible | ✦ Finish with control |
|---|---|---|
| Brushes, selections, paths, vector shapes and editable type. | Groups, masks, adjustment layers, smart objects and live filters. | Color-managed documents, detailed export controls and familiar keyboard-driven workflows. |

## A closer look

- **Layered editing:** groups, clipping masks, pixel and vector masks, fill and adjustment layers, smart objects, blend modes and layer styles.
- **A responsive canvas:** native Rust interface with a `wgpu` renderer and GPU-assisted composition.
- **Tools for fine work:** a pressure-aware brush engine, selection and mask tools, vector paths, shape layers and on-canvas typography.
- **Color with intention:** RGB, grayscale, CMYK and Lab documents at 8-, 16- and 32-bit depths, with ICC profiles, soft proofing and gamut warnings.
- **Repeatable workflows:** batch operations and a scriptable command engine for routine edits.

## Everything in the box

The current toolbar includes all 67 selectable tools:

<h4>🧰 67 tools</h4>
Move · Artboard · Rectangular and Elliptical Marquee · Single Row Marquee · Single Column Marquee · Lasso · Polygonal Lasso · Magnetic Lasso · Magic Wand · Crop · Perspective Crop · Eyedropper · Color Sampler · Ruler · Note · Count · Brush · Pencil · Mixer Brush · Color Replacement · Eraser · Background Eraser · Magic Eraser · Gradient · Paint Bucket · Horizontal Type · Vertical Type · Horizontal Type Mask · Vertical Type Mask · Hand · Rotate View · Zoom · Remove · Spot Healing Brush · Healing Brush · Patch · Content-Aware Move · Red Eye · Clone Stamp · Pattern Stamp · History Brush · Blur · Sharpen · Smudge · Dodge · Burn · Sponge · Quick Selection · Object Selection · Pen · Freeform Pen · Curvature Pen · Add Anchor Point · Delete Anchor Point · Convert Point · Path Selection · Direct Selection · Rectangle · Ellipse · Triangle · Polygon · Line · Custom Shape · Slice · Slice Select · Frame

## Open the files you already have

**Layered documents:** PSD and PSB, layered TIFF, OpenRaster, Krita `.kra`/`.krz` (read-only import), and Lunola Luma's native `.pcraft` format.

**Images:** PNG, JPEG, TIFF, WebP, GIF, BMP, TGA, ICO, QOI, PNM, OpenEXR and Radiance HDR. FITS is supported for astronomy images. HEIF/HEIC and several additional formats have read-only or build-dependent support.

Read/write details and limitations are listed in the [format capability matrix](crates/codecs/README.md#capability-matrix-default-build).

## Get the preview

[**View Lunola Luma v0.8.2 Preview →**](https://github.com/pujosety/Lunola-Luma/releases/tag/v0.8.2)

Version 0.8.2 is a published GitHub prerelease with 15 uploaded assets: 14 package files and `SHA256SUMS.txt`. It includes Windows x64 MSI and portable ZIP (both unsigned), Linux x86_64/aarch64 packages, and Flatpak x86_64/aarch64. macOS, Linux riscv64, WebAssembly, Windows x86/ARM64, and FreeBSD packages are not included. The 14 package checksums were verified against GitHub's reported SHA-256 digests.

This is a partial preview, not a stable release. CI reported failures in corpus tests, the wasm32 check, web build/size gate, and packaging lint. The local workspace test run stopped at the CLI tests: 27 passed, one failed (`usage_and_unknown_command`, expected the output to begin with `photocraft-cli `). Review the [release notes](releases/0.8.2.md) and checksums before installing. [Lunola Luma v0.7.0](https://github.com/pujosety/Lunola-Luma/releases/tag/v0.7.0) remains available.

## Build from source

1. Install the Rust toolchain and the platform prerequisites in the [build guide](docs/getting-started.md).
2. Clone this repository and follow the tested setup steps in that guide.
3. Use the [development reference](docs/development.md) for tests, fonts, environment settings and troubleshooting.

The desktop, command-line and WebAssembly targets share the same editing engine. Their internal package names are retained for compatibility; product-facing text and artwork use **Lunola Luma**.

## Project status

Lunola Luma is actively developed and remains an early-stage editor—not a drop-in replacement for every professional workflow. Some tools, typography workflows and integrations are still in progress. The [roadmap](ROADMAP.md), [feature assessment](docs/target-app-parity.md) and [known gaps](docs/gaps.md) distinguish complete, partial and planned work.

## Project links

- [Documentation index](docs/README.md)
- [Contribution guide](docs/contributing.md)
- [Security policy](SECURITY.md)
- [Release notes and package status](releases/README.md)
- [Web preview hosting](packaging/web/README.md)

## Credits and licenses

Lunola Luma is maintained by **Pujo Setyo**. Source-code licenses, required notices and third-party asset credits are preserved in [NOTICE](NOTICE), [ATTRIBUTION.md](ATTRIBUTION.md), and the individual license files. Lunola Luma marks are supplied by the project owner under a separate mark license: [brand license](brand/lunola-luma/LICENSE-brand.txt).

The software is independent and is not affiliated with, sponsored by, or endorsed by Adobe Inc. Product and file-format names are used only to describe compatible workflows. See [legal notices](NOTICE) for full credits.
