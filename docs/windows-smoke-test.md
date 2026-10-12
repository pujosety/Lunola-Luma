# Windows smoke-test record

This record describes a real local build and GUI smoke test, not a signed installer or a published release.

## Verified

- **Toolchain:** Windows 10; `cargo 1.96.1`, `rustc 1.96.1`.
- **Formatting:** `cargo fmt --all --check` passed.
- **Build:** `CARGO_INCREMENTAL=0 cargo build -j 1 -p photocraft -p photocraft-cli` passed after the final visible-brand wording changes.
- **Windows icon references:** `packaging/windows/check-icons.ps1` passed. The freshly generated app and CLI resource scripts contain Lunola Luma product names, LUNOLA company metadata, and PhotoCraft-compatible executable filenames.
- **Launch and menu:** launched the fresh desktop build with `PHOTOCRAFT_CPU_CANVAS=1`; the window title and supplied LUNOLA/LUMA lockup rendered. The top-level menu bar contains File, Edit, Image, Layer, Type, Select, Filter, View and Window—no Help. The File menu contains **About Lunola Luma**; its dialog shows the **By Pujo Setyo** byline and no Contributors or Models tabs.
- **About:** the fresh dialog shows the product description, **By Pujo Setyo**, version `0.7.0 (dev build)`, and `egui · wgpu · Lunola Luma Engine`; Contributors and Models tabs are absent.
- **Version/help text:** `photocraft.exe --version` prints `Lunola Luma 0.7.0 (dev build)`; `photocraft-cli.exe --version` prints `Lunola Luma CLI 0.7.0 (dev build)`. CLI help is branded Lunola Luma; the legacy `photocraft-cli` executable command remains for compatibility.
- **Open/save/export:** opened the synthetic 96×64 PNG fixture in the desktop app and saved it. Then the Windows CLI converted PNG → `.pcraft` → WebP; `photocraft-cli info` reopened both outputs and reported 96×64, 8-bit RGB, with no warnings.
- **Wayland guidance tests:** all 4 targeted `wayland_guidance_` tests passed after replacing the old launch-command text with translated X11 preference guidance.
- **Metadata syntax:** Python XML parsing passed for the WiX source, Linux AppStream template, and MIME metadata. `git diff --check` passed after the final changes.

## Test-suite result and baseline

`CARGO_INCREMENTAL=0 cargo test -p photocraft-ui-egui --lib --quiet` ran 2,195 tests: **2,184 passed, 1 failed, 10 ignored**. The remaining failure is `i18n::tests::bundled_catalogs_are_consistent` because `cs.tsv` contains duplicate `"Tool"` entries. This same Czech duplicate-key failure was present on the recorded upstream baseline (`c20b81b25168057e9cbffa2f45c36ec02dacf756`), where the suite had 2,184 passes, 2 failures, and 10 ignored. The separate baseline jobs-status failure no longer reproduces. A preceding test-link attempt after an interrupted command failed with MSVC LNK1120 unresolved symbols; disabling incremental compilation allowed the targeted byline test and full UI suite to build and run.

## Not verified

- WiX/MSI creation, MSI installation, code signing, and publisher display were not run: `dotnet` and `wix` were not available on PATH. The XML and shortcut-icon-reference checks are not substitutes for building or installing an MSI.
- macOS, Linux, FreeBSD, and web builds were not run in this Windows smoke test.
- No release artifacts or Lunola Luma release tag were created.

## Screenshots and fixture

- [Fresh app start screen](../brand/lunola-luma/evidence/after.png)
- [File menu with About and no top-level Help](../brand/lunola-luma/evidence/help-menu-removed.png)
- [About Lunola Luma dialog — By Pujo Setyo](../brand/lunola-luma/evidence/about.png)
- [Opened/saved synthetic image](../brand/lunola-luma/evidence/smoke-open-save.png)
- [Before rebrand](../brand/lunola-luma/evidence/before.png)
- Reproducible input and CLI outputs are in `brand/lunola-luma/evidence/`: `smoke-input.png`, `smoke-native.pcraft`, and `smoke-export.webp`.
