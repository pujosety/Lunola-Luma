# Lunola Luma rebrand change log

**Scope:** display identity and release preparation for Lunola Luma by Pujo Setyo, based on source commit `c20b81b25168057e9cbffa2f45c36ec02dacf756`. This is an implementation record, not a published release changelog.

## Identity and assets

- Set the visible product name to **Lunola Luma**, parent brand to **LUNOLA**, product to **LUMA**, and category to **Professional Photo Editor**.
- Used the supplied app icon and lockup as the authoritative references. Platform icon files and the macOS DMG art are generated from those references; no new logo artwork was drawn.
- Updated desktop/window/About branding and app-store/package descriptions. Windows resources display ProductName `Lunola Luma`, CompanyName `LUNOLA`, while retaining the original executable filenames and upstream copyright line.
- No separate launch splash was added: the current start screen already displays the approved lockup, and a transient duplicate would delay the editor without adding approved artwork.
- Removed old UI captures from the Lunola Luma README feature gallery. The archival images remain under `docs/images/` with attribution and are explicitly identified as historical references, not current Lunola Luma screenshots.
- Kept upstream author credits, source-code licenses, third-party notices, and the historical ArtCraft trademark restrictions. Removed ArtCraft artwork from the rebrand while retaining its legal notice.

## Menus and community controls

- Removed the Help top-level menu from the custom menu bar and macOS native menu setup.
- Moved About Lunola Luma to File. System Info remains searchable.
- About displays **By Pujo Setyo** as the Lunola Luma project byline; the Contributors and Models tabs are removed. Original contributor data, author credits, and legal notices remain intact.
- Replaced visible product wording in About's engine attribution, GUI/CLI version output, startup/error labels, and CLI help with Lunola Luma. Compatibility-oriented executable names, settings/log identifiers, and upstream/legal attribution remain unchanged.
- Removed Discord/community UI links and controls without removing required plain-text attribution to the original source and its authors.

## Distribution and compatibility

- Updated macOS bundle/DMG display assets, Linux desktop/AppStream/package descriptions, and Windows executable/installer display metadata.
- Preserved compatibility identifiers: Cargo packages, executable/bundle/desktop IDs, registry upgrade keys, MIME types, document formats and signatures, extensions, environment variables, portable-data folders, settings paths, and command IDs.
- **Release-history update:** The 0.7.0 candidate described in this rebrand record was later published (see [`releases/0.7.0.md`](../releases/0.7.0.md)). v0.8.2 and v0.8.3 also have published notes; the current release is v0.8.3 partial preview with 24 assets (23 packages plus `SHA256SUMS.txt`), no macOS build, and unsigned Windows packages. See [`releases/README.md`](../releases/README.md) for current status and caveats.

## Initial verification snapshot for the 0.7.0 candidate

- Windows `cargo fmt --all --check` passed. The optimized command `PHOTOCRAFT_REQUIRE_WINRES=1 CARGO_INCREMENTAL=0 cargo build --release --locked -j 1 -p photocraft -p photocraft-cli --features heif` completed successfully; GUI and CLI report `0.7.0 (dev build)` and are not included in the release assets.
- Full UI library tests: 2,184 passed, 1 failed, 10 ignored. The sole failure is the Czech duplicate `"Tool"` catalog key, also present in the recorded baseline; details are in [`windows-smoke-test.md`](windows-smoke-test.md).
- The four targeted Wayland-guidance tests passed; the live About dialog shows the byline, the Lunola Luma Engine attribution, and no Contributors or Models tabs.
- Fresh Windows GUI run confirmed no Help menu, About under File, and opening/saving a synthetic PNG. CLI conversion PNG → `.pcraft` → WebP was reopened and inspected successfully.
- The Windows x86/x64/ARM64 CI jobs produced MSI and portable packages, but all tested MSI and portable EXE signatures are **NotSigned**. macOS universal, Linux riscv64 and WebAssembly jobs failed and are not attached. The draft notes these limitations; do not publish until signing and the remaining platform gaps are resolved or explicitly accepted by the owner.
