# Lunola Luma — Phase 1 rebrand

Branding and asset record for **Lunola Luma by Pujo Setyo**. The editor's image-processing engine, document formats, command IDs, internal crate/binary names, file associations and source history remain unchanged; this branch changes the visible identity only.

## Identity

- Parent brand: LUNOLA.
- Product: LUMA.
- Display name: Lunola Luma.
- Positioning: Professional Photo Editor.
- About byline: **By Pujo Setyo**. Original contributor credits and required legal attribution remain preserved.
- Official user-supplied sources: [`references/official-lunola-luma-app-icon.png`](references/official-lunola-luma-app-icon.png) and [`references/official-lunola-luma-lockup.png`](references/official-lunola-luma-lockup.png).
- `assets/lunola-luma-lockup.png` is copied byte-for-byte from the supplied lockup. The icon generator does not redraw or recolour the mark; it preserves the transparent square canvas and colors, clears only isolated alpha≤8 specks outside a one-pixel silhouette halo, and resamples the original RGBA art into platform formats.
- Regenerate platform icons with `python brand/lunola-luma/generate_assets.py` (requires Pillow).

## Source and attribution

Project repository: <https://github.com/pujosety/Lunola-Luma>

Upstream base commit: `c20b81b25168057e9cbffa2f45c36ec02dacf756`.

The source code remains under its existing MIT OR Apache-2.0 terms. Copyright and third-party notices remain with their original authors in `NOTICE`, `ATTRIBUTION.md`, `LICENSE-MIT` and `LICENSE-APACHE`. Lunola Luma marks are project-owner supplied for this rebrand and its associated builds; see [`LICENSE-brand.txt`](LICENSE-brand.txt).

## Evidence

- [`evidence/before.png`](evidence/before.png): original app on this Windows workstation before the rebrand.
- [`evidence/after.png`](evidence/after.png): fresh Windows build landing screen after Help-menu removal; product title and supplied lockup are visible.
- [`evidence/help-menu-removed.png`](evidence/help-menu-removed.png): File menu open, showing About and no top-level Help entry.
- [`evidence/about.png`](evidence/about.png): About Lunola Luma dialog with **By Pujo Setyo** byline.
- [`evidence/smoke-open-save.png`](evidence/smoke-open-save.png): synthetic fixture open in the app after save; the status-bar path is cropped from this public evidence copy.
- [`evidence/dmg-background-current.png`](evidence/dmg-background-current.png): current generated macOS DMG background preview from the supplied icon and lockup (not a Finder window capture).
- `evidence/smoke-input.png`, `smoke-native.pcraft`, and `smoke-export.webp`: input and outputs from the Windows open/save/export smoke test. The CLI reopened both outputs and reported 96×64, 8-bit RGB with no warnings.

## Current verification

Current Windows build, test, menu, About, and format-conversion results are recorded in [`../../docs/windows-smoke-test.md`](../../docs/windows-smoke-test.md). The full UI test suite retains one known Czech duplicate-key failure also observed on the recorded baseline. The MSI was not built or signed.

## Baseline (before changes)

- Source baseline commit: `c20b81b25168057e9cbffa2f45c36ec02dacf756`.
- `cargo build -p photocraft`: passed on Windows using the installed MSVC toolchain.
- `cargo xtask layers`: passed (29 crates, no layer violations).
- `cargo test -p photocraft-ui-egui --lib`: 2,184 passed, 2 failed, 10 ignored. Existing failures: `i18n::tests::bundled_catalogs_are_consistent` (`cs`: duplicate `"Tool"`) and `jobs_ui::tests::the_frame_loop_keeps_running_during_a_slow_job_and_esc_cancels_it` (expected job status message). These are baseline failures, not caused by the rebrand.
- Native UI opened with CPU canvas. This workstation displayed a GPU-fallback notice when GPU acceleration was attempted; no document data was modified.

## Unchanged functionality

Lunola Luma's image engine, layer model, PSD/PSB and `.pcraft` serialization, open/save/export commands, keyboard shortcuts, editing tools, file extensions, URI/MIME/Bundle IDs, binary names and command APIs are intentionally unchanged.
